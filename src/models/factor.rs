use std::{
    cmp::Ordering::Less,
    fmt::{Debug, Display, Formatter},
    iter::{FusedIterator, Map},
    ops::{Add, Div, Mul},
};

use indexmap::map::Keys;
use itertools::Itertools;
use ndarray::prelude::*;
use prettytable::Table;
use serde::{Deserialize, Serialize};

use crate::{
    types::{FxIndexMap, FxIndexSet},
    utils::nan_to_zero,
};

/// Factor trait.
pub trait Factor:
    Clone + Debug + Display + Add + Mul + Div + Serialize + for<'a> Deserialize<'a>
{
    /// Labels iterator associated type.
    type LabelsIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Value type of the variables.
    type Value<'a>;

    /// Get the set of variables labels.
    fn labels(&self) -> Self::LabelsIter<'_>;

    /// Get reference to underlying values.
    fn values(&self) -> &ArrayD<f64>;

    /// Compute the factor normalization.
    fn normalize(self) -> Self;

    /// Compute the factor marginalization.
    fn marginalize<'a, I>(self, x: I) -> Self
    where
        I: IntoIterator<Item = &'a str>;

    /// Compute the factor reduction.
    fn reduce<'a, I>(self, x: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Self::Value<'a>)>;
}

/// Discrete factor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteFactor {
    levels: FxIndexMap<String, FxIndexSet<String>>,
    values: ArrayD<f64>,
}

impl DiscreteFactor {
    /// Construct a new discrete factor given its values and levels.
    pub fn new<D, I, J, K>(levels: I, values: Array<f64, D>) -> Self
    where
        D: Dimension,
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = K>,
        K: Into<String>,
    {
        // Collect levels.
        let levels: FxIndexMap<String, FxIndexSet<String>> = levels
            .into_iter()
            .map(|(x, ys)| {
                (
                    x.into(),
                    ys.into_iter().map(|y| y.into()).sorted().collect(),
                )
            })
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .collect();
        // Compute factor values shape.
        let shape: Vec<_> = levels
            .keys()
            .map(|x| levels.get(x).unwrap())
            .map(|x| x.len())
            .collect();
        // Assert levels are not empty.
        assert!(shape.iter().all(|&x| x > 0), "Levels must be non empty");
        // Cast array to dynamic shape.
        let values = values
            .into_shape(shape)
            .expect("Values and levels must have same shape")
            .into_dyn();

        Self { levels, values }
    }

    /// Get the set of variables levels.
    #[inline]
    pub const fn levels(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.levels
    }
}

impl Display for DiscreteFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Create print table.
        let mut table = Table::new();
        // Add header to table.
        table.add_row(self.levels.keys().chain([&"Values".to_string()]).collect());
        // Construct iterator over levels cartesian product.
        let levels = self.levels.values().multi_cartesian_product();
        // Add rows to table.
        for (i, x) in levels.zip(self.values.iter()) {
            table.add_row(i.into_iter().chain([&x.to_string()]).collect());
        }
        // Write table to formatter.
        write!(f, "{table}")
    }
}

impl Add for DiscreteFactor {
    type Output = Self;

    fn add(self, phi: Self) -> Self::Output {
        // Compute scope of factor sum.
        let levels: FxIndexMap<_, _> = iter_set::union_by(
            self.levels.clone().into_iter(),
            phi.levels.clone().into_iter(),
            |(x, _), (y, _)| x.cmp(&y),
        )
        .collect();
        // Compute broadcasting shapes.
        let lhs: Vec<_> = levels
            .keys()
            .map(|x| self.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        let rhs: Vec<_> = levels
            .keys()
            .map(|x| phi.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        // Apply broadcasting shapes.
        let lhs = self
            .values
            .into_shape(lhs)
            .expect("Failed to broadcast LHS factor values to given shape");
        let rhs = phi
            .values
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor values to given shape");
        // Compute factor sum.
        let values = (lhs + rhs).into_dyn();

        Self { levels, values }
    }
}

impl Mul for DiscreteFactor {
    type Output = Self;

    fn mul(self, phi: Self) -> Self::Output {
        // Compute scope of factor product.
        let levels: FxIndexMap<_, _> = iter_set::union_by(
            self.levels.clone().into_iter(),
            phi.levels.clone().into_iter(),
            |(x, _), (y, _)| x.cmp(&y),
        )
        .collect();
        // Compute broadcasting shapes.
        let lhs: Vec<_> = levels
            .keys()
            .map(|x| self.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        let rhs: Vec<_> = levels
            .keys()
            .map(|x| phi.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        // Apply broadcasting shapes.
        let lhs = self
            .values
            .into_shape(lhs)
            .expect("Failed to broadcast LHS factor values to given shape");
        let rhs = phi
            .values
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor values to given shape");
        // Compute factor product.
        let values = (lhs * rhs).into_dyn();

        Self { levels, values }
    }
}

impl Div for DiscreteFactor {
    type Output = Self;

    fn div(self, phi: Self) -> Self::Output {
        // Compute scope and levels of factor division.
        let levels = self.levels;
        // Assert RHS scope is subset of LHS scope.
        assert_eq!(iter_set::cmp(phi.levels.keys(), levels.keys()), Some(Less));
        // Compute broadcasting shapes.
        let rhs: Vec<_> = levels
            .keys()
            .map(|x| phi.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        // Apply broadcasting shapes.
        let rhs = phi
            .values
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor values to given shape");
        // Compute factor division.
        let values = (self.values / rhs)
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            .into_dyn();

        Self { levels, values }
    }
}

impl Factor for DiscreteFactor {
    type LabelsIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.levels.keys().map(|x| x.as_str())
    }

    #[inline]
    fn values(&self) -> &ArrayD<f64> {
        &self.values
    }

    #[inline]
    fn normalize(mut self) -> Self {
        // Normalize values.
        self.values /= self.values.sum();

        self
    }

    fn marginalize<'a, I>(mut self, x: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        // For each variable.
        let index: Vec<_> = x
            .into_iter()
            // Get variables indices.
            .map(|x| {
                self.levels
                    .get_index_of(x)
                    .expect("Failed to get variable index")
            })
            // Sort in decreasing order to ensure correctness.
            .sorted()
            .rev()
            // Collect to remove associated levels.
            .collect();

        // For each index.
        for x in index {
            // Sum given axis.
            self.values = self.values.sum_axis(Axis(x));
            // Remove associated level.
            self.levels.swap_remove_index(x);
        }

        // Re-sort levels variables.
        self.levels.sort_keys();

        self
    }

    fn reduce<'a, I>(mut self, x: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        // For each variable.
        let index: Vec<_> = x
            .into_iter()
            // Get variables and levels indices.
            .map(|(x, y)| {
                // Get variable index.
                let x = self
                    .levels
                    .get_index_of(x)
                    .expect("Failed to get variable index");
                // Get level index.
                let y = self
                    .levels
                    .get_index(x)
                    .expect("Failed to get variable by index")
                    .1
                    .get_index_of(y)
                    .expect("Failed to get level index");

                (x, y)
            })
            // Collect to remove associated levels.
            .collect();

        // For each (variable, level) index pairs.
        for (x, y) in index {
            // Reduce to given axis index.
            self.values.collapse_axis(Axis(x), y);
            // Reduce to given level.
            let y = self.levels[x]
                .swap_remove_index(y)
                .expect("Failed to get level by index");
            self.levels[x].clear();
            self.levels[x].insert(y);
        }

        self
    }
}

/// Discrete Conditional Probability Distribution (Discrete CPD).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteCPD {
    levels: FxIndexMap<String, FxIndexSet<String>>,
    values: ArrayD<f64>,
}

impl Display for DiscreteCPD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!() // FIXME:
    }
}

impl Add for DiscreteCPD {
    type Output = Self;

    fn add(self, cpd: Self) -> Self::Output {
        todo!() // FIXME:
    }
}

impl Mul for DiscreteCPD {
    type Output = Self;

    fn mul(self, cpd: Self) -> Self::Output {
        todo!() // FIXME:
    }
}

impl Div for DiscreteCPD {
    type Output = Self;

    fn div(self, cpd: Self) -> Self::Output {
        todo!() // FIXME:
    }
}

impl Factor for DiscreteCPD {
    type LabelsIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.levels.keys().map(|x| x.as_str())
    }

    #[inline]
    fn values(&self) -> &ndarray::ArrayD<f64> {
        &self.values
    }

    fn normalize(self) -> Self {
        todo!() // FIXME:
    }

    fn marginalize<'a, I>(self, x: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        todo!() // FIXME:
    }

    fn reduce<'a, I>(self, x: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        todo!() // FIXME:
    }
}

impl From<DiscreteCPD> for DiscreteFactor {
    fn from(cpd: DiscreteCPD) -> Self {
        todo!() // FIXME:
    }
}

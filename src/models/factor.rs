use std::{
    cmp::Ordering::{Equal, Less},
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display, Formatter},
    iter::{FusedIterator, Map},
    ops::{Add, Div, Mul},
};

use approx::*;
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
    Clone
    + Debug
    + Display
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Serialize
    + for<'a> Deserialize<'a>
{
    /// Labels iterator associated type.
    type ScopeIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Value type of the variables.
    type Value<'a>;

    /// Get the variables scope.
    fn scope(&self) -> Self::ScopeIter<'_>;

    /// Check whether a variable is in scope.
    fn in_scope(&self, x: &str) -> bool;

    /// Get reference to underlying values.
    fn values(&self) -> &ArrayD<f64>;

    /// Compute the factor normalization.
    fn normalize(self) -> Self;

    /// Compute the factor marginalization.
    fn marginalize<'a, I>(self, z: I) -> Self
    where
        I: IntoIterator<Item = &'a str>;

    /// Compute the factor reduction.
    fn reduce<'a, I>(self, z: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Self::Value<'a>)>;
}

/// Discrete factor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteFactor {
    states: FxIndexMap<String, FxIndexSet<String>>,
    values: ArrayD<f64>,
}

impl DiscreteFactor {
    /// Construct a new discrete factor given its values and states.
    pub fn new<D, I, J, K, V>(states: I, values: Array<f64, D>) -> Self
    where
        D: Dimension,
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Collect states.
        let states: FxIndexMap<String, FxIndexSet<String>> = states
            .into_iter()
            .map(|(x, ys)| (x.into(), ys.into_iter().map(|y| y.into()).collect()))
            .collect();
        // Compute factor values shape as given in input.
        let shape: Vec<usize> = states.values().map(|x| x.len()).collect();
        // Sort axes according to sorted variables scope.
        let mut axes: Vec<usize> = (0..states.len()).collect();
        axes.sort_by_key(|&i| {
            states
                .get_index(i)
                .expect("Failed to get variable label by index")
                .0
        });
        // Sort variables scope.
        let states: FxIndexMap<_, _> = states
            .into_iter()
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .collect();
        // Cast to n-dimensional array.
        let values = values
            // Reshape values to [X_0, X_1, ..., X_(n-1)].
            .into_shape(shape)
            .expect("Failed to reshape values")
            // Permute axes to align X axis w.r.t. sorted variables scope.
            .permuted_axes(axes)
            // Make into standard memory layout.
            .as_standard_layout()
            .to_owned()
            // Cast to dynamic shape.
            .into_dyn();

        Self { states, values }
    }

    /// Get the set of variables states.
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }
}

impl Display for DiscreteFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Create print table.
        let mut table = Table::new();
        // Add header to table.
        table.set_titles(self.states.keys().chain([&"Values".to_string()]).collect());
        // Construct iterator over states cartesian product.
        let states = self.states.values().multi_cartesian_product();
        // Add rows to table.
        for (i, x) in states.zip(self.values.iter()) {
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
        let states: FxIndexMap<_, _> = iter_set::union_by(
            self.states.clone().into_iter(),
            phi.states.clone().into_iter(),
            |(x, _), (y, _)| x.cmp(&y),
        )
        .collect();
        // Compute broadcasting shapes.
        let lhs = states
            .keys()
            .map(|x| self.states.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect_vec();
        let rhs = states
            .keys()
            .map(|x| phi.states.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect_vec();
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

        Self { states, values }
    }
}

impl Mul for DiscreteFactor {
    type Output = Self;

    fn mul(self, phi: Self) -> Self::Output {
        // Compute scope of factor product.
        let states: FxIndexMap<_, _> = iter_set::union_by(
            self.states.clone().into_iter(),
            phi.states.clone().into_iter(),
            |(x, _), (y, _)| x.cmp(&y),
        )
        .collect();
        // Compute broadcasting shapes.
        let lhs = states
            .keys()
            .map(|x| self.states.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect_vec();
        let rhs = states
            .keys()
            .map(|x| phi.states.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect_vec();
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

        Self { states, values }
    }
}

impl Div for DiscreteFactor {
    type Output = Self;

    fn div(self, phi: Self) -> Self::Output {
        // Compute scope and states of factor division.
        let states = self.states;
        // Assert RHS scope is subset of LHS scope.
        assert!(matches!(
            iter_set::cmp(phi.states.keys(), states.keys()),
            Some(Less) | Some(Equal)
        ));
        // Compute broadcasting shapes.
        let rhs = states
            .keys()
            .map(|x| phi.states.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect_vec();
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

        Self { states, values }
    }
}

impl Factor for DiscreteFactor {
    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.states.keys().map(|x| x.as_str())
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.states.contains_key(x)
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

    fn marginalize<'a, I>(mut self, z: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        // For each variable.
        let z: BTreeSet<_> = z
            .into_iter()
            // Get variables indices.
            .map(|x| {
                self.states
                    .get_index_of(x)
                    .expect("Failed to get variable index")
            })
            // Collect to sort and deduplicate states.
            .collect();

        // Sum in decreasing order to ensure correctness.
        for x in z.into_iter().rev() {
            // Sum given axis.
            self.values = self.values.sum_axis(Axis(x));
            // Remove associated state.
            self.states.swap_remove_index(x);
        }

        // Re-sort states variables.
        self.states.sort_keys();

        self
    }

    fn reduce<'a, I>(mut self, z: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        // For each variable.
        let z: BTreeMap<_, _> = z
            .into_iter()
            // Get variables and states indices, if present in scope.
            .filter_map(|(x, y)| {
                // Get variable index, if present in scope.
                self.states.get_index_of(x).map(|x| {
                    // Get state index.
                    (
                        x,
                        self.states[x]
                            .get_index_of(y)
                            .expect("Failed to get state index"),
                    )
                })
            })
            // Collect to sort and deduplicate states.
            .collect();

        // For each (variable, state) index pairs.
        for (x, y) in z {
            // Reduce to given axis index.
            self.values.collapse_axis(Axis(x), y);
            // Reduce to given state.
            let y = self.states[x]
                .swap_remove_index(y)
                .expect("Failed to get state by index");
            self.states[x].clear();
            self.states[x].insert(y);
        }

        self
    }
}

/// Discrete Conditional Probability Distribution (Discrete CPD).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteCPD {
    /// Target variable,
    x: String,
    /// Underlying factor.
    phi: DiscreteFactor,
}

impl DiscreteCPD {
    /// Construct a new tabular CPD given its values and states.
    pub fn new<I, J, K, V>((x, y): (K, J), z: I, values: Array2<f64>) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Cast target variable to String.
        let x = x.into();
        // Chain states as [X, Z].
        let states = [(x.clone(), y)]
            .into_iter()
            .chain(z.into_iter().map(|(s, t)| (s.into(), t)));
        // Assert sum over target axis yields ones.
        let values_sum = values.sum_axis(Axis(1));
        assert!(
            values_sum.iter().all(|x| x.relative_eq(&1., 1e-8, 1e-8)),
            "CPD rows must sum to one: {}",
            values_sum
        );
        // Align values axis [Z, X] to [X, Z] as states.
        let values = values.reversed_axes();
        // Construct underlying factor.
        let phi = DiscreteFactor::new(states, values);

        Self { x, phi }
    }

    /// Get the set of variables states.
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.phi.states
    }

    /// Get the target variable $X$ of the CPD $\mathcal(P)(X | \mathbf{Z})$
    #[inline]
    pub fn target(&self) -> &str {
        self.x.as_str()
    }
}

impl Display for DiscreteCPD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Create print table.
        let mut table = Table::new();
        // Get target, states and values.
        let (s, v) = (&self.phi.states, &self.phi.values);
        // Add first header to table. TODO: Add `with_hspan`if possible.
        table.set_titles(
            std::iter::repeat("")
                .take(s.len() - 1)
                .chain([self.x.as_str()])
                .collect(),
        );
        // Add second header to table.
        table.add_row(
            s.keys()
                .filter(|&x| !x.eq(&self.x))
                .chain(s[&self.x].iter())
                .collect(),
        );
        // If there are no conditioning variables ...
        if s.len() == 1 {
            // ... add only the row of marginal values.
            table.add_row(v.iter().map(|x| x.to_string()).collect());
            // Return table.
            return write!(f, "{table}");
        }
        // Get target index.
        let i = s
            .get_index_of(&self.x)
            .expect("Failed to get index of target variable");
        // Construct iterator over states levels.
        let states = s
            .iter()
            .filter_map(|(x, y)| match !x.eq(&self.x) {
                true => Some(y),
                false => None,
            })
            .multi_cartesian_product();
        // Construct iterator over values.
        let mut w = v.axis_iter(Axis(i)).map(|x| x.into_iter()).collect_vec();
        // Add rows to table.
        for s in states {
            table.add_row(
                s.into_iter()
                    .cloned()
                    .chain(w.iter_mut().map(|x| x.next().unwrap().to_string()))
                    .collect(),
            );
        }
        // Write table to formatter.
        write!(f, "{table}")
    }
}

impl Add for DiscreteCPD {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        // Compute factor addition.
        self.phi = self.phi + rhs.phi;
        // Return normalized CPD.
        self.normalize()
    }
}

impl Mul for DiscreteCPD {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        // Compute factor product.
        self.phi = self.phi * rhs.phi;
        // Return normalized CPD.
        self.normalize()
    }
}

impl Div for DiscreteCPD {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        // Compute factor division.
        self.phi = self.phi / rhs.phi;
        // Return normalized CPD.
        self.normalize()
    }
}

impl Factor for DiscreteCPD {
    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.phi.states.keys().map(|x| x.as_str())
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.phi.states.contains_key(x)
    }

    #[inline]
    fn values(&self) -> &ndarray::ArrayD<f64> {
        &self.phi.values
    }

    #[inline]
    fn normalize(mut self) -> Self {
        // Get normalization axis.
        let x = self
            .phi
            .states
            .get_index_of(&self.x)
            .expect("Failed to get target index");

        // Normalize over target axis.
        self.phi.values /= &self.phi.values.sum_axis(Axis(x)).insert_axis(Axis(x));

        self
    }

    #[inline]
    fn marginalize<'a, I>(mut self, z: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        // Assert variables do not include target.
        let z = z.into_iter().inspect(|&z| assert_ne!(z, self.x));
        // Marginalize underlying factor.
        self.phi = self.phi.marginalize(z);
        // Return normalized CPD.
        self.normalize()
    }

    #[inline]
    fn reduce<'a, I>(mut self, z: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        // Assert variables do not include target.
        let z = z.into_iter().inspect(|&(z, _)| assert_ne!(z, self.x));
        // Reduce underlying factor.
        self.phi = self.phi.reduce(z);
        // Return normalized CPD.
        self.normalize()
    }
}

impl From<DiscreteCPD> for DiscreteFactor {
    #[inline]
    fn from(other: DiscreteCPD) -> Self {
        // Return underlying factor.
        other.phi
    }
}

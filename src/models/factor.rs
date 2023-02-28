use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
    ops::{Add, Deref, Div, Mul},
};

use ndarray::prelude::*;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

use crate::utils::nan_to_zero;

pub trait Factor:
    Clone + Debug + Display + Add + Mul + Div + Serialize + for<'a> Deserialize<'a>
{
    fn labels(&self) -> &BTreeSet<String>;

    fn normalize(self) -> Self;

    fn marginalize<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = usize>;

    fn reduce<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = (usize, usize)>;
}

/// Categorical factor.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalFactor {
    values: ArrayD<f64>,
    labels: BTreeSet<String>,
    levels: FxHashMap<String, BTreeSet<String>>,
}

impl CategoricalFactor {
    /// Construct a new categorical factor given its values and levels.
    pub fn new<D, I, J, K>(values: Array<f64, D>, levels: I) -> Self
    where
        D: Dimension,
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = K>,
        K: Into<String>,
    {
        // Collect levels.
        let levels: FxHashMap<String, BTreeSet<String>> = levels
            .into_iter()
            .map(|(x, ys)| (x.into(), ys.into_iter().map(|y| y.into()).collect()))
            .collect();
        // Collect labels.
        let labels: BTreeSet<String> = levels.keys().map(|x| x.into()).collect();
        // Compute factor values shape.
        let shape: Vec<_> = labels
            .iter()
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

        Self {
            values,
            labels,
            levels,
        }
    }
}

impl Deref for CategoricalFactor {
    type Target = ArrayD<f64>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl Display for CategoricalFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!() // FIXME:
    }
}

impl Add for CategoricalFactor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!() // FIXME:
    }
}

impl Mul for CategoricalFactor {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // Compute scope of factor product.
        let labels = &self.labels | &other.labels;
        // Compute broadcasting shapes.
        let lhs: Vec<_> = labels
            .iter()
            .map(|x| self.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        let rhs: Vec<_> = labels
            .iter()
            .map(|x| other.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        // Apply broadcasting shapes.
        let lhs = self
            .values
            .into_shape(lhs)
            .expect("Failed to broadcast LHS factor values to given shape");
        let rhs = other
            .values
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor values to given shape");
        // Compute factor product.
        let values = (lhs * rhs).into_dyn();
        // Compute levels of product.
        let mut levels = self.levels;
        levels.extend(other.levels.into_iter());

        Self {
            values,
            labels,
            levels,
        }
    }
}

impl Div for CategoricalFactor {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        // Compute scope and levels of factor division.
        let (labels, levels) = (self.labels, self.levels);
        // Assert RHS scope is subset of LHS scope.
        assert!(other.labels.is_subset(&labels));
        // Compute broadcasting shapes.
        let rhs: Vec<_> = labels
            .iter()
            .map(|x| other.levels.get(x))
            .map(|x| x.map_or(1, |x| x.len()))
            .collect();
        // Apply broadcasting shapes.
        let rhs = other
            .values
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor values to given shape");
        // Compute factor division.
        let values = (self.values / rhs)
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            .into_dyn();

        Self {
            values,
            labels,
            levels,
        }
    }
}

impl Factor for CategoricalFactor {
    #[inline]
    fn labels(&self) -> &BTreeSet<String> {
        &self.labels
    }

    #[inline]
    fn normalize(mut self) -> Self {
        // Normalize values.
        self.values /= self.values.sum();

        self
    }

    fn marginalize<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        todo!() // FIXME:
    }

    fn reduce<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        todo!() // FIXME:
    }
}

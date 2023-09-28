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

pub trait Factor:
    Clone
    + Debug
    + Display
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + Eq
    + Serialize
    + for<'a> Deserialize<'a>
    + Send
    + Sync
    + Into<Self::Phi>
{
    type Phi: Factor;

    type ScopeIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    type Value<'a>;

    fn scope(&self) -> Self::ScopeIter<'_>;

    fn in_scope(&self, x: &str) -> bool;

    fn values(&self) -> &ArrayD<f64>;

    fn normalize(self) -> Self;

    fn marginalize<'a, Z>(self, z: Z) -> Self
    where
        Z: IntoIterator<Item = &'a str>;

    fn reduce<'a, Z>(self, z: Z) -> Self
    where
        Z: IntoIterator<Item = (&'a str, Self::Value<'a>)>;
}

pub trait JointProbabilityDistribution: Factor {
    fn from_factor(phi: Self::Phi) -> Self;
}

pub trait ConditionalProbabilityDistribution: Factor {
    fn from_factor(x: &str, phi: Self::Phi) -> Self;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalFactor {
    states: FxIndexMap<String, FxIndexSet<String>>,
    values: ArrayD<f64>,
}

impl CategoricalFactor {
    pub fn new<D, I, J, K, V>(states: I, values: Array<f64, D>) -> Self
    where
        D: Dimension,
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Collect states.
        let mut states: FxIndexMap<String, FxIndexSet<String>> = states
            .into_iter()
            .map(|(x, ys)| (x.into(), ys.into_iter().map_into().collect()))
            .collect();
        // Compute factor values shape as given in input.
        let shape = states.values().map(|x| x.len()).collect_vec();

        // Sort axes according to sorted variables scope.
        let mut axes = (0..states.len()).collect_vec();
        axes.sort_by_key(|&i| {
            states
                .get_index(i)
                .expect("Failed to get variable label by index")
                .0
        });
        // Sort variables scope.
        states.sort_keys();
        // Cast to n-dimensional array.
        let values = values
            // Reshape values to [X_0, X_1, ..., X_(n-1)].
            .into_shape(shape)
            .expect("Failed to reshape values")
            // Permute axes to align X axis w.r.t. sorted variables labels.
            .permuted_axes(axes)
            .into_dyn();

        // Align axes values w.r.t. sorted variables states.
        let mut axes = states
            .values()
            .map(|x| (0..x.len()).collect_vec())
            .collect_vec();
        axes.iter_mut()
            .zip(states.values())
            .for_each(|(axis, state)| axis.sort_by_key(|&i| &state[i]));
        // Sort variables states.
        states.values_mut().for_each(|x| x.sort());
        // Allocate new array for aligned values.
        let mut aligned_values = ArrayD::<f64>::zeros(values.shape());
        // Compute `from` and `to` indices.
        let axes = axes.into_iter().multi_cartesian_product().zip(
            states
                .values()
                .map(|x| 0..x.len())
                .multi_cartesian_product(),
        );
        // Permute values positions w.r.t. sorted variables states.
        axes.for_each(|(from, to)| aligned_values[to.as_slice()] = values[from.as_slice()]);

        // Cast to standard memory layout.
        let values = aligned_values.as_standard_layout().to_owned();

        Self { states, values }
    }

    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }
}

impl Display for CategoricalFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Convert to table.
        let table: Table = self.clone().into();
        // Write table to formatter.
        write!(f, "{table}")
    }
}

impl Add for CategoricalFactor {
    type Output = Self;

    fn add(self, phi: Self) -> Self::Output {
        // Compute scope of factor sum.
        let states: FxIndexMap<_, _> =
            iter_set::union_by(self.states.clone(), phi.states.clone(), |(x, _), (y, _)| {
                x.cmp(&y)
            })
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

impl Mul for CategoricalFactor {
    type Output = Self;

    fn mul(self, phi: Self) -> Self::Output {
        // Compute scope of factor product.
        let states: FxIndexMap<_, _> =
            iter_set::union_by(self.states.clone(), phi.states.clone(), |(x, _), (y, _)| {
                x.cmp(&y)
            })
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

impl Div for CategoricalFactor {
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

impl PartialEq for CategoricalFactor {
    fn eq(&self, other: &Self) -> bool {
        self.states == other.states && self.values.relative_eq(&other.values, 1e-8, 1e-8)
    }
}

impl Eq for CategoricalFactor {}

impl From<CategoricalFactor> for Table {
    fn from(other: CategoricalFactor) -> Table {
        // Create print table.
        let mut table = Table::new();
        // Add header to table.
        table.set_titles(other.states.keys().chain([&"Values".to_string()]).collect());
        // Construct iterator over states cartesian product.
        let states = other.states.values().multi_cartesian_product();
        // Add rows to table.
        for (i, x) in states.zip(other.values.iter()) {
            table.add_row(i.into_iter().chain([&x.to_string()]).collect());
        }

        table
    }
}

impl Factor for CategoricalFactor {
    type Phi = Self;

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

        // Assert values are in [0, 1].
        debug_assert!(self.values.iter().all(|x| (0. ..=1.).contains(x)));

        self
    }

    fn marginalize<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = &'a str>,
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

    fn reduce<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoricalJPD {
    phi: CategoricalFactor,
}

impl CategoricalJPD {
    pub fn new<D, I, J, K, V>(states: I, values: Array<f64, D>) -> Self
    where
        D: Dimension,
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Check all values are normalized.
        assert!(values.iter().all(|x| (0. ..=1.).contains(x)));
        // Construct underlying factor.
        let phi = CategoricalFactor::new(states, values);

        Self { phi }
    }

    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.phi.states()
    }
}

impl Display for CategoricalJPD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.phi, f)
    }
}

impl Add for CategoricalJPD {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        // Compute factor addition.
        self.phi = self.phi + rhs.phi;
        // Normalize values.
        self.normalize()
    }
}

impl Mul for CategoricalJPD {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        // Compute factor product.
        self.phi = self.phi * rhs.phi;
        // Normalize values.
        self.normalize()
    }
}

impl Div for CategoricalJPD {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        // Compute factor division.
        self.phi = self.phi / rhs.phi;
        // Normalize values.
        self.normalize()
    }
}

impl From<CategoricalJPD> for CategoricalFactor {
    #[inline]
    fn from(other: CategoricalJPD) -> Self {
        other.phi
    }
}

impl From<CategoricalJPD> for Table {
    fn from(other: CategoricalJPD) -> Table {
        other.phi.into()
    }
}

impl Factor for CategoricalJPD {
    type Phi = CategoricalFactor;

    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.phi.scope()
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.phi.in_scope(x)
    }

    #[inline]
    fn values(&self) -> &ndarray::ArrayD<f64> {
        self.phi.values()
    }

    #[inline]
    fn normalize(mut self) -> Self {
        // Normalize values.
        self.phi = self.phi.normalize();

        self
    }

    #[inline]
    fn marginalize<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = &'a str>,
    {
        // Marginalize underlying factor.
        self.phi = self.phi.marginalize(z);
        // Normalize values.
        self.normalize()
    }

    #[inline]
    fn reduce<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        // Reduce underlying factor.
        self.phi = self.phi.reduce(z);
        // Normalize values.
        self.normalize()
    }
}

impl JointProbabilityDistribution for CategoricalJPD {
    #[inline]
    fn from_factor(phi: Self::Phi) -> Self {
        // Normalize factor.
        let phi = phi.normalize();

        Self { phi }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoricalCPD {
    x: String,

    phi: CategoricalFactor,
}

impl CategoricalCPD {
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
        let phi = CategoricalFactor::new(states, values);

        Self { x, phi }
    }

    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.phi.states()
    }

    #[inline]
    pub fn target(&self) -> &str {
        self.x.as_str()
    }
}

impl Display for CategoricalCPD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Convert into table.
        let table: Table = self.clone().into();
        // Write table to formatter.
        write!(f, "{table}")
    }
}

impl Add for CategoricalCPD {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        // Compute factor addition.
        self.phi = self.phi + rhs.phi;
        // Normalize values.
        self.normalize()
    }
}

impl Mul for CategoricalCPD {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        // Compute factor product.
        self.phi = self.phi * rhs.phi;
        // Normalize values.
        self.normalize()
    }
}

impl Div for CategoricalCPD {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        // Compute factor division.
        self.phi = self.phi / rhs.phi;
        // Normalize values.
        self.normalize()
    }
}

impl From<CategoricalCPD> for CategoricalFactor {
    #[inline]
    fn from(other: CategoricalCPD) -> Self {
        other.phi
    }
}

impl From<CategoricalCPD> for Table {
    fn from(other: CategoricalCPD) -> Table {
        // Create print table.
        let mut table = Table::new();
        // Get target, states and values.
        let (s, v) = (&other.phi.states, &other.phi.values);
        // Add first header to table. TODO: Add `with_hspan`if possible.
        table.set_titles(
            std::iter::repeat("")
                .take(s.len() - 1)
                .chain([other.x.as_str()])
                .chain(std::iter::repeat("").take(s[&other.x].len() - 1))
                .collect(),
        );
        // Add second header to table.
        table.add_row(
            s.keys()
                .filter(|&x| !x.eq(&other.x))
                .chain(s[&other.x].iter())
                .collect(),
        );
        // If there are no conditioning variables ...
        if s.len() == 1 {
            // ... add only the row of marginal values.
            table.add_row(v.iter().map(|x| x.to_string()).collect());
            // Return table.
            return table;
        }
        // Get target index.
        let i = s
            .get_index_of(&other.x)
            .expect("Failed to get index of target variable");
        // Construct iterator over states levels.
        let states = s
            .iter()
            .filter_map(|(x, y)| match !x.eq(&other.x) {
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

        table
    }
}

impl Factor for CategoricalCPD {
    type Phi = CategoricalFactor;

    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.phi.scope()
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.phi.in_scope(x)
    }

    #[inline]
    fn values(&self) -> &ndarray::ArrayD<f64> {
        self.phi.values()
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

        // Assert values are in [0, 1].
        debug_assert!(self.phi.values.iter().all(|x| (0. ..=1.).contains(x)));

        self
    }

    #[inline]
    fn marginalize<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = &'a str>,
    {
        // Assert variables do not include target.
        let z = z.into_iter().inspect(|&z| assert_ne!(z, self.x));
        // Marginalize underlying factor.
        self.phi = self.phi.marginalize(z);
        // Normalize values.
        self.normalize()
    }

    #[inline]
    fn reduce<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        // Assert variables do not include target.
        let z = z.into_iter().inspect(|&(z, _)| assert_ne!(z, self.x));
        // Reduce underlying factor.
        self.phi = self.phi.reduce(z);
        // Normalize values.
        self.normalize()
    }
}

impl ConditionalProbabilityDistribution for CategoricalCPD {
    #[inline]
    fn from_factor(x: &str, phi: Self::Phi) -> Self {
        // Compute P(X | Z) as  P(X U Z) / P(Z).
        let mut phi = phi.clone() / phi.marginalize([x]);

        // Clone label.
        let x = x.to_owned();
        // Get normalization axis.
        let i = phi
            .states()
            .get_index_of(&x)
            .expect("Failed to get target index");
        // Normalize over target axis.
        phi.values /= &phi.values.sum_axis(Axis(i)).insert_axis(Axis(i));
        // Assert values are in [0, 1].
        debug_assert!(
            phi.values
                .iter()
                .all(|x| (0. ..=1.).contains(x) || x.is_nan()),
            "{}",
            phi.values
        );

        Self { x, phi }
    }
}

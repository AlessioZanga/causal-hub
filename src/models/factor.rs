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
    + PartialEq
    + Eq
    + Serialize
    + for<'a> Deserialize<'a>
    + Send
    + Sync
    + Into<Self::Phi>
{
    /// Underlying factor type.
    type Phi: Factor;

    /// Labels iterator associated type.
    type ScopeIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Value type of the variables.
    type Value<'a>;

    /// Get reference to underlying data.
    fn data(&self) -> &ArrayD<f64>;

    /// Get the variables scope.
    fn scope(&self) -> Self::ScopeIter<'_>;

    /// Check whether a variable is in scope.
    fn in_scope(&self, x: &str) -> bool;

    /// Compute the factor normalization.
    fn normalize(self) -> Self;

    /// Compute the factor marginalization.
    fn marginalize<'a, Z>(self, z: Z) -> Self
    where
        Z: IntoIterator<Item = &'a str>;

    /// Compute the factor reduction.
    fn reduce<'a, Z>(self, z: Z) -> Self
    where
        Z: IntoIterator<Item = (&'a str, Self::Value<'a>)>;
}

/// Joint Probability Distribution $\mathcal{P}(\mathbf{X})$ trait.
pub trait JointProbabilityDistribution: Factor {
    /// Construct joint distribution from associated factor.
    fn from_factor(phi: Self::Phi) -> Self;
}

/// Conditional Probability Distribution $\mathcal{P}(X \mid \mathbf{Z})$ trait.
pub trait ConditionalProbabilityDistribution: Factor {
    /// Construct conditional distribution from associated factor.
    fn from_factor(x: &str, phi: Self::Phi) -> Self;
}

/// Discrete Factor $\phi(\mathbf{X})$.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteFactor {
    states: FxIndexMap<String, FxIndexSet<String>>,
    data: ArrayD<f64>,
}

impl DiscreteFactor {
    /// Construct a new discrete factor given its data and states.
    pub fn new<D, I, J, K, V>(data: Array<f64, D>, states: I) -> Self
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
        // Compute factor data shape as given in input.
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
        let data = data
            // Reshape data to [X_0, X_1, ..., X_(n-1)].
            .into_shape(shape)
            .expect("Failed to reshape data")
            // Permute axes to align X axis w.r.t. sorted variables labels.
            .permuted_axes(axes)
            .into_dyn();

        // Align axes data w.r.t. sorted variables states.
        let mut axes = states
            .values()
            .map(|x| (0..x.len()).collect_vec())
            .collect_vec();
        axes.iter_mut()
            .zip(states.values())
            .for_each(|(axis, state)| axis.sort_by_key(|&i| &state[i]));
        // Sort variables states.
        states.values_mut().for_each(|x| x.sort());
        // Allocate new array for aligned data.
        let mut aligned_data = ArrayD::<f64>::zeros(data.shape());
        // Compute `from` and `to` indices.
        let axes = axes.into_iter().multi_cartesian_product().zip(
            states
                .values()
                .map(|x| 0..x.len())
                .multi_cartesian_product(),
        );
        // Permute data positions w.r.t. sorted variables states.
        axes.for_each(|(from, to)| aligned_data[to.as_slice()] = data[from.as_slice()]);

        // Cast to standard memory layout.
        let data = aligned_data.as_standard_layout().to_owned();

        Self { data, states }
    }

    /// Get the set of variables states.
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }
}

impl Display for DiscreteFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Convert to table.
        let table: Table = self.clone().into();
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
            .data
            .into_shape(lhs)
            .expect("Failed to broadcast LHS factor data to given shape");
        let rhs = phi
            .data
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor data to given shape");
        // Compute factor sum.
        let data = (lhs + rhs).into_dyn();

        Self { data, states }
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
            .data
            .into_shape(lhs)
            .expect("Failed to broadcast LHS factor data to given shape");
        let rhs = phi
            .data
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor data to given shape");
        // Compute factor product.
        let data = (lhs * rhs).into_dyn();

        Self { data, states }
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
            .data
            .into_shape(rhs)
            .expect("Failed to broadcast RHS factor data to given shape");
        // Compute factor division.
        let data = (self.data / rhs)
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            .into_dyn();

        Self { data, states }
    }
}

impl PartialEq for DiscreteFactor {
    fn eq(&self, other: &Self) -> bool {
        self.states == other.states && self.data.relative_eq(&other.data, 1e-8, 1e-8)
    }
}

impl Eq for DiscreteFactor {}

impl From<DiscreteFactor> for Table {
    fn from(other: DiscreteFactor) -> Table {
        // Create print table.
        let mut table = Table::new();
        // Add header to table.
        table.set_titles(other.states.keys().chain([&"Values".to_string()]).collect());
        // Construct iterator over states cartesian product.
        let states = other.states.values().multi_cartesian_product();
        // Add rows to table.
        for (i, x) in states.zip(other.data.iter()) {
            table.add_row(i.into_iter().chain([&x.to_string()]).collect());
        }

        table
    }
}

impl Factor for DiscreteFactor {
    type Phi = Self;

    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn data(&self) -> &ArrayD<f64> {
        &self.data
    }

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.states.keys().map(|x| x.as_str())
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.states.contains_key(x)
    }

    #[inline]
    fn normalize(mut self) -> Self {
        // Normalize data.
        self.data /= self.data.sum();

        // Assert data are in [0, 1].
        debug_assert!(self.data.iter().all(|x| (0. ..=1.).contains(x)));

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
            self.data = self.data.sum_axis(Axis(x));
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
            self.data.collapse_axis(Axis(x), y);
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

/// Discrete Joint Probability Distribution $\mathcal{P}(\mathbf{X})$ .
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscreteJPD {
    /// Underlying factor.
    phi: DiscreteFactor,
}

impl DiscreteJPD {
    /// Construct a new discrete JPD given its data and states.
    pub fn new<D, I, J, K, V>(data: Array<f64, D>, states: I) -> Self
    where
        D: Dimension,
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = V>,
        K: Into<String>,
        V: Into<String>,
    {
        // Check all data are normalized.
        assert!(data.iter().all(|x| (0. ..=1.).contains(x)));
        // Construct underlying factor.
        let phi = DiscreteFactor::new(data, states);

        Self { phi }
    }

    /// Get the set of variables states.
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.phi.states()
    }
}

impl Display for DiscreteJPD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.phi, f)
    }
}

impl Add for DiscreteJPD {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: Self) -> Self::Output {
        // Compute factor addition.
        self.phi = self.phi + rhs.phi;
        // Normalize data.
        self.normalize()
    }
}

impl Mul for DiscreteJPD {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        // Compute factor product.
        self.phi = self.phi * rhs.phi;
        // Normalize data.
        self.normalize()
    }
}

impl Div for DiscreteJPD {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        // Compute factor division.
        self.phi = self.phi / rhs.phi;
        // Normalize data.
        self.normalize()
    }
}

impl From<DiscreteJPD> for DiscreteFactor {
    #[inline]
    fn from(other: DiscreteJPD) -> Self {
        other.phi
    }
}

impl From<DiscreteJPD> for Table {
    fn from(other: DiscreteJPD) -> Table {
        other.phi.into()
    }
}

impl Factor for DiscreteJPD {
    type Phi = DiscreteFactor;

    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn data(&self) -> &ndarray::ArrayD<f64> {
        self.phi.data()
    }

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.phi.scope()
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.phi.in_scope(x)
    }

    #[inline]
    fn normalize(mut self) -> Self {
        // Normalize data.
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
        // Normalize data.
        self.normalize()
    }

    #[inline]
    fn reduce<'a, Z>(mut self, z: Z) -> Self
    where
        Z: IntoIterator<Item = (&'a str, Self::Value<'a>)>,
    {
        // Reduce underlying factor.
        self.phi = self.phi.reduce(z);
        // Normalize data.
        self.normalize()
    }
}

impl JointProbabilityDistribution for DiscreteJPD {
    #[inline]
    fn from_factor(phi: Self::Phi) -> Self {
        // Normalize factor.
        let phi = phi.normalize();

        Self { phi }
    }
}

/// Discrete Conditional Probability Distribution $\mathcal{P}(X \mid \mathbf{Z})$ .
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscreteCPD {
    /// Target variable,
    x: String,
    /// Underlying factor.
    phi: DiscreteFactor,
}

impl DiscreteCPD {
    /// Construct a new tabular CPD given its data and states.
    pub fn new<I, J, K, V>((x, y): (K, J), z: I, data: Array2<f64>) -> Self
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
        let data_sum = data.sum_axis(Axis(1));
        assert!(
            data_sum.iter().all(|x| x.relative_eq(&1., 1e-8, 1e-8)),
            "CPD rows must sum to one: {}",
            data_sum
        );
        // Align data axis [Z, X] to [X, Z] as states.
        let data = data.reversed_axes();
        // Construct underlying factor.
        let phi = DiscreteFactor::new(data, states);

        Self { x, phi }
    }

    /// Get the set of variables states.
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        self.phi.states()
    }

    /// Get the target variable $X$ of the CPD $\mathcal(P)(X | \mathbf{Z})$
    #[inline]
    pub fn target(&self) -> &str {
        self.x.as_str()
    }
}

impl Display for DiscreteCPD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Convert into table.
        let table: Table = self.clone().into();
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
        // Normalize data.
        self.normalize()
    }
}

impl Mul for DiscreteCPD {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        // Compute factor product.
        self.phi = self.phi * rhs.phi;
        // Normalize data.
        self.normalize()
    }
}

impl Div for DiscreteCPD {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        // Compute factor division.
        self.phi = self.phi / rhs.phi;
        // Normalize data.
        self.normalize()
    }
}

impl From<DiscreteCPD> for DiscreteFactor {
    #[inline]
    fn from(other: DiscreteCPD) -> Self {
        other.phi
    }
}

impl From<DiscreteCPD> for Table {
    fn from(other: DiscreteCPD) -> Table {
        // Create print table.
        let mut table = Table::new();
        // Get target, states and data.
        let (s, v) = (&other.phi.states, &other.phi.data);
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
            // ... add only the row of marginal data.
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
        // Construct iterator over data.
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

impl Factor for DiscreteCPD {
    type Phi = DiscreteFactor;

    type ScopeIter<'a> = Map<Keys<'a, String, FxIndexSet<String>>, fn(&'a String) -> &'a str>;

    type Value<'a> = &'a str;

    #[inline]
    fn data(&self) -> &ndarray::ArrayD<f64> {
        self.phi.data()
    }

    #[inline]
    fn scope(&self) -> Self::ScopeIter<'_> {
        self.phi.scope()
    }

    #[inline]
    fn in_scope(&self, x: &str) -> bool {
        self.phi.in_scope(x)
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
        self.phi.data /= &self.phi.data.sum_axis(Axis(x)).insert_axis(Axis(x));

        // Assert data are in [0, 1].
        debug_assert!(self.phi.data.iter().all(|x| (0. ..=1.).contains(x)));

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
        // Normalize data.
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
        // Normalize data.
        self.normalize()
    }
}

impl ConditionalProbabilityDistribution for DiscreteCPD {
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
        phi.data /= &phi.data.sum_axis(Axis(i)).insert_axis(Axis(i));
        // Assert data are in [0, 1].
        debug_assert!(
            phi.data
                .iter()
                .all(|x| (0. ..=1.).contains(x) || x.is_nan()),
            "{}",
            phi.data
        );

        Self { x, phi }
    }
}

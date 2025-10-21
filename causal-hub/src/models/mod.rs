mod bayesian_network;
use std::ops::{DivAssign, MulAssign};

use approx::{AbsDiffEq, RelativeEq};
pub use bayesian_network::*;

mod continuous_time_bayesian_network;
pub use continuous_time_bayesian_network::*;
use rand::Rng;

mod graphs;
use std::fmt::Debug;

pub use graphs::*;

use crate::types::{Labels, Set};

/// A trait for models with labelled variables.
pub trait Labelled {
    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Labels;

    /// Return the variable index for a given label.
    ///
    /// # Arguments
    ///
    /// * `x` - The label of the variable.
    ///
    /// # Panics
    ///
    /// * If the label is not in the map.
    ///
    /// # Returns
    ///
    /// The index of the variable.
    ///
    #[inline]
    fn label_to_index(&self, x: &str) -> usize {
        self.labels()
            .get_index_of(x)
            .unwrap_or_else(|| panic!("Variable `{x}` label does not exist."))
    }

    /// Return the label for a given variable index.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the variable.
    ///
    /// # Panics
    ///
    /// * If the index is out of bounds.
    ///
    /// # Returns
    ///
    /// The label of the variable.
    ///
    #[inline]
    fn index_to_label(&self, x: usize) -> &str {
        self.labels()
            .get_index(x)
            .unwrap_or_else(|| panic!("Variable `{x}` is out of bounds."))
    }

    /// Maps an index from this model to another model with the same label.
    ///
    /// # Arguments
    ///
    /// * `x` - The index in this model.
    /// * `other` - The labels of the other model.
    ///
    /// # Panics
    ///
    /// * If the index is out of bounds.
    /// * If the label does not exist in the other model.
    ///
    /// # Returns
    ///
    /// The index in the other model.
    ///
    #[inline]
    fn index_to(&self, x: usize, other: &Labels) -> usize {
        // Get the label of the variable in this model.
        let label = self.index_to_label(x);
        // Get the index of the variable in the other model.
        other.get_index_of(label).unwrap_or_else(|| {
            panic!("Variable `{label}` label does not exist in the other model.")
        })
    }

    /// Maps a set of indices from this model to another model with the same labels.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of indices in this model.
    /// * `other` - The labels of the other model.
    ///
    /// # Panics
    ///
    /// * If any index is out of bounds.
    /// * If any label does not exist in the other model.
    ///
    /// # Returns
    ///
    /// The set of indices in the other model.
    ///
    #[inline]
    fn indices_to(&self, x: &Set<usize>, other: &Labels) -> Set<usize> {
        x.iter().map(|&x| self.index_to(x, other)).collect()
    }

    /// Maps an index from another model to this model with the same label.
    ///
    /// # Arguments
    ///
    /// * `x` - The index in the other model.
    /// * `other` - The labels of the other model.
    ///
    /// # Panics
    ///
    /// * If the index is out of bounds.
    /// * If the label does not exist in this model.
    ///
    /// # Returns
    ///
    /// The index in this model.
    ///
    #[inline]
    fn index_from(&self, x: usize, other: &Labels) -> usize {
        // Get the label of the variable in the other model.
        let label = other
            .get_index(x)
            .unwrap_or_else(|| panic!("Variable `{x}` is out of bounds in the other model."));
        // Get the index of the variable in this model.
        self.labels()
            .get_index_of(label)
            .unwrap_or_else(|| panic!("Variable `{label}` label does not exist."))
    }

    /// Maps a set of indices from another model to this model with the same labels.
    ///
    /// # Arguments
    ///
    /// * `x` - The set of indices in the other model.
    /// * `other` - The labels of the other model.
    ///
    /// # Panics
    ///
    /// * If any index is out of bounds.
    /// * If any label does not exist in this model.
    ///
    /// # Returns
    ///
    /// The set of indices in this model.
    ///
    #[inline]
    fn indices_from(&self, x: &Set<usize>, other: &Labels) -> Set<usize> {
        x.iter().map(|&x| self.index_from(x, other)).collect()
    }
}

/// A trait for conditional probability distributions.
pub trait CPD: Clone + Debug + Labelled + PartialEq + AbsDiffEq + RelativeEq {
    /// The type of the support.
    type Support;
    /// The type of the parameters.
    type Parameters;
    /// The type of the sufficient statistics.
    type Statistics;

    /// Returns the labels of the conditioned variables.
    ///
    /// # Returns
    ///
    /// A reference to the conditioning labels.
    ///
    fn conditioning_labels(&self) -> &Labels;

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    fn parameters(&self) -> &Self::Parameters;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;

    /// Returns the sufficient statistics, if any.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the sufficient statistics.
    ///
    fn sample_statistics(&self) -> Option<&Self::Statistics>;

    /// Returns the log-likelihood of the fitted dataset, if any.
    ///
    /// # Returns
    ///
    /// An option containing the log-likelihood.
    ///
    fn sample_log_likelihood(&self) -> Option<f64>;

    /// Returns the value of probability (mass or density) function for P(X = x | Z = z).
    ///
    /// # Arguments
    ///
    /// * `x` - The value of the conditioned variables.
    /// * `z` - The value of the conditioning variables.
    ///
    /// # Returns
    ///
    /// The probability P(X = x | Z = z).
    ///
    fn pf(&self, x: &Self::Support, z: &Self::Support) -> f64;

    /// Samples from the conditional distribution P(X | Z = z).
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `z` - The value of the conditioning variables.
    ///
    /// # Returns
    ///
    /// A sample from P(X | Z = z).
    ///
    fn sample<R: Rng>(&self, rng: &mut R, z: &Self::Support) -> Self::Support;
}

/// A trait for conditional intensity matrices.
pub trait CIM: Clone + Debug + Labelled + PartialEq + AbsDiffEq + RelativeEq {
    /// The type of the support.
    type Support;
    /// The type of the parameters.
    type Parameters;
    /// The type of the sufficient statistics.
    type Statistics;

    /// Returns the labels of the conditioned variables.
    ///
    /// # Returns
    ///
    /// A reference to the conditioning labels.
    ///
    fn conditioning_labels(&self) -> &Labels;

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    fn parameters(&self) -> &Self::Parameters;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;

    /// Returns the sufficient statistics, if any.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the sufficient statistics.
    ///
    fn sample_statistics(&self) -> Option<&Self::Statistics>;

    /// Returns the log-likelihood of the fitted dataset, if any.
    ///
    /// # Returns
    ///
    /// An option containing the log-likelihood.
    ///
    fn sample_log_likelihood(&self) -> Option<f64>;
}

/// A trait for potential functions.
pub trait Phi:
    Clone
    + Debug
    + Labelled
    + PartialEq
    + AbsDiffEq
    + RelativeEq
    + for<'a> MulAssign<&'a Self>
    + for<'a> DivAssign<&'a Self>
{
    /// The type of the CPD.
    type CPD;
    /// The type of the parameters.
    type Parameters;
    /// The type of the evidence.
    type Evidence;

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    fn parameters(&self) -> &Self::Parameters;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;

    /// Conditions the potential on a set of variables.
    ///
    /// # Arguments
    ///
    /// * `e` - A map from variable indices to their observed states.
    ///
    /// # Returns
    ///
    /// A new potential instance.
    ///
    fn condition(&self, e: &Self::Evidence) -> Self;

    /// Marginalizes the potential over a set of variables.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of variable indices to marginalize over.
    ///
    /// # Returns
    ///
    /// A new potential instance.
    ///
    fn marginalize(&self, x: &Set<usize>) -> Self;

    /// Normalizes the potential.
    ///
    /// # Returns
    ///
    /// The normalized potential.
    ///
    fn normalize(&self) -> Self;

    /// Converts a CPD P(X | Z) to a potential \phi(X \cup Z).
    ///
    /// # Arguments
    ///
    /// * `cpd` - The CPD to convert.
    ///
    /// # Returns
    ///
    /// The corresponding potential.
    ///
    fn from_cpd(cpd: Self::CPD) -> Self;

    /// Converts a potential \phi(X \cup Z) to a CPD P(X | Z).
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Returns
    ///
    /// The corresponding CPD.
    ///
    fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> Self::CPD;
}

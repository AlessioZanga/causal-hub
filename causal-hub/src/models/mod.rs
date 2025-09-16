mod bayesian_network;
pub use bayesian_network::*;

mod continuous_time_bayesian_network;
pub use continuous_time_bayesian_network::*;

mod graphs;
pub use graphs::*;

mod potentials;
pub use potentials::*;

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
pub trait CPD {
    /// The type of the parameters.
    type Parameters;
    /// The type of the sufficient statistics.
    type SS;

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
}

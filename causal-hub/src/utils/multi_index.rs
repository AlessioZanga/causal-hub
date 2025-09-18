use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

/// A structure to compute the ravel index of a multi-dimensional array.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MI {
    shape: Array1<usize>,
    strides: Array1<usize>,
}

impl MI {
    /// Construct a new `MI` from the shape of each dimension.
    ///
    /// # Arguments
    ///
    /// * `shape` - An iterator over the shape of each dimension.
    ///
    /// # Returns
    ///
    /// A new `MI` instance.
    ///
    pub fn new<I>(shape: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Collect the multi index.
        let shape: Array1<_> = shape.into_iter().collect();
        // Allocate the strides of the parameters.
        let mut strides = Array1::from_elem(shape.len(), 1);
        // Compute cumulative product in reverse order (row-major strides).
        for i in (0..shape.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * shape[i + 1];
        }

        Self { shape, strides }
    }

    /// Return the number of dimensions.
    ///
    /// # Returns
    ///
    /// The number of dimensions.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        &self.shape
    }

    /// Compute the ravel index from a multi-dimensional index.
    ///
    /// # Arguments
    ///
    /// * `multi_index` - An iterator over the multi-dimensional index.
    ///
    /// # Returns
    ///
    /// The ravelled index.
    ///
    pub fn ravel<I>(&self, multi_index: I) -> usize
    where
        I: IntoIterator<Item = usize>,
    {
        self.strides
            .iter()
            .zip(multi_index)
            .map(|(i, j)| i * j)
            .sum()
    }

    /// Compute the multi-dimensional index from a ravelled index.
    ///
    /// # Arguments
    ///
    /// * `index` - The ravelled index.
    ///
    /// # Returns
    ///
    /// A vector containing the multi-dimensional index.
    ///
    pub fn unravel(&self, index: usize) -> Vec<usize> {
        let mut multi_index = Vec::with_capacity(self.shape.len());
        let mut remaining_index = index;

        for &stride in &self.strides {
            let value = remaining_index / stride;
            multi_index.push(value);
            remaining_index -= value * stride;
        }

        multi_index
    }
}

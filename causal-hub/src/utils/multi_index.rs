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
        // Compute cumulative product in reverse order (row-major strides) using scan.
        let mut strides: Vec<_> = shape
            .iter()
            .rev()
            .scan(1, |acc, &dim| {
                let stride = *acc;
                *acc *= dim;
                Some(stride)
            })
            .collect();
        // Reverse the strides to match the original order.
        strides.reverse();
        // Convert strides to array.
        let strides = Array1::from(strides);

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
        let mut remaining_index = index;

        self.strides
            .iter()
            .map(|&stride| {
                let value = remaining_index / stride;
                remaining_index %= stride;
                value
            })
            .collect()
    }
}

use ndarray::Array1;

/// A structure to compute the ravel index of a multi-dimensional array.
pub struct RavelMultiIndex {
    cardinality: Array1<usize>,
    strides: Array1<usize>,
}

/// A type alias for the ravel multi index.
pub type RMI = RavelMultiIndex;

impl RMI {
    /// Construct a new `RavelMultiIndex` from the cardinality of each dimension.
    ///
    /// # Arguments
    ///
    /// * `cardinality` - An iterator over the cardinality of each dimension.
    ///
    /// # Returns
    ///
    /// A new `RavelMultiIndex` instance.
    ///
    pub fn new<I>(cardinality: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Collect the multi index.
        let cardinality: Array1<_> = cardinality.into_iter().collect();
        // Allocate the strides of the parameters.
        let mut strides = Array1::from_elem(cardinality.len(), 1);
        // Compute cumulative product in reverse order (row-major strides).
        for i in (0..cardinality.len().saturating_sub(1)).rev() {
            strides[i] = strides[i + 1] * cardinality[i + 1];
        }

        Self {
            cardinality,
            strides,
        }
    }

    /// Return the number of dimensions.
    ///
    /// # Returns
    ///
    /// The number of dimensions.
    ///
    #[inline]
    pub fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
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
}

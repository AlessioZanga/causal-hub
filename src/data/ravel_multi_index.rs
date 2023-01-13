use ndarray::prelude::*;

/// Ravel multi-index to one-dimensional index.
pub struct RavelMultiIndex {
    ravel: Array1<usize>,
    size: usize,
}

impl RavelMultiIndex {
    /// Build the new multi-index map.
    pub fn new(cardinality: Array1<usize>) -> Self {
        // Assert non-empty.
        assert!(!cardinality.is_empty(), "Ravel multi index must not be empty");

        // Compute max size.
        let size = cardinality.product();

        // Make ravel mutable.
        let mut ravel = Array1::<usize>::ones(cardinality.dim());

        // From the end to the beginning of ravel ...
        for i in (1..ravel.len()).rev() {
            // ... compute the cumulative product.
            ravel[i - 1] = ravel[i] * cardinality[i];
        }

        Self { ravel, size }
    }

    /// Maps multi-index to one-dimensional index.
    pub fn call(&self, multi_index: Array1<usize>) -> usize {
        (&self.ravel * multi_index).sum()
    }

    /// Gets the maximum len of the associated one-dimensional axis.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.size
    }
}

use ndarray::prelude::*;

/// Ravel multi-index to one-dimensional index.
pub struct RavelMultiIndex {
    cardinality: Array1<usize>,
    ravel: Array1<usize>,
    size: usize,
}

impl RavelMultiIndex {
    /// Build the new multi-index map.
    pub fn new(cardinality: Array1<usize>) -> Self {
        // Assert non-empty.
        assert!(!cardinality.is_empty(), "Ravel multi index must not be empty");
        // Assert all strictly positive.
        assert!(
            cardinality.iter().all(|&x| x > 0),
            "Ravel multi index must not be empty"
        );

        // Compute max size.
        let size = cardinality.product();

        // Make ravel mutable.
        let mut ravel = Array1::<usize>::ones(cardinality.dim());

        // From the end to the beginning of ravel ...
        for i in (1..ravel.len()).rev() {
            // ... compute the cumulative product.
            ravel[i - 1] = ravel[i] * cardinality[i];
        }

        Self {
            cardinality,
            ravel,
            size,
        }
    }

    /// Maps multi-index to one-dimensional index.
    pub fn call(&self, multi_index: Array1<usize>) -> usize {
        (&self.ravel * multi_index).sum()
    }

    /// Gets the vector of variables cardinalities.
    pub fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }

    /// Gets the maximum len of the associated one-dimensional axis.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.size
    }
}

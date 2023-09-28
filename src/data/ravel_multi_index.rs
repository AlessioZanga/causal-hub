pub struct RavelMultiIndex {
    cardinality: Vec<usize>,
    ravel: Vec<usize>,
    size: usize,
}

impl RavelMultiIndex {
    #[inline]
    pub fn new<I>(cardinality: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        // Collect items into array.
        let cardinality = Vec::from_iter(cardinality);

        // Assert non-empty.
        assert!(
            !cardinality.is_empty(),
            "Ravel multi index must not be empty"
        );
        // Assert all strictly positive.
        assert!(
            cardinality.iter().all(|&x| x > 0),
            "Ravel multi index must not be empty"
        );

        // Compute max size.
        let size = cardinality.iter().product();

        // Make ravel mutable.
        let mut ravel = vec![1; cardinality.len()];

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

    #[inline]
    pub fn call<I>(&self, multi_index: I) -> usize
    where
        I: IntoIterator<Item = usize>,
    {
        self.ravel.iter().zip(multi_index).map(|(i, j)| i * j).sum()
    }

    #[inline]
    pub fn cardinality(&self) -> &Vec<usize> {
        &self.cardinality
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use causal_hub::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn call() {
        // Test that this implementation is not only equivalent, but the same, of Numpy.
        let map = RavelMultiIndex::new(array![6, 7, 8, 9]);

        assert_eq!(map.call(array![3, 1, 4, 1]), 1621);
    }
}

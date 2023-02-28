#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn mul() {
        // Initialize factors.
        let lhs = CategoricalFactor::new(
            array![0.5, 0.8, 0.1, 0., 0.3, 0.9],
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
        );
        let rhs = CategoricalFactor::new(
            array![0.5, 0.7, 0.1, 0.2],
            [("B", vec!["b1", "b2"]), ("C", vec!["c1", "c2"])],
        );
        // Compute factor product.
        let out = lhs * rhs;
        // Assert labels and levels of factor product.
        assert!(out.labels().iter().eq(&["A", "B", "C"]));
        // Assert values and shapes of factor product.
        assert_relative_eq!(
            out.deref(),
            &array![
                [[0.25, 0.35], [0.08, 0.16]],
                [[0.05, 0.07], [0., 0.]],
                [[0.15, 0.21], [0.09, 0.18]]
            ]
            .into_dyn()
        );
    }

    #[test]
    fn div() {
        // Initialize factors.
        let lhs = CategoricalFactor::new(
            array![0.5, 0.2, 0., 0., 0.3, 0.45],
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
        );
        let rhs = CategoricalFactor::new(array![0.8, 0., 0.6], [("A", vec!["a1", "a2", "a3"])]);
        // Compute factor division.
        let out = lhs / rhs;
        // Assert labels and levels of factor division.
        assert!(out.labels().iter().eq(&["A", "B"]));
        // Assert values and shapes of factor division.
        assert_relative_eq!(
            out.deref(),
            &array![[0.625, 0.25], [0., 0.], [0.5, 0.75]].into_dyn()
        );
    }
}

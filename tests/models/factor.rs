#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn mul() {
        // Initialize factors.
        let lhs = CategoricalFactor::new(
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
            array![0.5, 0.8, 0.1, 0., 0.3, 0.9],
        );
        let rhs = CategoricalFactor::new(
            [("B", vec!["b1", "b2"]), ("C", vec!["c1", "c2"])],
            array![0.5, 0.7, 0.1, 0.2],
        );
        // Compute factor product.
        let out = lhs * rhs;
        // Assert labels and levels of factor product.
        assert!(out.levels().keys().eq(&["A", "B", "C"]));
        // Assert values and shapes of factor product.
        assert_relative_eq!(
            out.values(),
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
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
            array![0.5, 0.2, 0., 0., 0.3, 0.45],
        );
        let rhs = CategoricalFactor::new([("A", vec!["a1", "a2", "a3"])], array![0.8, 0., 0.6]);
        // Compute factor division.
        let out = lhs / rhs;
        // Assert labels and levels of factor division.
        assert!(out.levels().keys().eq(&["A", "B"]));
        // Assert values and shapes of factor division.
        assert_relative_eq!(
            out.values(),
            &array![[0.625, 0.25], [0., 0.], [0.5, 0.75]].into_dyn()
        );
    }

    #[test]
    fn normalize() {
        // Initialize factor.
        let phi = CategoricalFactor::new(
            [
                ("A", vec!["a0", "a1"]),
                ("B", vec!["b0", "b1"]),
                ("C", vec!["c0", "c1"]),
                ("D", vec!["d0", "d1"]),
            ],
            array![
                300_000., 300_000., 300_000., 30., 500., 500., 5_000_000., 500., 100., 1_000_000.,
                100., 100., 10., 100_000., 100_000., 100_000.
            ],
        );

        assert_relative_eq!(
            phi.normalize().values(),
            &array![
                0.0416560212390167,
                0.0416560212390167,
                0.0416560212390167,
                4.16560212390167e-6,
                6.942670206502783e-5,
                6.942670206502783e-5,
                0.6942670206502782,
                6.942670206502783e-5,
                1.3885340413005566e-5,
                0.13885340413005565,
                1.3885340413005566e-5,
                1.3885340413005566e-5,
                1.3885340413005566e-6,
                0.013885340413005565,
                0.013885340413005565,
                0.013885340413005565
            ]
            .into_shape((2, 2, 2, 2))
            .unwrap()
            .into_dyn()
        );
    }

    #[test]
    fn marginalize() {
        // Initialize factor.
        let phi = CategoricalFactor::new(
            [
                ("A", vec!["a1", "a2", "a3"]),
                ("B", vec!["b1", "b2"]),
                ("C", vec!["c1", "c2"]),
            ],
            array![0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18],
        );

        assert_relative_eq!(
            phi.marginalize(["B"]).values(),
            &array![[0.33, 0.51], [0.05, 0.07], [0.24, 0.39]].into_dyn()
        );
    }
}

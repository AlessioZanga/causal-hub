#[cfg(test)]
mod discrete_factor {
    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn display() {
        // Initialize factor.
        let phi = DiscreteFactor::new(
            array![
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
            ],
            [
                ("A", vec!["a0", "a1"]),
                ("B", vec!["b0", "b1"]),
                ("C", vec!["c0", "c1"]),
                ("D", vec!["d0", "d1"]),
            ],
        );

        assert_eq!(
            format!("{phi}"),
            concat![
                "+----+----+----+----+--------------------------+\n",
                "| A  | B  | C  | D  | Values                   |\n",
                "+====+====+====+====+==========================+\n",
                "| a0 | b0 | c0 | d0 | 0.0416560212390167       |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b0 | c0 | d1 | 0.0416560212390167       |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b0 | c1 | d0 | 0.0416560212390167       |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b0 | c1 | d1 | 0.00000416560212390167   |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b1 | c0 | d0 | 0.00006942670206502783   |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b1 | c0 | d1 | 0.00006942670206502783   |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b1 | c1 | d0 | 0.6942670206502782       |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a0 | b1 | c1 | d1 | 0.00006942670206502783   |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b0 | c0 | d0 | 0.000013885340413005566  |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b0 | c0 | d1 | 0.13885340413005565      |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b0 | c1 | d0 | 0.000013885340413005566  |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b0 | c1 | d1 | 0.000013885340413005566  |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b1 | c0 | d0 | 0.0000013885340413005566 |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b1 | c0 | d1 | 0.013885340413005565     |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b1 | c1 | d0 | 0.013885340413005565     |\n",
                "+----+----+----+----+--------------------------+\n",
                "| a1 | b1 | c1 | d1 | 0.013885340413005565     |\n",
                "+----+----+----+----+--------------------------+\n",
            ]
        );
    }

    #[test]
    fn add() {
        // Initialize factors.
        let lhs = DiscreteFactor::new(
            array![0.5, 0.8, 0.1, 0., 0.3, 0.9],
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
        );
        let rhs = DiscreteFactor::new(
            array![0.5, 0.7, 0.1, 0.2],
            [("B", vec!["b1", "b2"]), ("C", vec!["c1", "c2"])],
        );
        // Compute factor sum.
        let out = lhs + rhs;
        // Assert labels and states of factor product.
        assert!(out.states().keys().eq(&["A", "B", "C"]));
        // Assert values and shapes of factor product.
        assert_relative_eq!(
            out.data(),
            &array![
                [[1.0, 1.2], [0.9, 1.0]],
                [[0.6, 0.8], [0.1, 0.2]],
                [[0.8, 1.0], [1.0, 1.1]]
            ]
            .into_dyn()
        );
    }

    #[test]
    fn mul() {
        // Initialize factors.
        let lhs = DiscreteFactor::new(
            array![0.5, 0.8, 0.1, 0., 0.3, 0.9],
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
        );
        let rhs = DiscreteFactor::new(
            array![0.5, 0.7, 0.1, 0.2],
            [("B", vec!["b1", "b2"]), ("C", vec!["c1", "c2"])],
        );
        // Compute factor product.
        let out = lhs * rhs;
        // Assert labels and states of factor product.
        assert!(out.states().keys().eq(&["A", "B", "C"]));
        // Assert values and shapes of factor product.
        assert_relative_eq!(
            out.data(),
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
        let lhs = DiscreteFactor::new(
            array![0.5, 0.2, 0., 0., 0.3, 0.45],
            [("A", vec!["a1", "a2", "a3"]), ("B", vec!["b1", "b2"])],
        );
        let rhs = DiscreteFactor::new(array![0.8, 0., 0.6], [("A", vec!["a1", "a2", "a3"])]);
        // Compute factor division.
        let out = lhs / rhs;
        // Assert labels and states of factor division.
        assert!(out.states().keys().eq(&["A", "B"]));
        // Assert values and shapes of factor division.
        assert_relative_eq!(
            out.data(),
            &array![[0.625, 0.25], [0., 0.], [0.5, 0.75]].into_dyn()
        );
    }

    #[test]
    fn normalize() {
        // Initialize factor.
        let phi = DiscreteFactor::new(
            array![
                300_000., 300_000., 300_000., 30., 500., 500., 5_000_000., 500., 100., 1_000_000.,
                100., 100., 10., 100_000., 100_000., 100_000.
            ],
            [
                ("A", vec!["a0", "a1"]),
                ("B", vec!["b0", "b1"]),
                ("C", vec!["c0", "c1"]),
                ("D", vec!["d0", "d1"]),
            ],
        );

        assert_relative_eq!(
            phi.normalize().data(),
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
        let phi = DiscreteFactor::new(
            array![0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18],
            [
                ("A", vec!["a1", "a2", "a3"]),
                ("B", vec!["b1", "b2"]),
                ("C", vec!["c1", "c2"]),
            ],
        );

        assert_relative_eq!(
            phi.marginalize(["B"]).data(),
            &array![[0.33, 0.51], [0.05, 0.07], [0.24, 0.39]].into_dyn()
        );
    }

    #[test]
    fn reduce() {
        // Initialize factor.
        let phi = DiscreteFactor::new(
            array![0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18],
            [
                ("A", vec!["a1", "a2", "a3"]),
                ("B", vec!["b1", "b2"]),
                ("C", vec!["c1", "c2"]),
            ],
        );

        assert_relative_eq!(
            phi.reduce([("C", "c1")]).data(),
            &array![[[0.25], [0.08]], [[0.05], [0.0]], [[0.15], [0.09]]].into_dyn()
        );
    }
}

mod discrete_cpd {
    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn display() {
        // Initialize CPD.
        let cpd = DiscreteCPD::new(
            ("Grade", vec!["g0", "g1", "g2"]),
            [
                ("Difficulty", vec!["d0", "d1"]),
                ("Intelligence", vec!["i0", "i1"]),
            ],
            array![
                [0.3, 0.4, 0.3],
                [0.05, 0.25, 0.7],
                [0.9, 0.08, 0.02],
                [0.5, 0.3, 0.2]
            ],
        );

        assert_eq!(
            format!("{cpd}"),
            concat!(
                "+------------+--------------+-------+------+------+\n",
                "|            |              | Grade |      |      |\n",
                "+============+==============+=======+======+======+\n",
                "| Difficulty | Intelligence | g0    | g1   | g2   |\n",
                "+------------+--------------+-------+------+------+\n",
                "| d0         | i0           | 0.3   | 0.4  | 0.3  |\n",
                "+------------+--------------+-------+------+------+\n",
                "| d0         | i1           | 0.9   | 0.08 | 0.02 |\n",
                "+------------+--------------+-------+------+------+\n",
                "| d1         | i0           | 0.05  | 0.25 | 0.7  |\n",
                "+------------+--------------+-------+------+------+\n",
                "| d1         | i1           | 0.5   | 0.3  | 0.2  |\n",
                "+------------+--------------+-------+------+------+\n",
            )
        );
    }

    #[test]
    fn add() {
        // Initialize CPD.
        let cpd = DiscreteCPD::new(
            ("Grade", vec!["g0", "g1", "g2"]),
            [
                ("Difficulty", vec!["d0", "d1"]),
                ("Intelligence", vec!["i0", "i1"]),
            ],
            array![
                [0.3, 0.4, 0.3],
                [0.05, 0.25, 0.7],
                [0.9, 0.08, 0.02],
                [0.5, 0.3, 0.2]
            ],
        );

        // Sum CPD.
        let out = cpd.clone() + cpd.clone();

        assert_relative_eq!(out.data(), cpd.data());
    }

    #[test]
    fn normalize() {
        // Initialize CPD.
        let cpd = DiscreteCPD::new(
            ("Grade", vec!["g0", "g1", "g2"]),
            [
                ("Difficulty", vec!["d0", "d1"]),
                ("Intelligence", vec!["i0", "i1"]),
            ],
            array![
                [0.3, 0.4, 0.3],
                [0.05, 0.25, 0.7],
                [0.9, 0.08, 0.02],
                [0.5, 0.3, 0.2]
            ],
        );

        assert_relative_eq!(cpd.clone().data(), cpd.normalize().data());
    }

    #[test]
    fn marginalize() {
        // Initialize CPD.
        let cpd = DiscreteCPD::new(
            ("Grade", vec!["g0", "g1", "g2"]),
            [
                ("Difficulty", vec!["d0", "d1"]),
                ("Intelligence", vec!["i0", "i1"]),
            ],
            array![
                [0.3, 0.4, 0.3],
                [0.05, 0.25, 0.7],
                [0.9, 0.08, 0.02],
                [0.5, 0.3, 0.2]
            ],
        );

        assert_relative_eq!(
            cpd.marginalize(["Intelligence"]).data(),
            &array![[0.6, 0.24, 0.16], [0.275, 0.275, 0.45]].into_dyn()
        );
    }

    #[test]
    fn reduce() {
        // Initialize CPD.
        let cpd = DiscreteCPD::new(
            ("Grade", vec!["g0", "g1", "g2"]),
            [
                ("Difficulty", vec!["d0", "d1"]),
                ("Intelligence", vec!["i0", "i1"]),
            ],
            array![
                [0.3, 0.4, 0.3],
                [0.05, 0.25, 0.7],
                [0.9, 0.08, 0.02],
                [0.5, 0.3, 0.2]
            ],
        );

        assert_relative_eq!(
            cpd.reduce([("Intelligence", "i0")]).data(),
            &array![[[0.3], [0.4], [0.3]], [[0.05], [0.25], [0.7]]].into_dyn()
        );
    }
}

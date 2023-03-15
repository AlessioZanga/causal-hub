#[cfg(test)]
mod variable_elimination {
    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;

    #[test]
    fn marginal() {
        // Initialize Bayesian network.
        let b = DiscreteBayesianNetwork::new(
            DiGraph::new(
                ["Difficulty", "Intelligence", "Grade", "SAT", "Letter"],
                [
                    ("Difficulty", "Grade"),
                    ("Intelligence", "Grade"),
                    ("Intelligence", "SAT"),
                    ("Grade", "Letter"),
                ],
            ),
            [
                DiscreteCPD::new(("Difficulty", vec!["d0", "d1"]), [], array![[0.6, 0.4]]),
                DiscreteCPD::new(("Intelligence", vec!["i0", "i1"]), [], array![[0.7, 0.3]]),
                DiscreteCPD::new(
                    ("Grade", vec!["g0", "g1", "g2"]),
                    [
                        ("Intelligence", vec!["i0", "i1"]),
                        ("Difficulty", vec!["d0", "d1"]),
                    ],
                    array![
                        [0.3, 0.4, 0.3],
                        [0.05, 0.25, 0.7],
                        [0.9, 0.08, 0.02],
                        [0.5, 0.3, 0.2]
                    ],
                ),
                DiscreteCPD::new(
                    ("SAT", vec!["s0", "s1"]),
                    [("Intelligence", vec!["i0", "i1"])],
                    array![[0.95, 0.05], [0.2, 0.8]],
                ),
                DiscreteCPD::new(
                    ("Letter", vec!["l0", "l1"]),
                    [("Grade", vec!["g0", "g1", "g2"])],
                    array![[0.1, 0.9], [0.4, 0.6], [0.99, 0.01]],
                ),
            ],
        );

        let true_dist = DiscreteJPD::new(
            [("Difficulty", vec!["d0", "d1"])],
            array![0.6, 0.4].into_dyn(),
        );

        // Construct estimator.
        let estimator = VariableElimination::new(&b);

        // Perform distribution estimation.
        let pred_dist = estimator.marginal("Difficulty");

        assert!(pred_dist.scope().eq(true_dist.scope()));
        assert_relative_eq!(pred_dist.values(), true_dist.values());
    }

    #[test]
    fn joint() {
        // Initialize Bayesian network.
        let b = DiscreteBayesianNetwork::new(
            DiGraph::new(
                ["Difficulty", "Intelligence", "Grade", "SAT", "Letter"],
                [
                    ("Difficulty", "Grade"),
                    ("Intelligence", "Grade"),
                    ("Intelligence", "SAT"),
                    ("Grade", "Letter"),
                ],
            ),
            [
                DiscreteCPD::new(("Difficulty", vec!["d0", "d1"]), [], array![[0.6, 0.4]]),
                DiscreteCPD::new(("Intelligence", vec!["i0", "i1"]), [], array![[0.7, 0.3]]),
                DiscreteCPD::new(
                    ("Grade", vec!["g0", "g1", "g2"]),
                    [
                        ("Intelligence", vec!["i0", "i1"]),
                        ("Difficulty", vec!["d0", "d1"]),
                    ],
                    array![
                        [0.3, 0.4, 0.3],
                        [0.05, 0.25, 0.7],
                        [0.9, 0.08, 0.02],
                        [0.5, 0.3, 0.2]
                    ],
                ),
                DiscreteCPD::new(
                    ("SAT", vec!["s0", "s1"]),
                    [("Intelligence", vec!["i0", "i1"])],
                    array![[0.95, 0.05], [0.2, 0.8]],
                ),
                DiscreteCPD::new(
                    ("Letter", vec!["l0", "l1"]),
                    [("Grade", vec!["g0", "g1", "g2"])],
                    array![[0.1, 0.9], [0.4, 0.6], [0.99, 0.01]],
                ),
            ],
        );

        let true_dist = DiscreteJPD::new(
            [("Difficulty", vec!["d0", "d1"])],
            array![0.6, 0.4].into_dyn(),
        );

        // Construct estimator.
        let estimator = VariableElimination::new(&b);

        // Perform distribution estimation.
        let pred_dist = estimator.joint(["Difficulty"]);

        assert_eq!(pred_dist, true_dist);
    }
}

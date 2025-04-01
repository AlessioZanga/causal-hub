#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub_next::{
        data::CategoricalData,
        distribution::Distribution,
        estimator::{Estimator, MLE},
    };
    use ndarray::prelude::*;

    #[test]
    fn test_fit() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![
            // A, B, C
            [0, 0, 0],
            [0, 0, 1],
            [1, 1, 0],
            [0, 1, 1],
            [1, 1, 1]
        ];
        let data = CategoricalData::new(&variables, values);

        let estimator = MLE::new(&data);

        // P(A)
        let distribution = estimator.fit(0, &[]);

        assert!(distribution.labels().iter().eq(["A"]));
        assert!(distribution
            .states()
            .values()
            .all(|x| x.iter().eq(["no", "yes"])));

        assert_eq!(
            distribution.parameters(),
            &array![
                // A: no, yes
                [0.6, 0.4]
            ]
        );

        assert_eq!(distribution.parameters_size(), 1);
        assert_eq!(distribution.sample_size(), Some(5));
        assert_relative_eq!(
            distribution.sample_log_likelihood().unwrap(),
            -3.365058335046282
        );

        assert_eq!(
            distribution.to_string(),
            concat!(
                "---------------\n",
                "| A    |      |\n",
                "| ---- | ---- |\n",
                "| no   | yes  |\n",
                "| ---- | ---- |\n",
                "| 0.60 | 0.40 |\n",
                "---------------\n",
            )
        );

        // P(A | B, C)
        let distribution = estimator.fit(0, &[1, 2]);

        assert!(distribution.labels().iter().eq(["A", "B", "C"]));
        assert!(distribution
            .states()
            .values()
            .all(|x| x.iter().eq(["no", "yes"])));

        assert_eq!(
            distribution.parameters(),
            &array![
                // A: no, yes
                [1.0, 0.0], // B: no, C: no
                [1.0, 0.0], // B: no, C: yes
                [0.0, 1.0], // B: yes, C: no
                [0.5, 0.5]  // B: yes, C: yes
            ]
        );

        assert_eq!(distribution.parameters_size(), 4);
        assert_eq!(distribution.sample_size(), Some(5));
        assert_relative_eq!(
            distribution.sample_log_likelihood().unwrap(),
            -1.3862943611198906
        );

        assert_eq!(
            distribution.to_string(),
            concat!(
                "-----------------------------\n",
                "|      |      | A    |      |\n",
                "| ---- | ---- | ---- | ---- |\n",
                "| B    | C    | no   | yes  |\n",
                "| ---- | ---- | ---- | ---- |\n",
                "| no   | no   | 1.00 | 0.00 |\n",
                "| no   | yes  | 1.00 | 0.00 |\n",
                "| yes  | no   | 0.00 | 1.00 |\n",
                "| yes  | yes  | 0.50 | 0.50 |\n",
                "-----------------------------\n",
            )
        );
    }

    #[test]
    #[should_panic(expected = "Variables to fit must be unique.")]
    fn test_unique_variables() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![
            // A, B, C
            [0, 0, 0],
            [0, 0, 1],
            [1, 1, 0],
            [0, 1, 1],
            [1, 1, 1]
        ];
        let data = CategoricalData::new(&variables, values);

        let estimator = MLE::new(&data);

        // P(A | A, C)
        let _distribution = estimator.fit(0, &[0, 2]);
    }

    #[test]
    #[should_panic(expected = "Marginal counts must be non-zero.")]
    fn test_non_zero_counts() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![
            // A, B, C
            [0, 0, 0],
            [0, 0, 1],
            [0, 1, 1],
            [1, 1, 1]
        ];
        let data = CategoricalData::new(&variables, values);

        let estimator = MLE::new(&data);

        // P(A | B, C)
        let _distribution = estimator.fit(0, &[1, 2]);
    }
}

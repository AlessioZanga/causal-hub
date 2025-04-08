#[cfg(test)]
mod tests {
    mod categorical_cpd {
        use approx::*;
        use causal_hub_next::{
            data::CategoricalData,
            distribution::Distribution,
            estimator::{CPDEstimator, BE},
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
            let data = CategoricalData::new(variables, values);

            let estimator = BE::new(1.0);

            // P(A)
            let distribution = estimator.fit(&data, 0, &[]);

            assert!(distribution.labels().iter().eq(["A"]));
            assert!(distribution
                .states()
                .values()
                .all(|x| x.iter().eq(["no", "yes"])));

            assert_relative_eq!(
                distribution.parameters(),
                &array![
                    // A: no, yes
                    [0.5714285714285714, 0.42857142857142855]
                ]
            );

            assert_eq!(distribution.parameters_size(), 1);
            assert_eq!(distribution.sample_size(), Some(5));
            assert_relative_eq!(
                distribution.sample_log_likelihood().unwrap(),
                -3.3734430845806758
            );

            assert_eq!(
                distribution.to_string(),
                concat!(
                    "---------------\n",
                    "| A    |      |\n",
                    "| ---- | ---- |\n",
                    "| no   | yes  |\n",
                    "| ---- | ---- |\n",
                    "| 0.57 | 0.43 |\n",
                    "---------------\n",
                )
            );

            // P(A | B, C)
            let distribution = estimator.fit(&data, 0, &[1, 2]);

            assert!(distribution.labels().iter().eq(["A", "B", "C"]));
            assert!(distribution
                .states()
                .values()
                .all(|x| x.iter().eq(["no", "yes"])));

            assert_relative_eq!(
                distribution.parameters(),
                &array![
                    // A: no, yes
                    [0.6666666666666666, 0.3333333333333333], // B: no, C: no
                    [0.6666666666666666, 0.3333333333333333], // B: no, C: yes
                    [0.3333333333333333, 0.6666666666666666], // B: yes, C: no
                    [0.5, 0.5]                                // B: yes, C: yes
                ]
            );

            assert_eq!(distribution.parameters_size(), 4);
            assert_eq!(distribution.sample_size(), Some(5));
            assert_relative_eq!(
                distribution.sample_log_likelihood().unwrap(),
                -2.602689685444384
            );

            assert_eq!(
                distribution.to_string(),
                concat!(
                    "-----------------------------\n",
                    "|      |      | A    |      |\n",
                    "| ---- | ---- | ---- | ---- |\n",
                    "| B    | C    | no   | yes  |\n",
                    "| ---- | ---- | ---- | ---- |\n",
                    "| no   | no   | 0.67 | 0.33 |\n",
                    "| no   | yes  | 0.67 | 0.33 |\n",
                    "| yes  | no   | 0.33 | 0.67 |\n",
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
            let data = CategoricalData::new(variables, values);

            let estimator = BE::new(1.0);

            // P(A | A, C)
            let _distribution = estimator.fit(&data, 0, &[0, 2]);
        }
    }
}

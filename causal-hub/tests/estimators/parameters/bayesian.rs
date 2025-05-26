#[cfg(test)]
mod tests {
    mod categorical {
        mod conditional_probability_distribution {
            use approx::*;
            use causal_hub::{
                datasets::CatData,
                distributions::CPD,
                estimators::{BE, CPDEstimator},
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
                let dataset = CatData::new(variables, values);

                let estimator = BE::new(&dataset, 1);

                // P(A)
                let distribution = estimator.fit(0, &[]);

                assert_eq!(distribution.label(), "A");
                assert!(distribution.states().iter().eq(["no", "yes"]));
                assert!(
                    distribution
                        .conditioning_labels()
                        .iter()
                        .eq(Vec::<&str>::new())
                );
                assert!(
                    distribution
                        .conditioning_states()
                        .values()
                        .all(|x| x.iter().eq(["no", "yes"]))
                );

                assert_relative_eq!(
                    distribution.parameters(),
                    &array![
                        // A: no, yes
                        [0.5714285714285714, 0.42857142857142855]
                    ]
                );

                assert_eq!(distribution.parameters_size(), 1);
                assert_eq!(distribution.sample_size(), Some(5.));
                assert_relative_eq!(
                    distribution.sample_log_likelihood().unwrap(),
                    -4.780356732903302
                );

                assert_eq!(
                    distribution.to_string(),
                    concat!(
                        "-----------------------\n",
                        "| A        |          |\n",
                        "| -------- | -------- |\n",
                        "| no       | yes      |\n",
                        "| -------- | -------- |\n",
                        "| 0.571429 | 0.428571 |\n",
                        "-----------------------\n",
                    )
                );

                // P(A | B, C)
                let distribution = estimator.fit(0, &[1, 2]);

                assert_eq!(distribution.label(), "A");
                assert!(distribution.states().iter().eq(["no", "yes"]));
                assert!(distribution.conditioning_labels().iter().eq(vec!["B", "C"]));
                assert!(
                    distribution
                        .conditioning_states()
                        .values()
                        .all(|x| x.iter().eq(["no", "yes"]))
                );

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
                assert_eq!(distribution.sample_size(), Some(5.));
                assert_relative_eq!(
                    distribution.sample_log_likelihood().unwrap(),
                    -8.501216236893097
                );

                assert_eq!(
                    distribution.to_string(),
                    concat!(
                        "---------------------------------------------\n",
                        "|          |          | A        |          |\n",
                        "| -------- | -------- | -------- | -------- |\n",
                        "| B        | C        | no       | yes      |\n",
                        "| -------- | -------- | -------- | -------- |\n",
                        "| no       | no       | 0.666667 | 0.333333 |\n",
                        "| no       | yes      | 0.666667 | 0.333333 |\n",
                        "| yes      | no       | 0.333333 | 0.666667 |\n",
                        "| yes      | yes      | 0.500000 | 0.500000 |\n",
                        "---------------------------------------------\n",
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
                let dataset = CatData::new(variables, values);

                let estimator = BE::new(&dataset, 1);

                // P(A | A, C)
                let _ = estimator.fit(0, &[0, 2]);
            }
        }
    }
}

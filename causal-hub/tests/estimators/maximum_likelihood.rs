#[cfg(test)]
mod tests {
    mod categorical_cpd {
        use approx::*;
        use causal_hub::{
            datasets::CategoricalDataset,
            distributions::CPD,
            estimators::{CPDEstimator, MLE},
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
            let dataset = CategoricalDataset::new(variables, values);

            let estimator = MLE::new(&dataset);

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
                    "-----------------------\n",
                    "| A        |          |\n",
                    "| -------- | -------- |\n",
                    "| no       | yes      |\n",
                    "| -------- | -------- |\n",
                    "| 0.600000 | 0.400000 |\n",
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
                    "---------------------------------------------\n",
                    "|          |          | A        |          |\n",
                    "| -------- | -------- | -------- | -------- |\n",
                    "| B        | C        | no       | yes      |\n",
                    "| -------- | -------- | -------- | -------- |\n",
                    "| no       | no       | 1.000000 | 0.000000 |\n",
                    "| no       | yes      | 1.000000 | 0.000000 |\n",
                    "| yes      | no       | 0.000000 | 1.000000 |\n",
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
            let dataset = CategoricalDataset::new(variables, values);

            let estimator = MLE::new(&dataset);

            // P(A | A, C)
            let _ = estimator.fit(0, &[0, 2]);
        }

        #[test]
        #[should_panic(expected = "Failed to get non-zero counts for variable 'A'.")]
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
            let dataset = CategoricalDataset::new(variables, values);

            let estimator = MLE::new(&dataset);

            // P(A | B, C)
            let _ = estimator.fit(0, &[1, 2]);
        }
    }

    mod categorical_bn {
        use approx::*;
        use causal_hub::{
            datasets::CategoricalDataset,
            distributions::CPD,
            estimators::{BNEstimator, MLE},
            graphs::{DiGraph, Graph},
            io::FromCsvReader,
            models::{BN, CatBN},
        };
        use csv::ReaderBuilder;
        use ndarray::prelude::*;

        #[test]
        fn test_fit() {
            let dataset = concat!(
                "A,B,C\n",
                "no,no,no\n",
                "no,no,yes\n",
                "no,yes,no\n",
                "no,yes,yes\n",
                "yes,no,no\n",
                "yes,no,yes\n",
                "yes,yes,no\n",
                "yes,yes,yes"
            );
            let dataset = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(dataset.as_bytes());
            let dataset = CategoricalDataset::from_csv_reader(dataset);

            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 2);

            let estimator = MLE::new(&dataset);

            let bn: CatBN = estimator.fit(graph);

            // P(A)
            let distribution = &bn.cpds()["A"];

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

            assert_eq!(
                distribution.parameters(),
                &array![
                    // A: no, yes
                    [0.5, 0.5]
                ]
            );

            assert_eq!(distribution.parameters_size(), 1);
            assert_eq!(distribution.sample_size(), Some(8));
            assert_relative_eq!(
                distribution.sample_log_likelihood().unwrap(),
                -5.545177444479562
            );

            assert_eq!(
                distribution.to_string(),
                concat!(
                    "-----------------------\n",
                    "| A        |          |\n",
                    "| -------- | -------- |\n",
                    "| no       | yes      |\n",
                    "| -------- | -------- |\n",
                    "| 0.500000 | 0.500000 |\n",
                    "-----------------------\n",
                )
            );
        }
    }
}

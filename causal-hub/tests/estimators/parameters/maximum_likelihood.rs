#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::{
        datasets::CatTable,
        estimators::{BNEstimator, CPDEstimator, MLE},
        io::CsvIO,
        labels,
        models::{BN, CPD, CatBN, DiGraph, Graph, Labelled},
        set, states,
    };
    use ndarray::prelude::*;

    mod conditional_probability_distribution {
        use super::*;

        mod categorical {
            use super::*;

            #[test]
            fn fit() {
                let states = states![
                    ("A", ["no", "yes"]),
                    ("B", ["no", "yes"]),
                    ("C", ["no", "yes"]),
                ];
                let values = array![
                    // A, B, C
                    [0, 0, 0],
                    [0, 0, 1],
                    [1, 1, 0],
                    [0, 1, 1],
                    [1, 1, 1]
                ];
                let dataset = CatTable::new(states, values);

                let estimator = MLE::new(&dataset);

                // P(A)
                let distribution = CPDEstimator::fit(&estimator, &set![0], &set![]);

                assert_eq!(&labels!["A"], distribution.labels());
                assert_eq!(&states![("A", ["no", "yes"])], distribution.states());
                assert_eq!(&labels![], distribution.conditioning_labels());
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
                assert_eq!(
                    distribution.sample_statistics().map(|s| s.sample_size()),
                    Some(5.)
                );
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
                let distribution = CPDEstimator::fit(&estimator, &set![0], &set![1, 2]);

                assert_eq!(&labels!["A"], distribution.labels());
                assert_eq!(&states![("A", ["no", "yes"])], distribution.states());
                assert_eq!(&labels!["B", "C"], distribution.conditioning_labels());
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
                assert_eq!(
                    distribution.sample_statistics().map(|s| s.sample_size()),
                    Some(5.)
                );
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
            #[should_panic(expected = "Variables and conditioning variables must be disjoint.")]
            fn unique_variables() {
                let states = states![
                    ("A", ["no", "yes"]),
                    ("B", ["no", "yes"]),
                    ("C", ["no", "yes"]),
                ];
                let values = array![
                    // A, B, C
                    [0, 0, 0],
                    [0, 0, 1],
                    [1, 1, 0],
                    [0, 1, 1],
                    [1, 1, 1]
                ];
                let dataset = CatTable::new(states, values);

                let estimator = MLE::new(&dataset);

                // P(A | A, C)
                let _ = CPDEstimator::fit(&estimator, &set![0], &set![0, 2]);
            }

            #[test]
            #[should_panic(expected = "Failed to get non-zero counts.")]
            fn non_zero_counts() {
                let states = states![
                    ("A", ["no", "yes"]),
                    ("B", ["no", "yes"]),
                    ("C", ["no", "yes"]),
                ];
                let values = array![
                    // A, B, C
                    [0, 0, 0],
                    [0, 0, 1],
                    [0, 1, 1],
                    [1, 1, 1]
                ];
                let dataset = CatTable::new(states, values);

                let estimator = MLE::new(&dataset);

                // P(A | B, C)
                let _ = CPDEstimator::fit(&estimator, &set![0], &set![1, 2]);
            }
        }

        mod gaussian {
            use causal_hub::datasets::GaussTable;

            use super::*;

            #[test]
            fn fit() {
                let csv = concat!(
                    "A,B,C\n",
                    "0.7244759610996034,2.6833663940564696,-1.1657447906269098\n",
                    "-0.8493802792558207,-1.3627303887930888,0.4017010838572639\n",
                    "-0.6667130630722627,-0.8603836511899117,1.0321217333118256\n",
                    "0.3512010732206103,0.05304024717622979,0.26298061562130404\n",
                    "-0.9484265435265308,-1.2909828103942118,0.05138052693081896\n",
                    "1.4224598866808345,3.976027760990921,-1.915431519452976\n",
                    "-1.0024750147169819,-4.141699082447313,4.110922335613383\n",
                    "0.8841740094546542,1.1265489405081641,0.4276912680121733\n",
                    "0.302767814223984,2.833698289205031,-1.9026596606194954\n",
                    "0.7850467625426617,0.8527120967629328,1.3250986082936653",
                );
                let dataset = GaussTable::from_csv(csv);

                let estimator = MLE::new(&dataset);

                // P(A)
                let distribution = CPDEstimator::fit(&estimator, &set![0], &set![]);
                assert_relative_eq!(
                    distribution.parameters().coefficients(), //
                    &array![[]]
                );
                assert_relative_eq!(
                    distribution.parameters().intercept(), //
                    &array![0.1],
                    epsilon = 1e-1
                );
                /* FIXME:
                assert_relative_eq!(
                    distribution.parameters().covariance(), //
                    &array![[1.]],
                    epsilon = 1e-1
                );
                */

                // P(B | A, C)
                let distribution = CPDEstimator::fit(&estimator, &set![1], &set![0, 2]);
                assert_relative_eq!(
                    distribution.parameters().coefficients(), //
                    &array![[1.5, -0.8]],
                    epsilon = 1e-1
                );
                assert_relative_eq!(
                    distribution.parameters().intercept(), //
                    &array![0.5],
                    epsilon = 1e-1
                );
                /* FIXME:
                assert_relative_eq!(
                    distribution.parameters().covariance(), //
                    &array![[0.49]],
                    epsilon = 1e-1
                );
                */

                // P(C)
                let distribution = CPDEstimator::fit(&estimator, &set![2], &set![]);
                assert_relative_eq!(
                    distribution.parameters().coefficients(), //
                    &array![[]]
                );
                /* FIXME:
                assert_relative_eq!(
                    distribution.parameters().intercept(), //
                    &array![1.],
                    epsilon = 1e-1
                );
                */
                /* FIXME:
                assert_relative_eq!(
                    distribution.parameters().covariance(), //
                    &array![[4.]],
                    epsilon = 1e-1
                );
                */
            }
        }
    }

    mod bayesian_network {
        use super::*;

        mod categorical {
            use super::*;

            #[test]
            fn fit() {
                let csv = concat!(
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
                let dataset = CatTable::from_csv(csv);

                let mut graph = DiGraph::empty(["A", "B", "C"]);
                graph.add_edge(0, 1);
                graph.add_edge(0, 2);
                graph.add_edge(1, 2);

                let estimator = MLE::new(&dataset);

                let bn: CatBN = BNEstimator::fit(&estimator, graph);

                // P(A)
                let distribution = &bn.cpds()["A"];

                assert_eq!(&labels!["A"], distribution.labels());
                assert_eq!(&states![("A", ["no", "yes"])], distribution.states());
                assert_eq!(&labels![], distribution.conditioning_labels());
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
                assert_eq!(
                    distribution.sample_statistics().map(|s| s.sample_size()),
                    Some(8.)
                );
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
}

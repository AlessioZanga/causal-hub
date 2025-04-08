#[cfg(test)]
mod tests {
    use causal_hub_next::{
        distribution::{CategoricalCPD, Distribution},
        graph::{DiGraph, Graph},
        model::{BayesianNetwork, CategoricalBN},
    };
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        // Initialize the graph.
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1); // A -> B
        graph.add_edge(0, 2); // A -> C
        graph.add_edge(1, 2); // B -> C

        // Initialize the distributions.
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                vec![("A", vec!["no", "yes"])], //
                array![[0.1, 0.9]],             //
            ),
            CategoricalCPD::new(
                // P(B | A)
                vec![
                    ("B", vec!["no", "yes"]), //
                    ("A", vec!["no", "yes"]), //
                ],
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            ),
            CategoricalCPD::new(
                // P(C | A, B)
                vec![
                    ("C", vec!["no", "yes"]), //
                    ("A", vec!["no", "yes"]), //
                    ("B", vec!["no", "yes"]), //
                ],
                array![
                    [0.1, 0.9], //
                    [0.3, 0.7], //
                    [0.5, 0.5], //
                    [0.6, 0.4], //
                ],
            ),
        ];
        // Initialize the model.
        let bn = CategoricalBN::new(graph, cpds);

        // Check the labels.
        assert!(bn.labels().iter().eq(["A", "B", "C"]));

        // Check the graph structure.
        assert_eq!(bn.graph().vertices().len(), 3);
        assert!(bn.graph().has_edge(0, 1));
        assert!(bn.graph().has_edge(0, 2));
        assert!(bn.graph().has_edge(1, 2));

        // Check the distributions.
        assert_eq!(bn.cdps().len(), 3);
        assert!(bn.cdps()[0].labels().iter().eq(["A"]));
        assert!(bn.cdps()[1].labels().iter().eq(["B", "A"]));
        assert!(bn.cdps()[2].labels().iter().eq(["C", "A", "B"]));

        // Check the states.
        assert_eq!(
            bn.cdps()[0].parameters(),
            &array![[0.1, 0.9]] //
        );
        assert_eq!(
            bn.cdps()[1].parameters(),
            &array![
                [0.2, 0.8], //
                [0.4, 0.6], //
            ]
        );
        assert_eq!(
            bn.cdps()[2].parameters(),
            &array![
                [0.1, 0.9], //
                [0.3, 0.7], //
                [0.5, 0.5], //
                [0.6, 0.4], //
            ]
        );

        // Check the sample size.
        assert_eq!(bn.parameters_size(), 7);
    }

    #[test]
    #[should_panic(expected = "Failed to map graph labels to distributions labels.")]
    fn test_unique_labels() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                vec![("A", vec!["no", "yes"])],
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(B | A)
                vec![("B", vec!["no", "yes"]), ("A", vec!["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];
        let _bn = CategoricalBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Failed to map graph labels to distributions labels.")]
    fn test_missing_distribution() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                vec![("A", vec!["no", "yes"])],
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(A)
                vec![("A", vec!["no", "yes"])],
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(B | A)
                vec![("B", vec!["no", "yes"]), ("A", vec!["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];
        let _bn = CategoricalBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Distributions labels must be the same as the graph parents.")]
    fn test_same_parents() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                vec![("A", vec!["no", "yes"])],
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(B | A)
                vec![("B", vec!["no", "yes"]), ("A", vec!["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
            CategoricalCPD::new(
                // P(C | A, B)
                vec![
                    ("C", vec!["no", "yes"]),
                    ("A", vec!["no", "yes"]),
                    ("B", vec!["no", "yes"]),
                ],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            ),
        ];
        let _bn = CategoricalBN::new(graph, cpds);
    }
}

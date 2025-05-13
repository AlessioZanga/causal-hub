#[cfg(test)]
mod tests {
    use causal_hub::{
        distributions::{CPD, CategoricalCPD},
        graphs::{DiGraph, Graph},
        models::{BN, CategoricalBN},
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
                ("A", vec!["no", "yes"]),        //
                Vec::<(&str, Vec<&str>)>::new(), //
                array![[0.1, 0.9]],              //
            ),
            CategoricalCPD::new(
                // P(B | A)
                ("B", vec!["no", "yes"]),       //
                vec![("A", vec!["no", "yes"])], //
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            ),
            CategoricalCPD::new(
                // P(C | A, B)
                ("C", vec!["no", "yes"]), //
                vec![
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
        assert_eq!(bn.cpds().len(), 3);
        assert_eq!(bn.cpds()[0].label(), "A");
        assert_eq!(bn.cpds()[1].label(), "B");
        assert_eq!(bn.cpds()[2].label(), "C");
        assert!(
            bn.cpds()[0]
                .conditioning_labels()
                .iter()
                .eq(Vec::<&str>::new())
        );
        assert!(bn.cpds()[1].conditioning_labels().iter().eq(["A"]));
        assert!(bn.cpds()[2].conditioning_labels().iter().eq(["A", "B"]));

        // Check the states.
        assert_eq!(
            bn.cpds()[0].parameters(),
            &array![[0.1, 0.9]] //
        );
        assert_eq!(
            bn.cpds()[1].parameters(),
            &array![
                [0.2, 0.8], //
                [0.4, 0.6], //
            ]
        );
        assert_eq!(
            bn.cpds()[2].parameters(),
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
    #[should_panic(expected = "Graph labels and distributions labels must be the same.")]
    fn test_unique_labels() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                ("A", vec!["no", "yes"]),
                Vec::<(&str, Vec<&str>)>::new(),
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(B | A)
                ("B", vec!["no", "yes"]),
                vec![("A", vec!["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];
        let _ = CategoricalBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Graph labels and distributions labels must be the same.")]
    fn test_missing_distribution() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                ("A", vec!["no", "yes"]),
                Vec::<(&str, Vec<&str>)>::new(),
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(A)
                ("A", vec!["no", "yes"]),
                Vec::<(&str, Vec<&str>)>::new(),
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(B | A)
                ("B", vec!["no", "yes"]),
                vec![("A", vec!["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];
        let _ = CategoricalBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Graph parents labels and conditioning labels must be the same.")]
    fn test_same_parents() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        let cpds = vec![
            CategoricalCPD::new(
                // P(A)
                ("A", vec!["no", "yes"]),
                Vec::<(&str, Vec<&str>)>::new(),
                array![[0.1, 0.9]],
            ),
            CategoricalCPD::new(
                // P(B | A)
                ("B", vec!["no", "yes"]),
                vec![("A", vec!["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
            CategoricalCPD::new(
                // P(C | A, B)
                ("C", vec!["no", "yes"]),
                vec![("A", vec!["no", "yes"]), ("B", vec!["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            ),
        ];
        let _ = CategoricalBN::new(graph, cpds);
    }
}

#[cfg(test)]
mod tests {
    use causal_hub::{
        map,
        models::{BN, CPD, CatBN, CatCPD, DiGraph, Graph, Labelled},
        set,
    };
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        // Initialize the graph.
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1); // A -> B
        graph.add_edge(0, 2); // A -> C
        graph.add_edge(1, 2); // B -> C

        // Set the states of the variables.
        let states = set!["no".to_owned(), "yes".to_owned()];

        // Initialize the distributions.
        let cpds = vec![
            CatCPD::new(
                // P(A)
                map![("A".to_owned(), states.clone())], //
                map![],                                 //
                array![[0.1, 0.9]],                     //
            ),
            CatCPD::new(
                // P(B | A)
                map![("B".to_owned(), states.clone())], //
                map![("A".to_owned(), states.clone())], //
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            ),
            CatCPD::new(
                // P(C | A, B)
                map![("C".to_owned(), states.clone())],
                map![
                    ("A".to_owned(), states.clone()),
                    ("B".to_owned(), states.clone()),
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
        let bn = CatBN::new(graph, cpds);

        // Check the labels.
        assert!(bn.labels().iter().eq(["A", "B", "C"]));

        // Check the graph structure.
        assert_eq!(bn.graph().vertices().len(), 3);
        assert!(bn.graph().has_edge(0, 1));
        assert!(bn.graph().has_edge(0, 2));
        assert!(bn.graph().has_edge(1, 2));

        // Check the distributions.
        assert_eq!(bn.cpds().len(), 3);
        assert_eq!(bn.cpds()[0].labels()[0], "A");
        assert_eq!(bn.cpds()[1].labels()[0], "B");
        assert_eq!(bn.cpds()[2].labels()[0], "C");
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
        let states = set!["no".to_owned(), "yes".to_owned()];
        let cpds = vec![
            CatCPD::new(
                // P(A)
                map![("A".to_owned(), states.clone())],
                map![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(B | A)
                map![("B".to_owned(), states.clone())],
                map![("A".to_owned(), states.clone())],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];
        let _ = CatBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Graph labels and distributions labels must be the same.")]
    fn test_missing_distribution() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        let states = set!["no".to_owned(), "yes".to_owned()];
        let cpds = vec![
            CatCPD::new(
                // P(A)
                map![("A".to_owned(), states.clone())],
                map![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(A)
                map![("A".to_owned(), states.clone())],
                map![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(B | A)
                map![("B".to_owned(), states.clone())],
                map![("A".to_owned(), states.clone())],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];
        let _ = CatBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Graph parents labels and conditioning labels must be the same.")]
    fn test_same_parents() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        let states = set!["no".to_owned(), "yes".to_owned()];
        let cpds = vec![
            CatCPD::new(
                // P(A)
                map![("A".to_owned(), states.clone())],
                map![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(B | A)
                map![("B".to_owned(), states.clone())],
                map![("A".to_owned(), states.clone())],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
            CatCPD::new(
                // P(C | A, B)
                map![("C".to_owned(), states.clone())],
                map![
                    ("A".to_owned(), states.clone()),
                    ("B".to_owned(), states.clone())
                ],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            ),
        ];
        let _ = CatBN::new(graph, cpds);
    }
}

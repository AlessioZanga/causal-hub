#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{BN, CPD, CatBN, CatCPD, DiGraph, Graph, Labelled},
        states,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() {
        // Initialize the graph.
        let mut graph = DiGraph::empty(["A", "B", "C"]);
        graph.add_edge(0, 1); // A -> B
        graph.add_edge(0, 2); // A -> C
        graph.add_edge(1, 2); // B -> C

        // Initialize the distributions.
        let cpds = [
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])], //
                states![],                     //
                array![[0.1, 0.9]],            //
            ),
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])], //
                states![("A", ["no", "yes"])],
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            ),
            CatCPD::new(
                // P(C | A, B)
                states![("C", ["no", "yes"])],
                states![("A", ["no", "yes"]), ("B", ["no", "yes"])],
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
        assert_eq!(&labels!["A", "B", "C"], bn.labels());

        // Check the graph structure.
        assert_eq!(bn.graph().vertices().len(), 3);
        assert!(bn.graph().has_edge(0, 1));
        assert!(bn.graph().has_edge(0, 2));
        assert!(bn.graph().has_edge(1, 2));

        // Check the distributions.
        assert_eq!(bn.cpds().len(), 3);
        assert_eq!(&labels!["A"], bn.cpds()[0].labels());
        assert_eq!(&labels!["B"], bn.cpds()[1].labels());
        assert_eq!(&labels!["C"], bn.cpds()[2].labels());
        assert_eq!(&labels![], bn.cpds()[0].conditioning_labels());
        assert_eq!(&labels!["A"], bn.cpds()[1].conditioning_labels());
        assert_eq!(&labels!["A", "B"], bn.cpds()[2].conditioning_labels());

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
    fn unique_labels() {
        let mut graph = DiGraph::empty(["A", "B", "C"]);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);

        let cpds = [
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])],
                states![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])],
                states![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];

        let _ = CatBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(expected = "Graph labels and distributions labels must be the same.")]
    fn missing_distribution() {
        let mut graph = DiGraph::empty(["A", "B", "C"]);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);

        let cpds = [
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])],
                states![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])],
                states![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])],
                states![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
        ];

        let _ = CatBN::new(graph, cpds);
    }

    #[test]
    #[should_panic(
        expected = "Graph parents labels and CPD conditioning labels must be the same:\n\t expected:    {\"A\"} ,\n\t found:       {\"A\", \"B\"} ."
    )]
    fn same_parents() {
        let mut graph = DiGraph::empty(["A", "B", "C"]);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        let cpds = [
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])],
                states![],
                array![[0.1, 0.9]],
            ),
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])],
                states![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            ),
            CatCPD::new(
                // P(C | A, B)
                states![("C", ["no", "yes"])],
                states![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            ),
        ];

        let _ = CatBN::new(graph, cpds);
    }
}

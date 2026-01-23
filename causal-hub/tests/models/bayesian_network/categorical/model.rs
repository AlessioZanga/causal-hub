#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{BN, CPD, CatBN, CatCPD, DiGraph, Graph, Labelled},
        states,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() -> Result<()> {
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
            )?,
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])], //
                states![("A", ["no", "yes"])],
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            )?,
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
            )?,
        ];
        // Initialize the model.
        let model = CatBN::new(graph, cpds)?;

        // Check the labels.
        assert_eq!(model.labels(), &labels!["A", "B", "C"]);

        // Check the graph structure.
        assert_eq!(model.graph().vertices().len(), 3);
        assert!(model.graph().has_edge(0, 1));
        assert!(model.graph().has_edge(0, 2));
        assert!(model.graph().has_edge(1, 2));

        // Check the distributions.
        assert_eq!(model.cpds().len(), 3);
        assert_eq!(model.cpds()[0].labels(), &labels!["A"]);
        assert_eq!(model.cpds()[1].labels(), &labels!["B"]);
        assert_eq!(model.cpds()[2].labels(), &labels!["C"]);
        assert_eq!(model.cpds()[0].conditioning_labels(), &labels![]);
        assert_eq!(model.cpds()[1].conditioning_labels(), &labels!["A"]);
        assert_eq!(model.cpds()[2].conditioning_labels(), &labels!["A", "B"]);

        // Check the states.
        assert_eq!(
            model.cpds()[0].parameters(),
            &array![[0.1, 0.9]] //
        );
        assert_eq!(
            model.cpds()[1].parameters(),
            &array![
                [0.2, 0.8], //
                [0.4, 0.6], //
            ]
        );
        assert_eq!(
            model.cpds()[2].parameters(),
            &array![
                [0.1, 0.9], //
                [0.3, 0.7], //
                [0.5, 0.5], //
                [0.6, 0.4], //
            ]
        );

        // Check the sample size.
        assert_eq!(model.parameters_size(), 7);

        Ok(())
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
            )
            .unwrap(),
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])],
                states![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )
            .unwrap(),
        ];

        let _ = CatBN::new(graph, cpds).unwrap();
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
            )
            .unwrap(),
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])],
                states![],
                array![[0.1, 0.9]],
            )
            .unwrap(),
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])],
                states![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )
            .unwrap(),
        ];

        let _ = CatBN::new(graph, cpds).unwrap();
    }

    #[test]
    fn same_parents() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"]);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        let cpds = [
            CatCPD::new(
                // P(A)
                states![("A", ["no", "yes"])],
                states![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                // P(B | A)
                states![("B", ["no", "yes"])],
                states![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?,
            CatCPD::new(
                // P(C | A, B)
                states![("C", ["no", "yes"])],
                states![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            )?,
        ];

        let res = CatBN::new(graph, cpds);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Model error: Graph parents labels and CPD conditioning labels must be the same:\n\t expected:    {\"A\"} ,\n\t found:       {\"A\", \"B\"} ."
        );

        Ok(())
    }
}

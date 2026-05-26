#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{BN, CPD, CatBN, CatCPD, DiGraph, Graph, Labelled},
        set, support,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() -> Result<()> {
        // Initialize the graph.
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?; // A -> B
        graph.add_edge(0, 2)?; // A -> C
        graph.add_edge(1, 2)?; // B -> C

        // Initialize the distributions.
        let cpds = [
            CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])], //
                support![],                     //
                array![[0.1, 0.9]],             //
            )?,
            CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])], //
                support![("A", ["no", "yes"])],
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            )?,
            CatCPD::new(
                // P(C | A, B)
                support![("C", ["no", "yes"])],
                support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
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
        assert!(model.graph().has_edge(0, 1)?);
        assert!(model.graph().has_edge(0, 2)?);
        assert!(model.graph().has_edge(1, 2)?);

        // Check the distributions.
        assert_eq!(model.cpds().len(), 3);
        assert_eq!(model.cpds()[0].labels(), &labels!["A"]);
        assert_eq!(model.cpds()[1].labels(), &labels!["B"]);
        assert_eq!(model.cpds()[2].labels(), &labels!["C"]);
        assert_eq!(model.cpds()[0].conditioning_labels(), &labels![]);
        assert_eq!(model.cpds()[1].conditioning_labels(), &labels!["A"]);
        assert_eq!(model.cpds()[2].conditioning_labels(), &labels!["A", "B"]);

        // Check the support.
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
    fn unique_labels() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;

        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 2)?;

        let cpds = [
            CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?,
        ];

        assert!(CatBN::new(graph, cpds).is_err());

        Ok(())
    }

    #[test]
    fn missing_distribution() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;

        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 2)?;

        let cpds = [
            CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?,
        ];

        assert!(CatBN::new(graph, cpds).is_err());

        Ok(())
    }

    #[test]
    fn same_parents() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;

        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;

        let cpds = [
            CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?,
            CatCPD::new(
                // P(C | A, B)
                support![("C", ["no", "yes"])],
                support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            )?,
        ];

        let res = CatBN::new(graph, cpds);
        match res {
            Err(err) => assert_eq!(
                err.kind.to_string(),
                "Labels mismatch: {\"A\"} != {\"A\", \"B\"}"
            ),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn select_subset() -> Result<()> {
        // A -> C and A -> B -> C means C depends on A and B.
        // Selecting A and C fails because C depends on B which is not selected.
        // So use a simpler graph: two independent root nodes.
        let graph = DiGraph::empty(["A", "B"])?;
        let cpds = [
            CatCPD::new(
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                support![("B", ["no", "yes"])],
                support![],
                array![[0.2, 0.8]],
            )?,
        ];
        let model = CatBN::new(graph, cpds)?;

        let sub = model.select(&set![1])?;
        assert_eq!(sub.labels(), &labels!["B"]);
        assert_eq!(sub.cpds().len(), 1);
        assert_eq!(sub.parameters_size(), 1);

        Ok(())
    }

    #[test]
    fn select_single_from_graph() -> Result<()> {
        // Select a root node with no parents.
        let mut graph = DiGraph::empty(["A", "B"])?;
        graph.add_edge(0, 1)?;
        let cpds = [
            CatCPD::new(
                support![("A", ["no", "yes"])],
                support![],
                array![[0.5, 0.5]],
            )?,
            CatCPD::new(
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.3, 0.7], [0.6, 0.4]],
            )?,
        ];
        let model = CatBN::new(graph, cpds)?;
        // Select A (root, no parents) - should work
        let sub = model.select(&set![0])?;
        assert_eq!(sub.labels(), &labels!["A"]);
        assert_eq!(sub.cpds().len(), 1);
        assert_eq!(sub.parameters_size(), 1);
        Ok(())
    }

    #[test]
    fn select_invalid_index() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.5, 0.5]],
        )?];
        let model = CatBN::new(graph, cpds)?;
        assert!(model.select(&set![5]).is_err());
        Ok(())
    }

    #[test]
    fn topological_order() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 2)?;
        let cpds = [
            CatCPD::new(
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?,
            CatCPD::new(
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?,
            CatCPD::new(
                support![("C", ["no", "yes"])],
                support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4]],
            )?,
        ];
        let model = CatBN::new(graph, cpds)?;
        assert_eq!(model.topological_order(), &[0, 1, 2]);
        Ok(())
    }

    #[test]
    fn with_optionals() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.5, 0.5]],
        )?];
        let model = CatBN::with_optionals(
            Some("test_model".to_string()),
            Some("A test".to_string()),
            graph,
            cpds,
        )?;
        assert_eq!(model.name(), Some("test_model"));
        assert_eq!(model.description(), Some("A test"));
        Ok(())
    }

    #[test]
    fn with_optionals_none() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.5, 0.5]],
        )?];
        let model = CatBN::with_optionals(None, None, graph, cpds)?;
        assert!(model.name().is_none());
        assert!(model.description().is_none());
        Ok(())
    }

    #[test]
    fn with_optionals_empty_name() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.5, 0.5]],
        )?];
        assert!(CatBN::with_optionals(Some(String::new()), None, graph, cpds,).is_err());
        Ok(())
    }

    #[test]
    fn with_optionals_empty_description() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.5, 0.5]],
        )?];
        assert!(CatBN::with_optionals(None, Some(String::new()), graph, cpds,).is_err());
        Ok(())
    }
}

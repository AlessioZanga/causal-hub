#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{BN, CPD, CatCPD, DiGraph, Graph, Labelled, MixedBN, MixedCPD},
        support,
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
            MixedCPD::from(CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])], //
                support![],                     //
                array![[0.1, 0.9]],             //
            )?),
            MixedCPD::from(CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])], //
                support![("A", ["no", "yes"])],
                array![
                    [0.2, 0.8], //
                    [0.4, 0.6], //
                ],
            )?),
            MixedCPD::from(CatCPD::new(
                // P(C | A, B)
                support![("C", ["no", "yes"])],
                support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![
                    [0.1, 0.9], //
                    [0.3, 0.7], //
                    [0.5, 0.5], //
                    [0.6, 0.4], //
                ],
            )?),
        ];
        // Initialize the model.
        let model = MixedBN::new(graph, cpds)?;

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

        // Check the parameters (via inner CPD matching).
        match &model.cpds()[0] {
            MixedCPD::Categorical(distribution) => {
                assert_eq!(distribution.parameters(), &array![[0.1, 0.9]])
            }
            _ => panic!("expected categorical"),
        }
        match &model.cpds()[1] {
            MixedCPD::Categorical(distribution) => {
                assert_eq!(distribution.parameters(), &array![[0.2, 0.8], [0.4, 0.6]])
            }
            _ => panic!("expected categorical"),
        }
        match &model.cpds()[2] {
            MixedCPD::Categorical(distribution) => {
                assert_eq!(
                    distribution.parameters(),
                    &array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4]]
                )
            }
            _ => panic!("expected categorical"),
        }

        // Check the parameters size.
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
            MixedCPD::from(CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?),
            MixedCPD::from(CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?),
        ];

        assert!(MixedBN::new(graph, cpds).is_err());

        Ok(())
    }

    #[test]
    fn missing_distribution() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;

        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 2)?;

        let cpds = [
            MixedCPD::from(CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?),
            MixedCPD::from(CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?),
            MixedCPD::from(CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?),
        ];

        assert!(MixedBN::new(graph, cpds).is_err());

        Ok(())
    }

    #[test]
    fn same_parents() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;

        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;

        let cpds = [
            MixedCPD::from(CatCPD::new(
                // P(A)
                support![("A", ["no", "yes"])],
                support![],
                array![[0.1, 0.9]],
            )?),
            MixedCPD::from(CatCPD::new(
                // P(B | A)
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?),
            MixedCPD::from(CatCPD::new(
                // P(C | A, B)
                support![("C", ["no", "yes"])],
                support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4],],
            )?),
        ];

        let res = MixedBN::new(graph, cpds);
        match res {
            Err(err) => assert_eq!(
                err.kind.to_string(),
                "Labels mismatch: {\"A\"} != {\"A\", \"B\"}"
            ),
            _ => panic!("Should be error"),
        };

        Ok(())
    }
}

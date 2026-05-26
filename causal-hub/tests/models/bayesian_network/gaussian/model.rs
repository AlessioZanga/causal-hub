#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{BN, DiGraph, GaussBN, GaussCPD, GaussCPDP, Graph, Labelled},
        set,
        types::Result,
    };
    use ndarray::prelude::*;

    fn empty_coeffs() -> Array2<f64> {
        Array2::zeros((1, 0))
    }

    #[test]
    fn new() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B"])?;
        graph.add_edge(0, 1)?;

        let cpds = [
            GaussCPD::new(
                labels!["A"],
                labels![],
                GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
            )?,
            GaussCPD::new(
                labels!["B"],
                labels!["A"],
                GaussCPDP::new(array![[2.0]], array![0.0], array![[0.5]])?,
            )?,
        ];

        let model = GaussBN::new(graph, cpds)?;
        assert_eq!(model.labels(), &labels!["A", "B"]);
        assert_eq!(model.cpds().len(), 2);
        assert!(model.graph().has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn select_subset() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;

        let cpds = [
            GaussCPD::new(
                labels!["A"],
                labels![],
                GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
            )?,
            GaussCPD::new(
                labels!["B"],
                labels!["A"],
                GaussCPDP::new(array![[2.0]], array![0.0], array![[0.5]])?,
            )?,
            GaussCPD::new(
                labels!["C"],
                labels!["A"],
                GaussCPDP::new(array![[3.0]], array![0.0], array![[0.25]])?,
            )?,
        ];

        let model = GaussBN::new(graph, cpds)?;
        let sub = model.select(&set![0, 2])?;
        assert_eq!(sub.labels(), &labels!["A", "C"]);
        assert_eq!(sub.cpds().len(), 2);
        assert!(sub.graph().has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn select_single_root() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [GaussCPD::new(
            labels!["A"],
            labels![],
            GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
        )?];
        let model = GaussBN::new(graph, cpds)?;
        let sub = model.select(&set![0])?;
        assert_eq!(sub.labels(), &labels!["A"]);
        assert_eq!(sub.cpds().len(), 1);
        Ok(())
    }

    #[test]
    fn select_invalid_index() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [GaussCPD::new(
            labels!["A"],
            labels![],
            GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
        )?];
        let model = GaussBN::new(graph, cpds)?;
        assert!(model.select(&set![5]).is_err());
        Ok(())
    }

    #[test]
    fn topological_order() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;

        let cpds = [
            GaussCPD::new(
                labels!["A"],
                labels![],
                GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
            )?,
            GaussCPD::new(
                labels!["B"],
                labels!["A"],
                GaussCPDP::new(array![[2.0]], array![0.0], array![[0.5]])?,
            )?,
            GaussCPD::new(
                labels!["C"],
                labels!["A"],
                GaussCPDP::new(array![[3.0]], array![0.0], array![[0.25]])?,
            )?,
        ];

        let model = GaussBN::new(graph, cpds)?;
        assert_eq!(model.topological_order(), &[0, 1, 2]);

        Ok(())
    }

    #[test]
    fn with_optionals() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [GaussCPD::new(
            labels!["A"],
            labels![],
            GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
        )?];

        let model = GaussBN::with_optionals(
            Some("gauss_test".to_string()),
            Some("A Gaussian test".to_string()),
            graph,
            cpds,
        )?;
        assert_eq!(model.name(), Some("gauss_test"));
        assert_eq!(model.description(), Some("A Gaussian test"));

        Ok(())
    }

    #[test]
    fn with_optionals_none() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;
        let cpds = [GaussCPD::new(
            labels!["A"],
            labels![],
            GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
        )?];
        let model = GaussBN::with_optionals(None, None, graph, cpds)?;
        assert!(model.name().is_none());
        assert!(model.description().is_none());
        Ok(())
    }

    #[test]
    fn parameters_size() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B"])?;
        graph.add_edge(0, 1)?;

        let cpds = [
            GaussCPD::new(
                labels!["A"],
                labels![],
                GaussCPDP::new(empty_coeffs(), array![0.0], array![[1.0]])?,
            )?,
            GaussCPD::new(
                labels!["B"],
                labels!["A"],
                GaussCPDP::new(array![[2.0]], array![0.0], array![[0.5]])?,
            )?,
        ];

        let model = GaussBN::new(graph, cpds)?;
        assert_eq!(model.parameters_size(), 5);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{DiGraph, Graph, Labelled},
        set,
        types::Result,
    };

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn has_edge() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "C", "B"])?;

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);
        assert!(!graph.has_edge(1, 0)?);
        assert!(!graph.has_edge(0, 2)?);

        Ok(())
    }

    #[test]
    fn add_edge() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn del_edge() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.del_edge(0, 1)?);
        assert!(!graph.has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_x() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        let result = graph.has_edge(5, 1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vertex `5` is out of bounds"
        );

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_y() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        let result = graph.has_edge(1, 5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vertex `5` is out of bounds"
        );

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        let result = graph.add_edge(5, 1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vertex `5` is out of bounds"
        );

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        let result = graph.add_edge(1, 5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vertex `5` is out of bounds"
        );

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        let result = graph.del_edge(5, 1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vertex `5` is out of bounds"
        );

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        let result = graph.del_edge(1, 5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Vertex `5` is out of bounds"
        );

        Ok(())
    }

    #[test]
    fn parents() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(1, 0)?);
        assert!(graph.add_edge(2, 0)?);
        assert!(graph.add_edge(3, 0)?);
        assert_eq!(graph.parents(&set![0])?, set![1, 2, 3]);
        assert_eq!(graph.parents(&set![1])?, set![]);
        assert_eq!(graph.parents(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn parents_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.parents(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn ancestors() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(1, 0)?);
        assert!(graph.add_edge(2, 0)?);
        assert!(graph.add_edge(3, 1)?);
        assert!(graph.add_edge(4, 2)?);
        assert_eq!(graph.ancestors(&set![0])?, set![1, 2, 4, 3]);
        assert_eq!(graph.ancestors(&set![1])?, set![3]);
        assert_eq!(graph.ancestors(&set![2])?, set![4]);
        assert_eq!(graph.ancestors(&set![3])?, set![]);
        assert_eq!(graph.ancestors(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn ancestors_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.ancestors(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn children() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(0, 2)?);
        assert!(graph.add_edge(0, 3)?);
        assert_eq!(graph.children(&set![0])?, set![1, 2, 3]);
        assert_eq!(graph.children(&set![1])?, set![]);
        assert_eq!(graph.children(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn children_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.children(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn descendants() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(0, 2)?);
        assert!(graph.add_edge(1, 3)?);
        assert!(graph.add_edge(2, 4)?);
        assert_eq!(graph.descendants(&set![0])?, set![1, 2, 4, 3]);
        assert_eq!(graph.descendants(&set![1])?, set![3]);
        assert_eq!(graph.descendants(&set![2])?, set![4]);
        assert_eq!(graph.descendants(&set![3])?, set![]);
        assert_eq!(graph.descendants(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn descendants_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.descendants(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn unique_labels() -> Result<()> {
        let labels = vec!["A", "A", "B"];
        let result = DiGraph::empty(labels);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Labels must be unique.");

        Ok(())
    }

    #[test]
    fn empty_labels() -> Result<()> {
        let labels: Vec<String> = vec![];
        DiGraph::empty(labels)?;

        Ok(())
    }
}

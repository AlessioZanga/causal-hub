#[cfg(test)]
mod tests {

    use causal_hub::{
        models::{Graph, Labelled, UnGraph},
        set,
        types::Result,
    };

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn has_edge() -> Result<()> {
        let mut graph = UnGraph::empty(["A", "C", "B"])?;

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 0)?);
        assert!(!graph.has_edge(0, 2)?);

        Ok(())
    }

    #[test]
    fn add_edge() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 0)?);

        Ok(())
    }

    #[test]
    fn del_edge() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.del_edge(0, 1)?);
        assert!(!graph.has_edge(0, 1)?);
        assert!(!graph.has_edge(1, 0)?);

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_x() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.has_edge(5, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_y() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.has_edge(1, 5);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.add_edge(5, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.add_edge(1, 5);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.del_edge(5, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.del_edge(1, 5);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn neighbors() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(0, 2)?);
        assert!(graph.add_edge(0, 3)?);
        assert_eq!(graph.neighbors(&set![0])?, set![1, 2, 3]);
        assert_eq!(graph.neighbors(&set![1])?, set![0]);
        assert_eq!(graph.neighbors(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn neighbors_out_of_bounds() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.neighbors(&set![5]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn unique_labels() -> Result<()> {
        let labels = vec!["A", "A", "B"];
        let res = UnGraph::empty(labels);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Labels must be unique.");

        Ok(())
    }

    #[test]
    fn empty_labels() -> Result<()> {
        let labels: Vec<String> = vec![];
        UnGraph::empty(labels)?;

        Ok(())
    }
}

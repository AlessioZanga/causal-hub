#[cfg(test)]
mod tests {
    use causal_hub::{
        inference::TopologicalOrder,
        models::{DiGraph, Graph},
        types::{Error, Result},
    };

    #[test]
    fn topological_order_simple() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;

        let sorted = graph.topological_order().ok_or(Error::NotADag)?;
        assert_eq!(sorted, [0, 1, 2]);

        Ok(())
    }

    #[test]
    fn topological_order_multiple_paths() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C", "D"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 3)?;
        graph.add_edge(2, 3)?;

        let sorted = graph.topological_order().ok_or(Error::NotADag)?;
        assert_eq!(sorted, [0, 1, 2, 3]);

        Ok(())
    }

    #[test]
    fn topological_order_disconnected_graph() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C", "D"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(2, 3)?;

        let sorted = graph.topological_order().ok_or(Error::NotADag)?;
        assert_eq!(sorted, [0, 2, 1, 3]);

        Ok(())
    }

    #[test]
    fn topological_order_single_vertex() -> Result<()> {
        let graph = DiGraph::empty(["A"])?;

        let sorted = graph.topological_order().ok_or(Error::NotADag)?;
        assert_eq!(sorted, [0]);

        Ok(())
    }

    #[test]
    fn topological_order_empty_graph() -> Result<()> {
        let labels: [String; 0] = [];
        let graph = DiGraph::empty(labels)?;

        let sorted = graph.topological_order().ok_or(Error::NotADag)?;
        assert!(sorted.is_empty());

        Ok(())
    }

    #[test]
    fn topological_order_cyclic_graph() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;
        graph.add_edge(2, 0)?;

        assert!(graph.topological_order().is_none());

        Ok(())
    }

    #[test]
    fn topological_order_self_loop() -> Result<()> {
        let mut graph = DiGraph::empty(["A"])?;
        graph.add_edge(0, 0)?;

        assert!(graph.topological_order().is_none());

        Ok(())
    }
}

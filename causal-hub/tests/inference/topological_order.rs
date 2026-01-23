#[cfg(test)]
mod tests {
    use causal_hub::{
        inference::TopologicalOrder,
        models::{DiGraph, Graph},
    };

    #[test]
    fn topological_order_simple() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = DiGraph::empty(["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let sorted = graph.topological_order().ok_or("No topological order")?;
        assert_eq!(sorted, [0, 1, 2]);

        Ok(())
    }

    #[test]
    fn topological_order_multiple_paths() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = DiGraph::empty(["A", "B", "C", "D"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let sorted = graph.topological_order().ok_or("No topological order")?;
        assert_eq!(sorted, [0, 1, 2, 3]);

        Ok(())
    }

    #[test]
    fn topological_order_disconnected_graph() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = DiGraph::empty(["A", "B", "C", "D"]);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);

        let sorted = graph.topological_order().ok_or("No topological order")?;
        assert_eq!(sorted, [0, 2, 1, 3]);

        Ok(())
    }

    #[test]
    fn topological_order_single_vertex() -> Result<(), Box<dyn std::error::Error>> {
        let graph = DiGraph::empty(["A"]);

        let sorted = graph.topological_order().ok_or("No topological order")?;
        assert_eq!(sorted, [0]);

        Ok(())
    }

    #[test]
    fn topological_order_empty_graph() -> Result<(), Box<dyn std::error::Error>> {
        let labels: [String; 0] = [];
        let graph = DiGraph::empty(labels);

        let sorted = graph.topological_order().ok_or("No topological order")?;
        assert!(sorted.is_empty());

        Ok(())
    }

    #[test]
    fn topological_order_cyclic_graph() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = DiGraph::empty(["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        assert!(graph.topological_order().is_none());

        Ok(())
    }

    #[test]
    fn topological_order_self_loop() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = DiGraph::empty(["A"]);
        graph.add_edge(0, 0);

        assert!(graph.topological_order().is_none());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use causal_hub::{
        inference::TopologicalOrder,
        models::{DiGraph, Graph},
    };

    #[test]
    fn test_topological_order_simple() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let sorted = graph.topological_order().unwrap();
        assert_eq!(sorted, vec![0, 1, 2]);
    }

    #[test]
    fn test_topological_order_multiple_paths() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C", "D"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let sorted = graph.topological_order().unwrap();
        assert_eq!(sorted, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_topological_order_disconnected_graph() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C", "D"]);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);

        let sorted = graph.topological_order().unwrap();
        assert_eq!(sorted, vec![0, 2, 1, 3]);
    }

    #[test]
    fn test_topological_order_single_vertex() {
        let graph = DiGraph::empty(vec!["A"]);

        let sorted = graph.topological_order().unwrap();
        assert_eq!(sorted, vec![0]);
    }

    #[test]
    fn test_topological_order_empty_graph() {
        let labels: Vec<String> = vec![];
        let graph = DiGraph::empty(labels);

        let sorted = graph.topological_order().unwrap();
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_topological_order_cyclic_graph() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        assert!(graph.topological_order().is_none());
    }

    #[test]
    fn test_topological_order_self_loop() {
        let mut graph = DiGraph::empty(vec!["A"]);
        graph.add_edge(0, 0);

        assert!(graph.topological_order().is_none());
    }
}

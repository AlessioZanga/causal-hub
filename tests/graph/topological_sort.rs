#[cfg(test)]
mod tests {
    use causal_hub_next::graph::{DiGraph, Graph, TopologicalSort};

    #[test]
    fn test_topological_sort_simple() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted, vec![0, 1, 2]);
    }

    #[test]
    fn test_topological_sort_multiple_paths() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C", "D"]);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_topological_sort_disconnected_graph() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C", "D"]);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);

        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted, vec![0, 2, 1, 3]);
    }

    #[test]
    fn test_topological_sort_single_node() {
        let graph = DiGraph::empty(vec!["A"]);

        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted, vec![0]);
    }

    #[test]
    fn test_topological_sort_empty_graph() {
        let graph = DiGraph::empty(vec![]);

        let sorted = graph.topological_sort().unwrap();
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_topological_sort_cyclic_graph() {
        let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        assert!(graph.topological_sort().is_none());
    }

    #[test]
    fn test_topological_sort_self_loop() {
        let mut graph = DiGraph::empty(vec!["A"]);
        graph.add_edge(0, 0);

        assert!(graph.topological_sort().is_none());
    }
}

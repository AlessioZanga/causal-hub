#[cfg(test)]
mod tests {
    use causal_hub_next::undirected_graph::UndirectedGraph;

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn test_has_edge() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.add_edge(0, 1);
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.add_edge(0, 1);
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
    }

    #[test]
    fn test_del_edge() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.add_edge(0, 1);
        graph.del_edge(0, 1);
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(1, 0));
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_has_edge_out_of_bounds_x() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.has_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_has_edge_out_of_bounds_y() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.has_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_add_edge_out_of_bounds_x() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.add_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_add_edge_out_of_bounds_y() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.add_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_del_edge_out_of_bounds_x() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.del_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_del_edge_out_of_bounds_y() {
        let mut graph = UndirectedGraph::new(&LABELS);
        graph.del_edge(1, 5);
    }
}

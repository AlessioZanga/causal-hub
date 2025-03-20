#[cfg(test)]
mod tests {
    use causal_hub_next::graph::undirected::UndirectedGraph;

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn test_has_edge() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
    }

    #[test]
    fn test_del_edge() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        assert!(graph.add_edge(0, 1));
        assert!(graph.del_edge(0, 1));
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(1, 0));
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_has_edge_out_of_bounds_x() {
        let graph = UndirectedGraph::empty(&LABELS);
        graph.has_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_has_edge_out_of_bounds_y() {
        let graph = UndirectedGraph::empty(&LABELS);
        graph.has_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_add_edge_out_of_bounds_x() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        graph.add_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_add_edge_out_of_bounds_y() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        graph.add_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_del_edge_out_of_bounds_x() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        graph.del_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_del_edge_out_of_bounds_y() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        graph.del_edge(1, 5);
    }

    #[test]
    fn test_neighbors() {
        let mut graph = UndirectedGraph::empty(&LABELS);
        assert!(graph.add_edge(0, 1));
        assert!(graph.add_edge(0, 2));
        assert!(graph.add_edge(0, 3));
        assert_eq!(graph.neighbors(0), vec![1, 2, 3]);
        assert_eq!(graph.neighbors(1), vec![0]);
        assert_eq!(graph.neighbors(4), vec![]);
    }

    #[test]
    #[should_panic(expected = "Vertex 5 index out of bounds")]
    fn test_neighbors_out_of_bounds() {
        let graph = UndirectedGraph::empty(&LABELS);
        graph.neighbors(5);
    }

    #[test]
    #[should_panic(expected = "Labels must be unique.")]
    fn test_unique_labels() {
        let labels = ["A", "A", "B"];
        UndirectedGraph::empty(&labels);
    }

    #[test]
    fn test_empty_labels() {
        let labels: [&str; 0] = [];
        UndirectedGraph::empty(&labels);
    }
}

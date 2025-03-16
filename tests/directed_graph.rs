#[cfg(test)]
mod tests {
    use causal_hub_next::directed_graph::DirectedGraph;

    #[test]
    fn test_has_edge() {
        let mut graph = DirectedGraph::new(5);
        graph.add_edge(0, 1);
        assert!(graph.has_edge(0, 1));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new(5);
        graph.add_edge(0, 1);
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn test_del_edge() {
        let mut graph = DirectedGraph::new(5);
        graph.add_edge(0, 1);
        graph.del_edge(0, 1);
        assert!(!graph.has_edge(0, 1));
    }
}

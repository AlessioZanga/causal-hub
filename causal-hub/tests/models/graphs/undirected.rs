#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{Graph, Labelled, UnGraph},
        set,
    };

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn has_edge() {
        let mut graph = UnGraph::empty(["A", "C", "B"]);

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn add_edge() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(1, 0));
    }

    #[test]
    fn del_edge() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.del_edge(0, 1));
        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(1, 0));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn has_edge_out_of_bounds_x() {
        let graph = UnGraph::empty(LABELS.to_vec());
        graph.has_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn has_edge_out_of_bounds_y() {
        let graph = UnGraph::empty(LABELS.to_vec());
        graph.has_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn add_edge_out_of_bounds_x() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        graph.add_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn add_edge_out_of_bounds_y() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        graph.add_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn del_edge_out_of_bounds_x() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        graph.del_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn del_edge_out_of_bounds_y() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        graph.del_edge(1, 5);
    }

    #[test]
    fn neighbors() {
        let mut graph = UnGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.add_edge(0, 2));
        assert!(graph.add_edge(0, 3));
        assert_eq!(graph.neighbors(&set![0]), set![1, 2, 3]);
        assert_eq!(graph.neighbors(&set![1]), set![0]);
        assert_eq!(graph.neighbors(&set![4]), set![]);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn neighbors_out_of_bounds() {
        let graph = UnGraph::empty(LABELS.to_vec());
        graph.neighbors(&set![5]);
    }

    #[test]
    #[should_panic(expected = "Labels must be unique.")]
    fn unique_labels() {
        let labels = vec!["A", "A", "B"];
        UnGraph::empty(labels);
    }

    #[test]
    fn empty_labels() {
        let labels: Vec<String> = vec![];
        UnGraph::empty(labels);
    }
}

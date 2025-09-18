#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{DiGraph, Graph, Labelled},
        set,
    };

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn has_edge() {
        let mut graph = DiGraph::empty(["A", "C", "B"]);

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn add_edge() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn del_edge() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.del_edge(0, 1));
        assert!(!graph.has_edge(0, 1));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn has_edge_out_of_bounds_x() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.has_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn has_edge_out_of_bounds_y() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.has_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn add_edge_out_of_bounds_x() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.add_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn add_edge_out_of_bounds_y() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.add_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn del_edge_out_of_bounds_x() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.del_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn del_edge_out_of_bounds_y() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.del_edge(1, 5);
    }

    #[test]
    fn parents() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(1, 0));
        assert!(graph.add_edge(2, 0));
        assert!(graph.add_edge(3, 0));
        assert_eq!(graph.parents(&set![0]), set![1, 2, 3]);
        assert_eq!(graph.parents(&set![1]), set![]);
        assert_eq!(graph.parents(&set![4]), set![]);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn parents_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.parents(&set![5]);
    }

    #[test]
    fn ancestors() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(1, 0));
        assert!(graph.add_edge(2, 0));
        assert!(graph.add_edge(3, 1));
        assert!(graph.add_edge(4, 2));
        assert_eq!(graph.ancestors(&set![0]), set![1, 2, 4, 3]);
        assert_eq!(graph.ancestors(&set![1]), set![3]);
        assert_eq!(graph.ancestors(&set![2]), set![4]);
        assert_eq!(graph.ancestors(&set![3]), set![]);
        assert_eq!(graph.ancestors(&set![4]), set![]);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn ancestors_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.ancestors(&set![5]);
    }

    #[test]
    fn children() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.add_edge(0, 2));
        assert!(graph.add_edge(0, 3));
        assert_eq!(graph.children(&set![0]), set![1, 2, 3]);
        assert_eq!(graph.children(&set![1]), set![]);
        assert_eq!(graph.children(&set![4]), set![]);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn children_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.children(&set![5]);
    }

    #[test]
    fn descendants() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.add_edge(0, 2));
        assert!(graph.add_edge(1, 3));
        assert!(graph.add_edge(2, 4));
        assert_eq!(graph.descendants(&set![0]), set![1, 2, 4, 3]);
        assert_eq!(graph.descendants(&set![1]), set![3]);
        assert_eq!(graph.descendants(&set![2]), set![4]);
        assert_eq!(graph.descendants(&set![3]), set![]);
        assert_eq!(graph.descendants(&set![4]), set![]);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn descendants_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.descendants(&set![5]);
    }

    #[test]
    #[should_panic(expected = "Labels must be unique.")]
    fn unique_labels() {
        let labels = vec!["A", "A", "B"];
        DiGraph::empty(labels);
    }

    #[test]
    fn empty_labels() {
        let labels: Vec<String> = vec![];
        DiGraph::empty(labels);
    }
}

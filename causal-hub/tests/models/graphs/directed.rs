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

        assert_eq!(graph.add_edge(0, 1), Ok(true));
        assert_eq!(graph.has_edge(0, 1), Ok(true));
        assert_eq!(graph.has_edge(1, 0), Ok(false));
        assert_eq!(graph.has_edge(0, 2), Ok(false));
    }

    #[test]
    fn add_edge() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert_eq!(graph.add_edge(0, 1), Ok(true));
        assert_eq!(graph.has_edge(0, 1), Ok(true));
    }

    #[test]
    fn del_edge() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert_eq!(graph.add_edge(0, 1), Ok(true));
        assert_eq!(graph.del_edge(0, 1), Ok(true));
        assert_eq!(graph.has_edge(0, 1), Ok(false));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn has_edge_out_of_bounds_x() {
        let graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.has_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn has_edge_out_of_bounds_y() {
        let graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.has_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn add_edge_out_of_bounds_x() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.add_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn add_edge_out_of_bounds_y() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.add_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn del_edge_out_of_bounds_x() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.del_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn del_edge_out_of_bounds_y() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.del_edge(1, 5);
    }

    #[test]
    fn parents() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert_eq!(graph.add_edge(1, 0), Ok(true));
        assert_eq!(graph.add_edge(2, 0), Ok(true));
        assert_eq!(graph.add_edge(3, 0), Ok(true));
        assert_eq!(graph.parents(&set![0]), Ok(set![1, 2, 3]));
        assert_eq!(graph.parents(&set![1]), Ok(set![]));
        assert_eq!(graph.parents(&set![4]), Ok(set![]));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn parents_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.parents(&set![5]);
    }

    #[test]
    fn ancestors() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert_eq!(graph.add_edge(1, 0), Ok(true));
        assert_eq!(graph.add_edge(2, 0), Ok(true));
        assert_eq!(graph.add_edge(3, 1), Ok(true));
        assert_eq!(graph.add_edge(4, 2), Ok(true));
        assert_eq!(graph.ancestors(&set![0]), Ok(set![1, 2, 4, 3]));
        assert_eq!(graph.ancestors(&set![1]), Ok(set![3]));
        assert_eq!(graph.ancestors(&set![2]), Ok(set![4]));
        assert_eq!(graph.ancestors(&set![3]), Ok(set![]));
        assert_eq!(graph.ancestors(&set![4]), Ok(set![]));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn ancestors_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.ancestors(&set![5]);
    }

    #[test]
    fn children() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert_eq!(graph.add_edge(0, 1), Ok(true));
        assert_eq!(graph.add_edge(0, 2), Ok(true));
        assert_eq!(graph.add_edge(0, 3), Ok(true));
        assert_eq!(graph.children(&set![0]), Ok(set![1, 2, 3]));
        assert_eq!(graph.children(&set![1]), Ok(set![]));
        assert_eq!(graph.children(&set![4]), Ok(set![]));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn children_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.children(&set![5]);
    }

    #[test]
    fn descendants() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert_eq!(graph.add_edge(0, 1), Ok(true));
        assert_eq!(graph.add_edge(0, 2), Ok(true));
        assert_eq!(graph.add_edge(1, 3), Ok(true));
        assert_eq!(graph.add_edge(2, 4), Ok(true));
        assert_eq!(graph.descendants(&set![0]), Ok(set![1, 2, 4, 3]));
        assert_eq!(graph.descendants(&set![1]), Ok(set![3]));
        assert_eq!(graph.descendants(&set![2]), Ok(set![4]));
        assert_eq!(graph.descendants(&set![3]), Ok(set![]));
        assert_eq!(graph.descendants(&set![4]), Ok(set![]));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn descendants_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        let _ = graph.descendants(&set![5]);
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

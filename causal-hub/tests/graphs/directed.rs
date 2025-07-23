#[cfg(test)]
mod tests {
    use causal_hub::graphs::{DiGraph, Graph};

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn test_has_edge() {
        let mut graph = DiGraph::empty(["A", "C", "B"]);

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(0, 2));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.has_edge(0, 1));
    }

    #[test]
    fn test_del_edge() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.del_edge(0, 1));
        assert!(!graph.has_edge(0, 1));
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_has_edge_out_of_bounds_x() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.has_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_has_edge_out_of_bounds_y() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.has_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_add_edge_out_of_bounds_x() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.add_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_add_edge_out_of_bounds_y() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.add_edge(1, 5);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_del_edge_out_of_bounds_x() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.del_edge(5, 1);
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_del_edge_out_of_bounds_y() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        graph.del_edge(1, 5);
    }

    #[test]
    fn test_parents() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(1, 0));
        assert!(graph.add_edge(2, 0));
        assert!(graph.add_edge(3, 0));
        assert_eq!(graph.parents(0), vec![1, 2, 3]);
        assert_eq!(graph.parents(1), Vec::<usize>::new());
        assert_eq!(graph.parents(4), Vec::<usize>::new());
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_parents_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.parents(5);
    }

    #[test]
    fn test_ancestors() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(1, 0));
        assert!(graph.add_edge(2, 0));
        assert!(graph.add_edge(3, 1));
        assert!(graph.add_edge(4, 2));
        assert_eq!(graph.ancestors(0), vec![1, 2, 4, 3]);
        assert_eq!(graph.ancestors(1), vec![3]);
        assert_eq!(graph.ancestors(2), vec![4]);
        assert_eq!(graph.ancestors(3), Vec::<usize>::new());
        assert_eq!(graph.ancestors(4), Vec::<usize>::new());
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_ancestors_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.ancestors(5);
    }

    #[test]
    fn test_children() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.add_edge(0, 2));
        assert!(graph.add_edge(0, 3));
        assert_eq!(graph.children(0), vec![1, 2, 3]);
        assert_eq!(graph.children(1), Vec::<usize>::new());
        assert_eq!(graph.children(4), Vec::<usize>::new());
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_children_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.children(5);
    }

    #[test]
    fn test_descendants() {
        let mut graph = DiGraph::empty(LABELS.to_vec());
        assert!(graph.add_edge(0, 1));
        assert!(graph.add_edge(0, 2));
        assert!(graph.add_edge(1, 3));
        assert!(graph.add_edge(2, 4));
        assert_eq!(graph.descendants(0), vec![1, 2, 4, 3]);
        assert_eq!(graph.descendants(1), vec![3]);
        assert_eq!(graph.descendants(2), vec![4]);
        assert_eq!(graph.descendants(3), Vec::<usize>::new());
        assert_eq!(graph.descendants(4), Vec::<usize>::new());
    }

    #[test]
    #[should_panic(expected = "Vertex `5` is out of bounds")]
    fn test_descendants_out_of_bounds() {
        let graph = DiGraph::empty(LABELS.to_vec());
        graph.descendants(5);
    }

    #[test]
    #[should_panic(expected = "Labels must be unique.")]
    fn test_unique_labels() {
        let labels = vec!["A", "A", "B"];
        DiGraph::empty(labels);
    }

    #[test]
    fn test_empty_labels() {
        let labels: Vec<String> = vec![];
        DiGraph::empty(labels);
    }
}

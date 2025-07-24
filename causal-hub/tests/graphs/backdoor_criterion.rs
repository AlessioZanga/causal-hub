#[cfg(test)]
mod tests {
    mod digraph {
        use causal_hub::{
            graphs::{BackdoorCriterion, DiGraph, Graph},
            types::Set,
        };

        // Tests for `is_backdoor_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_is_backdoor_set_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([5], [1], []);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_is_backdoor_set_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([0], [5], []);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn test_is_backdoor_set_out_of_bounds_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([0], [1], [5]);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_is_backdoor_set_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([], [1], []);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_is_backdoor_set_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([0], [], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_is_backdoor_set_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([0], [0], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn test_is_backdoor_set_non_disjoint_x_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([0], [1], [0]);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn test_is_backdoor_set_non_disjoint_y_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_backdoor_set([0], [1], [1]);
        }

        #[test]
        fn test_is_backdoor_set_edge() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);

            // Test for backdoor criterion.
            assert!(!graph.is_backdoor_set([0], [1], []));
            assert!(!graph.is_backdoor_set([1], [0], []));

            // Remove the edge and test again.
            graph.del_edge(0, 1);

            // Test for backdoor criterion after removing the edge.
            assert!(graph.is_backdoor_set([0], [1], []));
            assert!(graph.is_backdoor_set([1], [0], []));
        }

        #[test]
        fn test_is_backdoor_set_chain() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            // Test for backdoor criterion.
            assert!(graph.is_backdoor_set([0], [2], []));
            assert!(graph.is_backdoor_set([2], [0], [1]));
        }

        #[test]
        fn test_is_backdoor_set_fork() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            // Test for backdoor criterion.
            assert!(!graph.is_backdoor_set([1], [2], []));
            assert!(!graph.is_backdoor_set([2], [1], []));
            assert!(graph.is_backdoor_set([1], [2], [0]));
            assert!(graph.is_backdoor_set([2], [1], [0]));
        }

        #[test]
        fn test_is_backdoor_set_collider() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            // Test for backdoor criterion.
            assert!(graph.is_backdoor_set([1], [2], []));
            assert!(graph.is_backdoor_set([2], [1], []));
        }

        // TODO:

        // Test for `is_minimal_backdoor_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_is_minimal_backdoor_set_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([5], [1], [], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_is_minimal_backdoor_set_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([0], [5], [], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn test_is_minimal_backdoor_set_out_of_bounds_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([0], [1], [5], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_is_minimal_backdoor_set_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([], [1], [], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_is_minimal_backdoor_set_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([0], [], [], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_is_minimal_backdoor_set_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([0], [0], [], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn test_is_minimal_backdoor_set_non_disjoint_x_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([0], [1], [0], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn test_is_minimal_backdoor_set_non_disjoint_y_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_backdoor_set([0], [1], [1], None::<Set<_>>, None::<Set<_>>);
        }

        // TODO:

        // Test for `find_minimal_backdoor_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_find_minimal_backdoor_set_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_backdoor_set([5], [1], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_find_minimal_backdoor_set_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_backdoor_set([0], [5], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_find_minimal_backdoor_set_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_backdoor_set([], [1], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_find_minimal_backdoor_set_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_backdoor_set([0], [], None::<Set<_>>, None::<Set<_>>);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_find_minimal_backdoor_set_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_backdoor_set([0], [0], None::<Set<_>>, None::<Set<_>>);
        }

        // TODO:
    }
}

#[cfg(test)]
mod tests {
    mod digraph {
        use causal_hub::{
            assets::*,
            graphs::{DiGraph, Graph, GraphicalSeparation},
            models::BN,
            set,
            types::Set,
        };
        use dry::macro_for;
        use paste::paste;

        // Tests for `is_separator` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_is_separator_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([5], [1], []);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_is_separator_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([0], [5], []);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn test_is_separator_out_of_bounds_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([0], [1], [5]);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_is_separator_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([], [1], []);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_is_separator_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([0], [], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_is_separator_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([0], [0], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn test_is_separator_non_disjoint_x_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([0], [1], [0]);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn test_is_separator_non_disjoint_y_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator([0], [1], [1]);
        }

        #[test]
        fn test_is_separator_edge() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);

            // Test for d-separation.
            assert!(!graph.is_separator([0], [1], []));
            assert!(!graph.is_separator([1], [0], []));

            // Remove the edge and test again.
            graph.del_edge(0, 1);

            // Test for d-separation after removing the edge.
            assert!(graph.is_separator([0], [1], []));
            assert!(graph.is_separator([1], [0], []));
        }

        #[test]
        fn test_is_separator_chain() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            // Test for d-separation.
            assert!(!graph.is_separator([0], [2], []));
            assert!(!graph.is_separator([2], [0], []));
            assert!(graph.is_separator([0], [2], [1]));
            assert!(graph.is_separator([2], [0], [1]));
        }

        #[test]
        fn test_is_separator_fork() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            // Test for d-separation.
            assert!(!graph.is_separator([1], [2], []));
            assert!(!graph.is_separator([2], [1], []));
            assert!(graph.is_separator([1], [2], [0]));
            assert!(graph.is_separator([2], [1], [0]));
        }

        #[test]
        fn test_is_separator_collider() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            // Test for d-separation.
            assert!(graph.is_separator([1], [2], []));
            assert!(graph.is_separator([2], [1], []));
            assert!(!graph.is_separator([1], [2], [0]));
            assert!(!graph.is_separator([2], [1], [0]));
        }

        #[test]
        fn test_is_separator_primer_figure_2_7() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["U", "W", "X", "Y", "Z"]);
            // Add edges to the graph.
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            // Test for d-separation.
            assert!(graph.is_separator([3], [4], [])); // {Y} _||_ {Z} | {} ?
            assert!(!graph.is_separator([3], [4], [1])); // {Y} _|_ {Z} | {W} ?
            assert!(!graph.is_separator([3], [4], [0])); // {Y} _|_ {Z} | {U} ?
            assert!(graph.is_separator([3], [4], [1, 2])); // {Y} _||_ {Z} | {W, X} ?
            assert!(graph.is_separator([3], [4], [2, 1])); // {Y} _||_ {Z} | {X, W} ?
        }

        #[test]
        fn test_is_separator_primer_figure_2_8() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["T", "U", "W", "X", "Y", "Z"]);
            // Add edges to the graph.
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            // Test for d-separation.
            assert!(!graph.is_separator([4], [5], [])); // {Y} _||_ {Z} | {} ?
            assert!(!graph.is_separator([5], [4], [])); // {Z} _||_ {Y} | {} ?

            assert!(graph.is_separator([4], [5], [0])); // {Y} _||_ {Z} | {T} ?
            assert!(graph.is_separator([5], [4], [0])); // {Z} _||_ {Y} | {T} ?

            assert!(!graph.is_separator([4], [5], [0, 2])); // {Y} _||_ {Z} | {T, W} ?
            assert!(!graph.is_separator([5], [4], [0, 2])); // {Z} _||_ {Y} | {T, W} ?
            assert!(!graph.is_separator([4], [5], [2, 0])); // {Y} _||_ {Z} | {W, T} ?
            assert!(!graph.is_separator([5], [4], [2, 0])); // {Z} _||_ {Y} | {W, T} ?

            assert!(graph.is_separator([4], [5], [0, 2, 3])); // {Y} _||_ {Z} | {T, W, X} ?
            assert!(graph.is_separator([5], [4], [0, 2, 3])); // {Z} _||_ {Y} | {T, W, X} ?
            assert!(graph.is_separator([4], [5], [0, 3, 2])); // {Y} _||_ {Z} | {T, X, W} ?
            assert!(graph.is_separator([5], [4], [0, 3, 2])); // {Z} _||_ {Y} | {T, X, W} ?
            assert!(graph.is_separator([4], [5], [2, 0, 3])); // {Y} _||_ {Z} | {W, T, X} ?
            assert!(graph.is_separator([5], [4], [2, 0, 3])); // {Z} _||_ {Y} | {W, T, X} ?
            assert!(graph.is_separator([4], [5], [2, 3, 0])); // {Y} _||_ {Z} | {W, X, T} ?
            assert!(graph.is_separator([5], [4], [2, 3, 0])); // {Z} _||_ {Y} | {W, X, T} ?
            assert!(graph.is_separator([5], [4], [3, 0, 2])); // {Z} _||_ {Y} | {X, T, W} ?
            assert!(graph.is_separator([4], [5], [3, 0, 2])); // {Y} _||_ {Z} | {X, T, W} ?
            assert!(graph.is_separator([5], [4], [3, 2, 0])); // {Z} _||_ {Y} | {X, W, T} ?
            assert!(graph.is_separator([4], [5], [3, 2, 0])); // {Y} _||_ {Z} | {X, W, T} ?
        }

        macro_for!(
            $bn in [
                alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
                hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
                pigs, sachs, survey, water, win95pts
            ] {
            paste! {
                #[test]
                fn [<test_is_separator_ $bn>]() {
                    // Get the BN from the assets.
                    let bn = [<load_ $bn>]();
                    // Get the graph from the BN.
                    let g = bn.graph();
                    // Get the vertices of the graph.
                    let v: Set<_> = g.vertices();
                    // For each vertex ...
                    for &x in &v {
                        // Get the parents of the vertex.
                        let pa_x: Set<_> = g.parents(x).into_iter().collect();
                        // Get the descendants of the vertex.
                        let de_x = g.descendants(x);
                        // Get the non-descendants of the vertex: V - De(x) - Pa(x) - {x}.
                        let non_de_x = &v - &de_x;
                        let mut non_de_x = &non_de_x - &pa_x;
                        non_de_x.swap_remove(&x);
                        // Assert every vertex is d-separated from its non-descendants given its parents.
                        assert!(non_de_x.is_empty() || g.is_separator([x], non_de_x, pa_x));
                    }
                }
            }
        });

        // Test for `is_minimal_separator` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_is_minimal_separator_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([5], [1], []);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_is_minimal_separator_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([0], [5], []);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn test_is_minimal_separator_out_of_bounds_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([0], [1], [5]);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_is_minimal_separator_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([], [1], []);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_is_minimal_separator_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([0], [], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_is_minimal_separator_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([0], [0], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn test_is_minimal_separator_non_disjoint_x_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([0], [1], [0]);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn test_is_minimal_separator_non_disjoint_y_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator([0], [1], [1]);
        }

        #[test]
        fn test_is_minimal_separator_edge() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);

            // Test for d-separation.
            assert!(!graph.is_minimal_separator([0], [1], []));
            assert!(!graph.is_minimal_separator([1], [0], []));

            // Remove the edge and test again.
            graph.del_edge(0, 1);

            // Test for d-separation after removing the edge.
            assert!(graph.is_minimal_separator([0], [1], []));
            assert!(graph.is_minimal_separator([1], [0], []));
        }

        #[test]
        fn test_is_minimal_separator_chain() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            // Test for d-separation.
            assert!(!graph.is_minimal_separator([0], [2], []));
            assert!(!graph.is_minimal_separator([2], [0], []));
            assert!(graph.is_minimal_separator([0], [2], [1]));
            assert!(graph.is_minimal_separator([2], [0], [1]));
        }

        #[test]
        fn test_is_minimal_separator_fork() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            // Test for d-separation.
            assert!(!graph.is_minimal_separator([1], [2], []));
            assert!(!graph.is_minimal_separator([2], [1], []));
            assert!(graph.is_minimal_separator([1], [2], [0]));
            assert!(graph.is_minimal_separator([2], [1], [0]));
        }

        #[test]
        fn test_is_minimal_separator_collider() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            // Test for d-separation.
            assert!(graph.is_minimal_separator([1], [2], []));
            assert!(graph.is_minimal_separator([2], [1], []));
            assert!(!graph.is_minimal_separator([1], [2], [0]));
            assert!(!graph.is_minimal_separator([2], [1], [0]));
        }

        #[test]
        fn test_is_minimal_separator_primer_figure_2_7() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["U", "W", "X", "Y", "Z"]);
            // Add edges to the graph.
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            // Test for d-separation.
            assert!(graph.is_minimal_separator([3], [4], [])); // {Y} _||_ {Z} | {} ?
            assert!(!graph.is_minimal_separator([3], [4], [1])); // {Y} _|_ {Z} | {W} ?
            assert!(!graph.is_minimal_separator([3], [4], [0])); // {Y} _|_ {Z} | {U} ?
            assert!(!graph.is_minimal_separator([3], [4], [1, 2])); // {Y} _||_ {Z} | {W, X} ?
            assert!(!graph.is_minimal_separator([3], [4], [2, 1])); // {Y} _||_ {Z} | {X, W} ?
        }

        #[test]
        fn test_is_minimal_separator_primer_figure_2_8() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["T", "U", "W", "X", "Y", "Z"]);
            // Add edges to the graph.
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            // Test for d-separation.
            assert!(!graph.is_minimal_separator([4], [5], [])); // {Y} _||_ {Z} | {} ?
            assert!(!graph.is_minimal_separator([5], [4], [])); // {Z} _||_ {Y} | {} ?

            assert!(graph.is_minimal_separator([4], [5], [0])); // {Y} _||_ {Z} | {T} ?
            assert!(graph.is_minimal_separator([5], [4], [0])); // {Z} _||_ {Y} | {T} ?
        }

        // Test for `find_minimal_separator` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_find_minimal_separator_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator([5], [1]);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_find_minimal_separator_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator([0], [5]);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_find_minimal_separator_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator([], [1]);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_find_minimal_separator_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator([0], []);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_find_minimal_separator_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator([0], [0]);
        }

        #[test]
        fn test_find_minimal_separator_edge() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);

            // Test for d-separation.
            assert_eq!(graph.find_minimal_separator([0], [1]), None);
            assert_eq!(graph.find_minimal_separator([1], [0]), None);

            // Remove the edge and test again.
            graph.del_edge(0, 1);

            // Test for d-separation after removing the edge.
            assert_eq!(graph.find_minimal_separator([0], [1]), Some(set![]));
            assert_eq!(graph.find_minimal_separator([1], [0]), Some(set![]));
        }

        #[test]
        fn test_find_minimal_separator_chain() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            // Test for d-separation.
            assert_eq!(graph.find_minimal_separator([0], [2]), Some(set![1]));
            assert_eq!(graph.find_minimal_separator([2], [0]), Some(set![1]));
        }

        #[test]
        fn test_find_minimal_separator_fork() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            // Test for d-separation.
            assert_eq!(graph.find_minimal_separator([1], [2]), Some(set![0]));
            assert_eq!(graph.find_minimal_separator([2], [1]), Some(set![0]));
        }

        #[test]
        fn test_find_minimal_separator_collider() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            // Test for d-separation.
            assert_eq!(graph.find_minimal_separator([1], [2]), Some(set![]));
            assert_eq!(graph.find_minimal_separator([2], [1]), Some(set![]));
        }

        #[test]
        fn test_find_minimal_separator_primer_figure_2_7() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["U", "W", "X", "Y", "Z"]);
            // Add edges to the graph.
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            // Test for d-separation.
            assert_eq!(graph.find_minimal_separator([3], [4]), Some(set![])); // {Y} _||_ {Z} | {} ?
        }

        #[test]
        fn test_find_minimal_separator_primer_figure_2_8() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["T", "U", "W", "X", "Y", "Z"]);
            // Add edges to the graph.
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            // Test for d-separation.
            assert_eq!(graph.find_minimal_separator([4], [5]), Some(set![0])); // {Y} _||_ {Z} | {T} ?
            assert_eq!(graph.find_minimal_separator([5], [4]), Some(set![0])); // {Z} _||_ {Y} | {T} ?
        }
    }
}

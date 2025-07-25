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

        // Tests for `is_separator_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_is_separator_set_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![5], &set![1], &set![]);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_is_separator_set_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![0], &set![5], &set![]);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn test_is_separator_set_out_of_bounds_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![0], &set![1], &set![5]);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_is_separator_set_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![], &set![1], &set![]);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_is_separator_set_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![0], &set![], &set![]);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_is_separator_set_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![0], &set![0], &set![]);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn test_is_separator_set_non_disjoint_x_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![0], &set![1], &set![0]);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn test_is_separator_set_non_disjoint_y_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_separator_set(&set![0], &set![1], &set![1]);
        }

        #[test]
        fn test_is_separator_set_edge() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);

            assert!(!graph.is_separator_set(&set![0], &set![1], &set![]));
            assert!(!graph.is_separator_set(&set![1], &set![0], &set![]));

            // Remove the edge and test again.
            graph.del_edge(0, 1);

            assert!(graph.is_separator_set(&set![0], &set![1], &set![]));
            assert!(graph.is_separator_set(&set![1], &set![0], &set![]));
        }

        #[test]
        fn test_is_separator_set_chain() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            assert!(!graph.is_separator_set(&set![0], &set![2], &set![]));
            assert!(!graph.is_separator_set(&set![2], &set![0], &set![]));
            assert!(graph.is_separator_set(&set![0], &set![2], &set![1]));
            assert!(graph.is_separator_set(&set![2], &set![0], &set![1]));
        }

        #[test]
        fn test_is_separator_set_fork() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            assert!(!graph.is_separator_set(&set![1], &set![2], &set![]));
            assert!(!graph.is_separator_set(&set![2], &set![1], &set![]));
            assert!(graph.is_separator_set(&set![1], &set![2], &set![0]));
            assert!(graph.is_separator_set(&set![2], &set![1], &set![0]));
        }

        #[test]
        fn test_is_separator_set_collider() {
            // Initialize an empty graph.
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            // Add edges to the graph.
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            assert!(graph.is_separator_set(&set![1], &set![2], &set![]));
            assert!(graph.is_separator_set(&set![2], &set![1], &set![]));
            assert!(!graph.is_separator_set(&set![1], &set![2], &set![0]));
            assert!(!graph.is_separator_set(&set![2], &set![1], &set![0]));
        }

        #[test]
        fn test_is_separator_set_primer_figure_2_7() {
            let mut graph = DiGraph::empty(vec!["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            assert!(graph.is_separator_set(&set![3], &set![4], &set![]));
            assert!(!graph.is_separator_set(&set![3], &set![4], &set![1]));
            assert!(!graph.is_separator_set(&set![3], &set![4], &set![0]));
            assert!(graph.is_separator_set(&set![3], &set![4], &set![1, 2]));
            assert!(graph.is_separator_set(&set![3], &set![4], &set![2, 1]));
        }

        #[test]
        fn test_is_separator_set_primer_figure_2_8() {
            let mut graph = DiGraph::empty(vec!["T", "U", "W", "X", "Y", "Z"]);
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

            assert!(!graph.is_separator_set(&set![4], &set![5], &set![]));
            assert!(!graph.is_separator_set(&set![5], &set![4], &set![]));

            assert!(graph.is_separator_set(&set![4], &set![5], &set![0]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![0]));

            assert!(!graph.is_separator_set(&set![4], &set![5], &set![0, 2]));
            assert!(!graph.is_separator_set(&set![5], &set![4], &set![0, 2]));
            assert!(!graph.is_separator_set(&set![4], &set![5], &set![2, 0]));
            assert!(!graph.is_separator_set(&set![5], &set![4], &set![2, 0]));

            assert!(graph.is_separator_set(&set![4], &set![5], &set![0, 2, 3]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![0, 2, 3]));
            assert!(graph.is_separator_set(&set![4], &set![5], &set![0, 3, 2]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![0, 3, 2]));
            assert!(graph.is_separator_set(&set![4], &set![5], &set![2, 0, 3]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![2, 0, 3]));
            assert!(graph.is_separator_set(&set![4], &set![5], &set![2, 3, 0]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![2, 3, 0]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![3, 0, 2]));
            assert!(graph.is_separator_set(&set![4], &set![5], &set![3, 0, 2]));
            assert!(graph.is_separator_set(&set![5], &set![4], &set![3, 2, 0]));
            assert!(graph.is_separator_set(&set![4], &set![5], &set![3, 2, 0]));
        }

        macro_for!(
            $bn in [
                alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
                hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
                pigs, sachs, survey, water, win95pts
            ] {
            paste! {
                #[test]
                fn [<test_is_separator_set_ $bn>]() {
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
                        assert!(non_de_x.is_empty() || g.is_separator_set(&set![x], &non_de_x, &pa_x));
                    }
                }
            }
        });

        // Test for `is_minimal_separator_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_is_minimal_separator_set_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![5], &set![1], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_is_minimal_separator_set_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![0], &set![5], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn test_is_minimal_separator_set_out_of_bounds_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![0], &set![1], &set![5], None, None);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_is_minimal_separator_set_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![], &set![1], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_is_minimal_separator_set_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![0], &set![], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_is_minimal_separator_set_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![0], &set![0], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn test_is_minimal_separator_set_non_disjoint_x_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![0], &set![1], &set![0], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn test_is_minimal_separator_set_non_disjoint_y_z() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.is_minimal_separator_set(&set![0], &set![1], &set![1], None, None);
        }

        #[test]
        fn test_is_minimal_separator_set_edge() {
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            graph.add_edge(0, 1);

            assert!(!graph.is_minimal_separator_set(&set![0], &set![1], &set![], None, None));
            assert!(!graph.is_minimal_separator_set(&set![1], &set![0], &set![], None, None));

            graph.del_edge(0, 1);

            assert!(graph.is_minimal_separator_set(&set![0], &set![1], &set![], None, None));
            assert!(graph.is_minimal_separator_set(&set![1], &set![0], &set![], None, None));
        }

        #[test]
        fn test_is_minimal_separator_set_chain() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            assert!(!graph.is_minimal_separator_set(&set![0], &set![2], &set![], None, None));
            assert!(!graph.is_minimal_separator_set(&set![2], &set![0], &set![], None, None));
            assert!(graph.is_minimal_separator_set(&set![0], &set![2], &set![1], None, None));
            assert!(graph.is_minimal_separator_set(&set![2], &set![0], &set![1], None, None));
        }

        #[test]
        fn test_is_minimal_separator_set_fork() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            assert!(!graph.is_minimal_separator_set(&set![1], &set![2], &set![], None, None));
            assert!(!graph.is_minimal_separator_set(&set![2], &set![1], &set![], None, None));
            assert!(graph.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None));
            assert!(graph.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None));
        }

        #[test]
        fn test_is_minimal_separator_set_collider() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            assert!(graph.is_minimal_separator_set(&set![1], &set![2], &set![], None, None));
            assert!(graph.is_minimal_separator_set(&set![2], &set![1], &set![], None, None));
            assert!(!graph.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None));
            assert!(!graph.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None));
        }

        #[test]
        fn test_is_minimal_separator_set_primer_figure_2_7() {
            let mut graph = DiGraph::empty(vec!["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            assert!(graph.is_minimal_separator_set(&set![3], &set![4], &set![], None, None));
            assert!(!graph.is_minimal_separator_set(&set![3], &set![4], &set![1], None, None));
            assert!(!graph.is_minimal_separator_set(&set![3], &set![4], &set![0], None, None));
            assert!(!graph.is_minimal_separator_set(&set![3], &set![4], &set![1, 2], None, None));
            assert!(!graph.is_minimal_separator_set(&set![3], &set![4], &set![2, 1], None, None));
        }

        #[test]
        fn test_is_minimal_separator_set_primer_figure_2_8() {
            let mut graph = DiGraph::empty(vec!["T", "U", "W", "X", "Y", "Z"]);
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

            assert!(!graph.is_minimal_separator_set(&set![4], &set![5], &set![], None, None));
            assert!(!graph.is_minimal_separator_set(&set![5], &set![4], &set![], None, None));

            assert!(graph.is_minimal_separator_set(&set![4], &set![5], &set![0], None, None));
            assert!(graph.is_minimal_separator_set(&set![5], &set![4], &set![0], None, None));
        }

        // Test for `find_minimal_separator_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn test_find_minimal_separator_set_out_of_bounds_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator_set(&set![5], &set![1], None, None);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn test_find_minimal_separator_set_out_of_bounds_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator_set(&set![0], &set![5], None, None);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn test_find_minimal_separator_set_empty_x() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator_set(&set![], &set![1], None, None);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn test_find_minimal_separator_set_empty_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator_set(&set![0], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn test_find_minimal_separator_set_non_disjoint_x_y() {
            let graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.find_minimal_separator_set(&set![0], &set![0], None, None);
        }

        #[test]
        fn test_find_minimal_separator_set_edge() {
            let mut graph = DiGraph::empty(vec!["A", "B"]);
            graph.add_edge(0, 1);

            assert_eq!(
                graph.find_minimal_separator_set(&set![0], &set![1], None, None),
                None
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![0], None, None),
                None
            );

            graph.del_edge(0, 1);

            assert_eq!(
                graph.find_minimal_separator_set(&set![0], &set![1], None, None),
                Some(set![])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![0], None, None),
                Some(set![])
            );
        }

        #[test]
        fn test_find_minimal_separator_set_chain() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(0, 1);
            graph.add_edge(1, 2);

            assert_eq!(
                graph.find_minimal_separator_set(&set![0], &set![2], None, None),
                Some(set![1])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![2], &set![0], None, None),
                Some(set![1])
            );
        }

        #[test]
        fn test_find_minimal_separator_set_fork() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);

            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![2], None, None),
                Some(set![0])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![2], &set![1], None, None),
                Some(set![0])
            );
        }

        #[test]
        fn test_find_minimal_separator_set_collider() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C"]);
            graph.add_edge(1, 0);
            graph.add_edge(2, 0);

            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![2], None, None),
                Some(set![])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![2], &set![1], None, None),
                Some(set![])
            );
        }

        #[test]
        fn test_find_minimal_separator_set_primer_figure_2_7() {
            let mut graph = DiGraph::empty(vec!["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(&i), graph.label_to_index(&j));
            }

            assert_eq!(
                graph.find_minimal_separator_set(&set![3], &set![4], None, None),
                Some(set![])
            );
        }

        #[test]
        fn test_find_minimal_separator_set_primer_figure_2_8() {
            let mut graph = DiGraph::empty(vec!["T", "U", "W", "X", "Y", "Z"]);
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

            assert_eq!(
                graph.find_minimal_separator_set(&set![4], &set![5], None, None),
                Some(set![0])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![5], &set![4], None, None),
                Some(set![0])
            );
        }
    }
}

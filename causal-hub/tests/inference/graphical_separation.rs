#[cfg(test)]
mod tests {
    mod digraph {
        use causal_hub::{
            assets::*,
            inference::GraphicalSeparation,
            models::{BN, DiGraph, Graph, Labelled},
            set,
        };
        use dry::macro_for;
        use paste::paste;

        // Tests for `is_separator_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn is_separator_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![5], &set![1], &set![]);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn is_separator_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![0], &set![5], &set![]);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn is_separator_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![0], &set![1], &set![5]);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn is_separator_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![], &set![1], &set![]);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn is_separator_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![0], &set![], &set![]);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn is_separator_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![0], &set![0], &set![]);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn is_separator_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![0], &set![1], &set![0]);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn is_separator_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_separator_set(&set![0], &set![1], &set![1]);
        }

        #[test]
        fn is_separator_set_edge() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B"]);
            // Add edges to the g.
            g.add_edge(0, 1);

            assert!(!g.is_separator_set(&set![0], &set![1], &set![]));
            assert!(!g.is_separator_set(&set![1], &set![0], &set![]));

            // Remove the edge and test again.
            g.del_edge(0, 1);

            assert!(g.is_separator_set(&set![0], &set![1], &set![]));
            assert!(g.is_separator_set(&set![1], &set![0], &set![]));
        }

        #[test]
        fn is_separator_set_chain() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            assert!(!g.is_separator_set(&set![0], &set![2], &set![]));
            assert!(!g.is_separator_set(&set![2], &set![0], &set![]));
            assert!(g.is_separator_set(&set![0], &set![2], &set![1]));
            assert!(g.is_separator_set(&set![2], &set![0], &set![1]));
        }

        #[test]
        fn is_separator_set_fork() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert!(!g.is_separator_set(&set![1], &set![2], &set![]));
            assert!(!g.is_separator_set(&set![2], &set![1], &set![]));
            assert!(g.is_separator_set(&set![1], &set![2], &set![0]));
            assert!(g.is_separator_set(&set![2], &set![1], &set![0]));
        }

        #[test]
        fn is_separator_set_collider() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            assert!(g.is_separator_set(&set![1], &set![2], &set![]));
            assert!(g.is_separator_set(&set![2], &set![1], &set![]));
            assert!(!g.is_separator_set(&set![1], &set![2], &set![0]));
            assert!(!g.is_separator_set(&set![2], &set![1], &set![0]));
        }

        #[test]
        fn is_separator_set_primer_figure_2_7() {
            let mut g = DiGraph::empty(["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                g.add_edge(g.label_to_index(i), g.label_to_index(j));
            }

            assert!(g.is_separator_set(&set![3], &set![4], &set![]));
            assert!(!g.is_separator_set(&set![3], &set![4], &set![1]));
            assert!(!g.is_separator_set(&set![3], &set![4], &set![0]));
            assert!(g.is_separator_set(&set![3], &set![4], &set![1, 2]));
            assert!(g.is_separator_set(&set![3], &set![4], &set![2, 1]));
        }

        #[test]
        fn is_separator_set_primer_figure_2_8() {
            let mut g = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"]);
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                g.add_edge(g.label_to_index(i), g.label_to_index(j));
            }

            assert!(!g.is_separator_set(&set![4], &set![5], &set![]));
            assert!(!g.is_separator_set(&set![5], &set![4], &set![]));

            assert!(g.is_separator_set(&set![4], &set![5], &set![0]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![0]));

            assert!(!g.is_separator_set(&set![4], &set![5], &set![0, 2]));
            assert!(!g.is_separator_set(&set![5], &set![4], &set![0, 2]));
            assert!(!g.is_separator_set(&set![4], &set![5], &set![2, 0]));
            assert!(!g.is_separator_set(&set![5], &set![4], &set![2, 0]));

            assert!(g.is_separator_set(&set![4], &set![5], &set![0, 2, 3]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![0, 2, 3]));
            assert!(g.is_separator_set(&set![4], &set![5], &set![0, 3, 2]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![0, 3, 2]));
            assert!(g.is_separator_set(&set![4], &set![5], &set![2, 0, 3]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![2, 0, 3]));
            assert!(g.is_separator_set(&set![4], &set![5], &set![2, 3, 0]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![2, 3, 0]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![3, 0, 2]));
            assert!(g.is_separator_set(&set![4], &set![5], &set![3, 0, 2]));
            assert!(g.is_separator_set(&set![5], &set![4], &set![3, 2, 0]));
            assert!(g.is_separator_set(&set![4], &set![5], &set![3, 2, 0]));
        }

        macro_for!(
            $bn in [
                alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
                hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
                pigs, sachs, survey, water, win95pts
            ] {
            paste! {
                #[test]
                fn [<is_separator_set_ $bn>]() {
                    // Get the BN from the assets.
                    let bn = [<load_ $bn>]();
                    // Get the g from the BN.
                    let g = bn.graph();
                    // Get the vertices of the g.
                    let v = g.vertices();
                    // For each vertex ...
                    for &x in &v {
                        // Map to a set.
                        let x = set![x];
                        // Get the parents of the vertex.
                        let pa_x = g.parents(&x);
                        // Get the descendants of the vertex.
                        let de_x = g.descendants(&x);
                        // Get the non-descendants of the vertex: V - De(x) - Pa(x) - {x}.
                        let non_de_x = &(&(&v - &de_x) - &pa_x) - &x;
                        assert!(non_de_x.is_empty() || g.is_separator_set(&x, &non_de_x, &pa_x));
                    }
                }
            }
        });

        // Test for `is_minimal_separator_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn is_minimal_separator_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![5], &set![1], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn is_minimal_separator_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![0], &set![5], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn is_minimal_separator_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![0], &set![1], &set![5], None, None);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn is_minimal_separator_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![], &set![1], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn is_minimal_separator_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![0], &set![], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn is_minimal_separator_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![0], &set![0], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn is_minimal_separator_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![0], &set![1], &set![0], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn is_minimal_separator_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_separator_set(&set![0], &set![1], &set![1], None, None);
        }

        #[test]
        fn is_minimal_separator_set_edge() {
            let mut g = DiGraph::empty(["A", "B"]);
            g.add_edge(0, 1);

            assert!(!g.is_minimal_separator_set(&set![0], &set![1], &set![], None, None));
            assert!(!g.is_minimal_separator_set(&set![1], &set![0], &set![], None, None));

            g.del_edge(0, 1);

            assert!(g.is_minimal_separator_set(&set![0], &set![1], &set![], None, None));
            assert!(g.is_minimal_separator_set(&set![1], &set![0], &set![], None, None));
        }

        #[test]
        fn is_minimal_separator_set_chain() {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            assert!(!g.is_minimal_separator_set(&set![0], &set![2], &set![], None, None));
            assert!(!g.is_minimal_separator_set(&set![2], &set![0], &set![], None, None));
            assert!(g.is_minimal_separator_set(&set![0], &set![2], &set![1], None, None));
            assert!(g.is_minimal_separator_set(&set![2], &set![0], &set![1], None, None));
        }

        #[test]
        fn is_minimal_separator_set_fork() {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert!(!g.is_minimal_separator_set(&set![1], &set![2], &set![], None, None));
            assert!(!g.is_minimal_separator_set(&set![2], &set![1], &set![], None, None));
            assert!(g.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None));
            assert!(g.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None));
        }

        #[test]
        fn is_minimal_separator_set_collider() {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            assert!(g.is_minimal_separator_set(&set![1], &set![2], &set![], None, None));
            assert!(g.is_minimal_separator_set(&set![2], &set![1], &set![], None, None));
            assert!(!g.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None));
            assert!(!g.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None));
        }

        #[test]
        fn is_minimal_separator_set_primer_figure_2_7() {
            let mut g = DiGraph::empty(["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                g.add_edge(g.label_to_index(i), g.label_to_index(j));
            }

            assert!(g.is_minimal_separator_set(&set![3], &set![4], &set![], None, None));
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![1], None, None));
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![0], None, None));
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![1, 2], None, None));
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![2, 1], None, None));
        }

        #[test]
        fn is_minimal_separator_set_primer_figure_2_8() {
            let mut g = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"]);
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                g.add_edge(g.label_to_index(i), g.label_to_index(j));
            }

            assert!(!g.is_minimal_separator_set(&set![4], &set![5], &set![], None, None));
            assert!(!g.is_minimal_separator_set(&set![5], &set![4], &set![], None, None));

            assert!(g.is_minimal_separator_set(&set![4], &set![5], &set![0], None, None));
            assert!(g.is_minimal_separator_set(&set![5], &set![4], &set![0], None, None));
        }

        // Test for `find_minimal_separator_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn find_minimal_separator_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_separator_set(&set![5], &set![1], None, None);
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn find_minimal_separator_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_separator_set(&set![0], &set![5], None, None);
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn find_minimal_separator_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_separator_set(&set![], &set![1], None, None);
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn find_minimal_separator_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_separator_set(&set![0], &set![], None, None);
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn find_minimal_separator_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_separator_set(&set![0], &set![0], None, None);
        }

        #[test]
        fn find_minimal_separator_set_edge() {
            let mut g = DiGraph::empty(["A", "B"]);
            g.add_edge(0, 1);

            assert_eq!(
                g.find_minimal_separator_set(&set![0], &set![1], None, None),
                None
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![0], None, None),
                None
            );

            g.del_edge(0, 1);

            assert_eq!(
                g.find_minimal_separator_set(&set![0], &set![1], None, None),
                Some(set![])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![0], None, None),
                Some(set![])
            );
        }

        #[test]
        fn find_minimal_separator_set_chain() {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            assert_eq!(
                g.find_minimal_separator_set(&set![0], &set![2], None, None),
                Some(set![1])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![2], &set![0], None, None),
                Some(set![1])
            );
        }

        #[test]
        fn find_minimal_separator_set_fork() {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![2], None, None),
                Some(set![0])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![2], &set![1], None, None),
                Some(set![0])
            );
        }

        #[test]
        fn find_minimal_separator_set_collider() {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![2], None, None),
                Some(set![])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![2], &set![1], None, None),
                Some(set![])
            );
        }

        #[test]
        fn find_minimal_separator_set_primer_figure_2_7() {
            let mut g = DiGraph::empty(["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                g.add_edge(g.label_to_index(i), g.label_to_index(j));
            }

            assert_eq!(
                g.find_minimal_separator_set(&set![3], &set![4], None, None),
                Some(set![])
            );
        }

        #[test]
        fn find_minimal_separator_set_primer_figure_2_8() {
            let mut g = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"]);
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                g.add_edge(g.label_to_index(i), g.label_to_index(j));
            }

            assert_eq!(
                g.find_minimal_separator_set(&set![4], &set![5], None, None),
                Some(set![0])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![5], &set![4], None, None),
                Some(set![0])
            );
        }
    }
}

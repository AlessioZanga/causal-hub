#[cfg(test)]
mod tests {
    mod digraph {
        use causal_hub::{
            inference::BackdoorCriterion,
            models::{DiGraph, Graph, Labelled},
            set,
        };

        // Tests for `is_backdoor_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn is_backdoor_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![5], &set![1], &set![]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn is_backdoor_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![0], &set![5], &set![]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn is_backdoor_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![0], &set![1], &set![5]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn is_backdoor_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![], &set![1], &set![]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn is_backdoor_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![0], &set![], &set![]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn is_backdoor_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![0], &set![0], &set![]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn is_backdoor_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![0], &set![1], &set![0]).unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn is_backdoor_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_backdoor_set(&set![0], &set![1], &set![1]).unwrap();
        }

        #[test]
        fn is_backdoor_set_edge() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B"]);
            // Add edges to the g.
            g.add_edge(0, 1);

            // Test for backdoor criterion.
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![]).unwrap());
            assert!(!g.is_backdoor_set(&set![1], &set![0], &set![]).unwrap());

            // Remove the edge and test again.
            g.del_edge(0, 1);

            // Test for backdoor criterion after removing the edge.
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![]).unwrap());
            assert!(g.is_backdoor_set(&set![1], &set![0], &set![]).unwrap());
        }

        #[test]
        fn is_backdoor_set_chain() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            // Test for backdoor criterion.
            assert!(g.is_backdoor_set(&set![0], &set![2], &set![]).unwrap());
            assert!(g.is_backdoor_set(&set![2], &set![0], &set![1]).unwrap());
        }

        #[test]
        fn is_backdoor_set_fork() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            // Test for backdoor criterion.
            assert!(!g.is_backdoor_set(&set![1], &set![2], &set![]).unwrap());
            assert!(!g.is_backdoor_set(&set![2], &set![1], &set![]).unwrap());
            assert!(g.is_backdoor_set(&set![1], &set![2], &set![0]).unwrap());
            assert!(g.is_backdoor_set(&set![2], &set![1], &set![0]).unwrap());
        }

        #[test]
        fn is_backdoor_set_collider() {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            // Test for backdoor criterion.
            assert!(g.is_backdoor_set(&set![1], &set![2], &set![]).unwrap());
            assert!(g.is_backdoor_set(&set![2], &set![1], &set![]).unwrap());
        }

        #[test]
        fn is_backdoor_set_primer_figure_3_7() {
            let mut g = DiGraph::empty(["A", "E", "X", "Y", "Z"]);
            for (i, j) in [
                ("A", "Y"),
                ("A", "Z"),
                ("E", "X"),
                ("E", "Z"),
                ("X", "Y"),
                ("Z", "X"),
                ("Z", "Y"),
            ] {
                g.add_edge(g.label_to_index(i).unwrap(), g.label_to_index(j).unwrap());
            }

            assert!(!g.is_backdoor_set(&set![2], &set![3], &set![]).unwrap());
            assert!(!g.is_backdoor_set(&set![2], &set![3], &set![4]).unwrap());
            assert!(g.is_backdoor_set(&set![2], &set![3], &set![0, 4]).unwrap());
            assert!(g.is_backdoor_set(&set![2], &set![3], &set![1, 4]).unwrap());
            assert!(
                g.is_backdoor_set(&set![2], &set![3], &set![0, 1, 4])
                    .unwrap()
            );
        }

        // Test for `is_minimal_backdoor_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn is_minimal_backdoor_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![5], &set![1], &set![], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn is_minimal_backdoor_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![0], &set![5], &set![], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Z is out of bounds.")]
        fn is_minimal_backdoor_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![0], &set![1], &set![5], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn is_minimal_backdoor_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![], &set![1], &set![], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn is_minimal_backdoor_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![0], &set![], &set![], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn is_minimal_backdoor_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![0], &set![0], &set![], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets X and Z must be disjoint.")]
        fn is_minimal_backdoor_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![0], &set![1], &set![0], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets Y and Z must be disjoint.")]
        fn is_minimal_backdoor_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.is_minimal_backdoor_set(&set![0], &set![1], &set![1], None, None)
                .unwrap();
        }

        #[test]
        fn is_minimal_backdoor_set_primer_figure_3_7() {
            let mut g = DiGraph::empty(["A", "E", "X", "Y", "Z"]);
            for (i, j) in [
                ("A", "Y"),
                ("A", "Z"),
                ("E", "X"),
                ("E", "Z"),
                ("X", "Y"),
                ("Z", "X"),
                ("Z", "Y"),
            ] {
                g.add_edge(g.label_to_index(i).unwrap(), g.label_to_index(j).unwrap());
            }

            assert!(
                !g.is_minimal_backdoor_set(&set![2], &set![3], &set![], None, None)
                    .unwrap()
            );
            assert!(
                !g.is_minimal_backdoor_set(&set![2], &set![3], &set![4], None, None)
                    .unwrap()
            );
            assert!(
                g.is_minimal_backdoor_set(&set![2], &set![3], &set![0, 4], None, None)
                    .unwrap()
            );
            assert!(
                g.is_minimal_backdoor_set(&set![2], &set![3], &set![1, 4], None, None)
                    .unwrap()
            );
            assert!(
                !g.is_minimal_backdoor_set(&set![2], &set![3], &set![0, 1, 4], None, None)
                    .unwrap()
            );
            assert!(
                g.is_minimal_backdoor_set(
                    &set![2],
                    &set![3],
                    &set![0, 1, 4],
                    Some(&set![0, 1]),
                    None
                )
                .unwrap()
            );
        }

        // Test for `find_minimal_backdoor_set` method.

        #[test]
        #[should_panic(expected = "Vertex `5` in set X is out of bounds.")]
        fn find_minimal_backdoor_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_backdoor_set(&set![5], &set![1], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Vertex `5` in set Y is out of bounds.")]
        fn find_minimal_backdoor_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_backdoor_set(&set![0], &set![5], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Set X must not be empty.")]
        fn find_minimal_backdoor_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_backdoor_set(&set![], &set![1], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Set Y must not be empty.")]
        fn find_minimal_backdoor_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_backdoor_set(&set![0], &set![], None, None)
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Sets X and Y must be disjoint.")]
        fn find_minimal_backdoor_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            g.find_minimal_backdoor_set(&set![0], &set![0], None, None)
                .unwrap();
        }

        #[test]
        fn find_minimal_backdoor_set_primer_figure_3_7() {
            let mut g = DiGraph::empty(["A", "E", "X", "Y", "Z"]);
            for (i, j) in [
                ("A", "Y"),
                ("A", "Z"),
                ("E", "X"),
                ("E", "Z"),
                ("X", "Y"),
                ("Z", "X"),
                ("Z", "Y"),
            ] {
                g.add_edge(g.label_to_index(i).unwrap(), g.label_to_index(j).unwrap());
            }

            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], None, Some(&set![0, 1]))
                    .unwrap(),
                None
            );
            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], Some(&set![0]), None)
                    .unwrap(),
                Some(set![0, 4])
            );
            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], None, None)
                    .unwrap(),
                Some(set![1, 4])
            );
            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], Some(&set![0, 1]), None)
                    .unwrap(),
                Some(set![0, 1, 4])
            );
        }
    }
}

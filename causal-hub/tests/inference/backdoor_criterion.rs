#[cfg(test)]
mod tests {
    mod digraph {
        use causal_hub::{
            inference::BackdoorCriterion,
            models::{DiGraph, Graph, Labelled},
            set,
            types::Result,
        };

        // Tests for `is_backdoor_set` method.

        #[test]
        fn is_backdoor_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![5], &set![1], &set![]).is_err());
        }

        #[test]
        fn is_backdoor_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![0], &set![5], &set![]).is_err());
        }

        #[test]
        fn is_backdoor_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![5]).is_err());
        }

        #[test]
        fn is_backdoor_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![], &set![1], &set![]).is_err());
        }

        #[test]
        fn is_backdoor_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![0], &set![], &set![]).is_err());
        }

        #[test]
        fn is_backdoor_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![0], &set![0], &set![]).is_err());
        }

        #[test]
        fn is_backdoor_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![0]).is_err());
        }

        #[test]
        fn is_backdoor_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![1]).is_err());
        }

        #[test]
        fn is_backdoor_set_edge() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B"]);
            // Add edges to the g.
            g.add_edge(0, 1);

            // Test for backdoor criterion.
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![])?);
            assert!(!g.is_backdoor_set(&set![1], &set![0], &set![])?);

            // Remove the edge and test again.
            g.del_edge(0, 1);

            // Test for backdoor criterion after removing the edge.
            assert!(g.is_backdoor_set(&set![0], &set![1], &set![])?);
            assert!(g.is_backdoor_set(&set![1], &set![0], &set![])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_chain() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            // Test for backdoor criterion.
            assert!(g.is_backdoor_set(&set![0], &set![2], &set![])?);
            assert!(g.is_backdoor_set(&set![2], &set![0], &set![1])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_fork() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            // Test for backdoor criterion.
            assert!(!g.is_backdoor_set(&set![1], &set![2], &set![])?);
            assert!(!g.is_backdoor_set(&set![2], &set![1], &set![])?);
            assert!(g.is_backdoor_set(&set![1], &set![2], &set![0])?);
            assert!(g.is_backdoor_set(&set![2], &set![1], &set![0])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_collider() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            // Test for backdoor criterion.
            assert!(g.is_backdoor_set(&set![1], &set![2], &set![])?);
            assert!(g.is_backdoor_set(&set![2], &set![1], &set![])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_primer_figure_3_7() -> Result<()> {
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
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert!(!g.is_backdoor_set(&set![2], &set![3], &set![])?);
            assert!(!g.is_backdoor_set(&set![2], &set![3], &set![4])?);
            assert!(g.is_backdoor_set(&set![2], &set![3], &set![0, 4])?);
            assert!(g.is_backdoor_set(&set![2], &set![3], &set![1, 4])?);
            assert!(g.is_backdoor_set(&set![2], &set![3], &set![0, 1, 4])?);

            Ok(())
        }

        // Test for `is_minimal_backdoor_set` method.

        #[test]
        fn is_minimal_backdoor_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![5], &set![1], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![0], &set![5], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![0], &set![1], &set![5], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![], &set![1], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![0], &set![], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![0], &set![0], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![0], &set![1], &set![0], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_backdoor_set(&set![0], &set![1], &set![1], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_backdoor_set_primer_figure_3_7() -> Result<()> {
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
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert!(!g.is_minimal_backdoor_set(&set![2], &set![3], &set![], None, None)?);
            assert!(!g.is_minimal_backdoor_set(&set![2], &set![3], &set![4], None, None)?);
            assert!(g.is_minimal_backdoor_set(&set![2], &set![3], &set![0, 4], None, None)?);
            assert!(g.is_minimal_backdoor_set(&set![2], &set![3], &set![1, 4], None, None)?);
            assert!(!g.is_minimal_backdoor_set(&set![2], &set![3], &set![0, 1, 4], None, None)?);
            assert!(g.is_minimal_backdoor_set(
                &set![2],
                &set![3],
                &set![0, 1, 4],
                Some(&set![0, 1]),
                None
            )?);

            Ok(())
        }

        // Test for `find_minimal_backdoor_set` method.

        #[test]
        fn find_minimal_backdoor_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_backdoor_set(&set![5], &set![1], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_backdoor_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_backdoor_set(&set![0], &set![5], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_backdoor_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_backdoor_set(&set![], &set![1], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_backdoor_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_backdoor_set(&set![0], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_backdoor_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_backdoor_set(&set![0], &set![0], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_backdoor_set_primer_figure_3_7() -> Result<()> {
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
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], None, Some(&set![0, 1]))?,
                None
            );
            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], Some(&set![0]), None)?,
                Some(set![0, 4])
            );
            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], None, None)?,
                Some(set![1, 4])
            );
            assert_eq!(
                g.find_minimal_backdoor_set(&set![2], &set![3], Some(&set![0, 1]), None)?,
                Some(set![0, 1, 4])
            );

            Ok(())
        }
    }
}

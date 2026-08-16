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
        fn is_backdoor_set_out_of_bounds_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![5], &set![1], &set![]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_out_of_bounds_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![0], &set![5], &set![]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_out_of_bounds_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![0], &set![1], &set![5]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_empty_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![], &set![1], &set![]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_empty_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![0], &set![], &set![]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_non_disjoint_x_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![0], &set![0], &set![]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_non_disjoint_x_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![0], &set![1], &set![0]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_non_disjoint_y_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_backdoor_set(&set![0], &set![1], &set![1]).is_err());

            Ok(())
        }

        #[test]
        fn is_backdoor_set_edge() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B"])?;
            // Add edges to the g.
            graph.add_edge(0, 1)?;

            // Test for backdoor criterion.
            assert!(graph.is_backdoor_set(&set![0], &set![1], &set![])?);
            assert!(!graph.is_backdoor_set(&set![1], &set![0], &set![])?);

            // Remove the edge and test again.
            graph.del_edge(0, 1)?;

            // Test for backdoor criterion after removing the edge.
            assert!(graph.is_backdoor_set(&set![0], &set![1], &set![])?);
            assert!(graph.is_backdoor_set(&set![1], &set![0], &set![])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_chain() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            // Add edges to the g.
            graph.add_edge(0, 1)?;
            graph.add_edge(1, 2)?;

            // Test for backdoor criterion.
            assert!(graph.is_backdoor_set(&set![0], &set![2], &set![])?);
            assert!(graph.is_backdoor_set(&set![2], &set![0], &set![1])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_fork() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            // Add edges to the g.
            graph.add_edge(0, 1)?;
            graph.add_edge(0, 2)?;

            // Test for backdoor criterion.
            assert!(!graph.is_backdoor_set(&set![1], &set![2], &set![])?);
            assert!(!graph.is_backdoor_set(&set![2], &set![1], &set![])?);
            assert!(graph.is_backdoor_set(&set![1], &set![2], &set![0])?);
            assert!(graph.is_backdoor_set(&set![2], &set![1], &set![0])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_collider() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            // Add edges to the g.
            graph.add_edge(1, 0)?;
            graph.add_edge(2, 0)?;

            // Test for backdoor criterion.
            assert!(graph.is_backdoor_set(&set![1], &set![2], &set![])?);
            assert!(graph.is_backdoor_set(&set![2], &set![1], &set![])?);

            Ok(())
        }

        #[test]
        fn is_backdoor_set_primer_figure_3_7() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "E", "X", "Y", "Z"])?;
            for (i, j) in [
                ("A", "Y"),
                ("A", "Z"),
                ("E", "X"),
                ("E", "Z"),
                ("X", "Y"),
                ("Z", "X"),
                ("Z", "Y"),
            ] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert!(!graph.is_backdoor_set(&set![2], &set![3], &set![])?);
            assert!(!graph.is_backdoor_set(&set![2], &set![3], &set![4])?);
            assert!(graph.is_backdoor_set(&set![2], &set![3], &set![0, 4])?);
            assert!(graph.is_backdoor_set(&set![2], &set![3], &set![1, 4])?);
            assert!(graph.is_backdoor_set(&set![2], &set![3], &set![0, 1, 4])?);

            Ok(())
        }

        // Test for `is_minimal_backdoor_set` method.

        #[test]
        fn is_minimal_backdoor_set_out_of_bounds_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![5], &set![1], &set![], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_out_of_bounds_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![0], &set![5], &set![], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_out_of_bounds_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![0], &set![1], &set![5], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_empty_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![], &set![1], &set![], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_empty_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![0], &set![], &set![], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_non_disjoint_x_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![0], &set![0], &set![], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_non_disjoint_x_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![0], &set![1], &set![0], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_non_disjoint_y_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_backdoor_set(&set![0], &set![1], &set![1], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn is_minimal_backdoor_set_primer_figure_3_7() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "E", "X", "Y", "Z"])?;
            for (i, j) in [
                ("A", "Y"),
                ("A", "Z"),
                ("E", "X"),
                ("E", "Z"),
                ("X", "Y"),
                ("Z", "X"),
                ("Z", "Y"),
            ] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert!(!graph.is_minimal_backdoor_set(&set![2], &set![3], &set![], None, None)?);
            assert!(!graph.is_minimal_backdoor_set(&set![2], &set![3], &set![4], None, None)?);
            assert!(graph.is_minimal_backdoor_set(&set![2], &set![3], &set![0, 4], None, None)?);
            assert!(graph.is_minimal_backdoor_set(&set![2], &set![3], &set![1, 4], None, None)?);
            assert!(!graph.is_minimal_backdoor_set(
                &set![2],
                &set![3],
                &set![0, 1, 4],
                None,
                None
            )?);
            assert!(graph.is_minimal_backdoor_set(
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
        fn find_minimal_backdoor_set_out_of_bounds_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_backdoor_set(&set![5], &set![1], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn find_minimal_backdoor_set_out_of_bounds_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_backdoor_set(&set![0], &set![5], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn find_minimal_backdoor_set_empty_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_backdoor_set(&set![], &set![1], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn find_minimal_backdoor_set_empty_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_backdoor_set(&set![0], &set![], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn find_minimal_backdoor_set_non_disjoint_x_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_backdoor_set(&set![0], &set![0], None, None)
                    .is_err()
            );

            Ok(())
        }

        #[test]
        fn find_minimal_backdoor_set_primer_figure_3_7() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "E", "X", "Y", "Z"])?;
            for (i, j) in [
                ("A", "Y"),
                ("A", "Z"),
                ("E", "X"),
                ("E", "Z"),
                ("X", "Y"),
                ("Z", "X"),
                ("Z", "Y"),
            ] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert_eq!(
                graph.find_minimal_backdoor_set(&set![2], &set![3], None, Some(&set![0, 1]))?,
                None
            );
            assert_eq!(
                graph.find_minimal_backdoor_set(&set![2], &set![3], Some(&set![0]), None)?,
                Some(set![0, 4])
            );
            assert_eq!(
                graph.find_minimal_backdoor_set(&set![2], &set![3], None, None)?,
                Some(set![1, 4])
            );
            assert_eq!(
                graph.find_minimal_backdoor_set(&set![2], &set![3], Some(&set![0, 1]), None)?,
                Some(set![0, 1, 4])
            );

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::*,
        inference::GraphicalSeparation,
        models::{BN, DiGraph, Graph, HasLabels},
        set,
        types::Result,
    };
    use dry::macro_for;
    use paste::paste;

    mod digraph {
        use super::*;

        // Tests for `is_separator_set` method.

        #[test]
        fn is_separator_set_out_of_bounds_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_separator_set(&set![5], &set![1], &set![]).is_err());
            Ok(())
        }

        #[test]
        fn is_separator_set_out_of_bounds_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_separator_set(&set![0], &set![5], &set![]).is_err());
            Ok(())
        }

        #[test]
        fn is_separator_set_out_of_bounds_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_separator_set(&set![0], &set![1], &set![5])
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_separator_set_empty_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_separator_set(&set![], &set![1], &set![]).is_err());
            Ok(())
        }

        #[test]
        fn is_separator_set_empty_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_separator_set(&set![0], &set![], &set![]).is_err());
            Ok(())
        }

        #[test]
        fn is_separator_set_non_disjoint_x_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(graph.is_separator_set(&set![0], &set![0], &set![]).is_err());
            Ok(())
        }

        #[test]
        fn is_separator_set_non_disjoint_x_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_separator_set(&set![0], &set![1], &set![0])
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_separator_set_non_disjoint_y_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_separator_set(&set![0], &set![1], &set![1])
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_separator_set_edge() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B"])?;
            // Add edges to the g.
            graph.add_edge(0, 1)?;

            assert!(!graph.is_separator_set(&set![0], &set![1], &set![])?);
            assert!(!graph.is_separator_set(&set![1], &set![0], &set![])?);

            // Remove the edge and test again.
            graph.del_edge(0, 1)?;

            assert!(graph.is_separator_set(&set![0], &set![1], &set![])?);
            assert!(graph.is_separator_set(&set![1], &set![0], &set![])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_chain() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            // Add edges to the g.
            graph.add_edge(0, 1)?;
            graph.add_edge(1, 2)?;

            assert!(!graph.is_separator_set(&set![0], &set![2], &set![])?);
            assert!(!graph.is_separator_set(&set![2], &set![0], &set![])?);
            assert!(graph.is_separator_set(&set![0], &set![2], &set![1])?);
            assert!(graph.is_separator_set(&set![2], &set![0], &set![1])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_fork() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            // Add edges to the g.
            graph.add_edge(0, 1)?;
            graph.add_edge(0, 2)?;

            assert!(!graph.is_separator_set(&set![1], &set![2], &set![])?);
            assert!(!graph.is_separator_set(&set![2], &set![1], &set![])?);
            assert!(graph.is_separator_set(&set![1], &set![2], &set![0])?);
            assert!(graph.is_separator_set(&set![2], &set![1], &set![0])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_collider() -> Result<()> {
            // Initialize an empty g.
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            // Add edges to the g.
            graph.add_edge(1, 0)?;
            graph.add_edge(2, 0)?;

            assert!(graph.is_separator_set(&set![1], &set![2], &set![])?);
            assert!(graph.is_separator_set(&set![2], &set![1], &set![])?);
            assert!(!graph.is_separator_set(&set![1], &set![2], &set![0])?);
            assert!(!graph.is_separator_set(&set![2], &set![1], &set![0])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_primer_figure_2_7() -> Result<()> {
            let mut graph = DiGraph::empty(["U", "W", "X", "Y", "Z"])?;
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert!(graph.is_separator_set(&set![3], &set![4], &set![])?);
            assert!(!graph.is_separator_set(&set![3], &set![4], &set![1])?);
            assert!(!graph.is_separator_set(&set![3], &set![4], &set![0])?);
            assert!(graph.is_separator_set(&set![3], &set![4], &set![1, 2])?);
            assert!(graph.is_separator_set(&set![3], &set![4], &set![2, 1])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_primer_figure_2_8() -> Result<()> {
            let mut graph = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"])?;
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert!(!graph.is_separator_set(&set![4], &set![5], &set![])?);
            assert!(!graph.is_separator_set(&set![5], &set![4], &set![])?);

            assert!(graph.is_separator_set(&set![4], &set![5], &set![0])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![0])?);

            assert!(!graph.is_separator_set(&set![4], &set![5], &set![0, 2])?);
            assert!(!graph.is_separator_set(&set![5], &set![4], &set![0, 2])?);
            assert!(!graph.is_separator_set(&set![4], &set![5], &set![2, 0])?);
            assert!(!graph.is_separator_set(&set![5], &set![4], &set![2, 0])?);

            assert!(graph.is_separator_set(&set![4], &set![5], &set![0, 2, 3])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![0, 2, 3])?);
            assert!(graph.is_separator_set(&set![4], &set![5], &set![0, 3, 2])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![0, 3, 2])?);
            assert!(graph.is_separator_set(&set![4], &set![5], &set![2, 0, 3])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![2, 0, 3])?);
            assert!(graph.is_separator_set(&set![4], &set![5], &set![2, 3, 0])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![2, 3, 0])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![3, 0, 2])?);
            assert!(graph.is_separator_set(&set![4], &set![5], &set![3, 0, 2])?);
            assert!(graph.is_separator_set(&set![5], &set![4], &set![3, 2, 0])?);
            assert!(graph.is_separator_set(&set![4], &set![5], &set![3, 2, 0])?);

            Ok(())
        }

        macro_for!(
            $bn in [
                alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
                hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
                pigs, sachs, survey, water, win95pts
            ] {
            paste! {
                #[test]
                fn [<is_separator_set_ $bn>]() -> Result<()> {
                    // Get the BN from the assets.
                    let model = [<load_ $bn>]()?;
                    // Get the graph from the BN.
                    let graph = model.graph();
                    // Get the vertices of the graph.
                    let v = graph.vertices();
                    // For each vertex ...
                    for &x in &v {
                        // Map to a set.
                        let x = set![x];
                        // Get the parents of the vertex.
                        let pa_x = graph.parents(&x)?;
                        // Get the descendants of the vertex.
                        let de_x = graph.descendants(&x)?;
                        // Get the non-descendants of the vertex: V - De(x) - Pa(x) - {x}.
                        let non_de_x = &(&(&v - &de_x) - &pa_x) - &x;
                        assert!(non_de_x.is_empty() || graph.is_separator_set(&x, &non_de_x, &pa_x)?);
                    }

                    Ok(())
                }
            }
        });

        // Test for `is_minimal_separator_set` method.

        #[test]
        fn is_minimal_separator_set_out_of_bounds_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![5], &set![1], &set![], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_out_of_bounds_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![0], &set![5], &set![], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_out_of_bounds_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![0], &set![1], &set![5], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_empty_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![], &set![1], &set![], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_empty_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![0], &set![], &set![], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_non_disjoint_x_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![0], &set![0], &set![], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_non_disjoint_x_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![0], &set![1], &set![0], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_non_disjoint_y_z() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .is_minimal_separator_set(&set![0], &set![1], &set![1], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_edge() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B"])?;
            graph.add_edge(0, 1)?;

            assert!(!graph.is_minimal_separator_set(&set![0], &set![1], &set![], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![1], &set![0], &set![], None, None)?);

            graph.del_edge(0, 1)?;

            assert!(graph.is_minimal_separator_set(&set![0], &set![1], &set![], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![1], &set![0], &set![], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_chain() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            graph.add_edge(0, 1)?;
            graph.add_edge(1, 2)?;

            assert!(!graph.is_minimal_separator_set(&set![0], &set![2], &set![], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![2], &set![0], &set![], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![0], &set![2], &set![1], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![2], &set![0], &set![1], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_fork() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            graph.add_edge(0, 1)?;
            graph.add_edge(0, 2)?;

            assert!(!graph.is_minimal_separator_set(&set![1], &set![2], &set![], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![2], &set![1], &set![], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_collider() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            graph.add_edge(1, 0)?;
            graph.add_edge(2, 0)?;

            assert!(graph.is_minimal_separator_set(&set![1], &set![2], &set![], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![2], &set![1], &set![], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_primer_figure_2_7() -> Result<()> {
            let mut graph = DiGraph::empty(["U", "W", "X", "Y", "Z"])?;
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert!(graph.is_minimal_separator_set(&set![3], &set![4], &set![], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![3], &set![4], &set![1], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![3], &set![4], &set![0], None, None)?);
            assert!(!graph.is_minimal_separator_set(
                &set![3],
                &set![4],
                &set![1, 2],
                None,
                None
            )?);
            assert!(!graph.is_minimal_separator_set(
                &set![3],
                &set![4],
                &set![2, 1],
                None,
                None
            )?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_primer_figure_2_8() -> Result<()> {
            let mut graph = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"])?;
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert!(!graph.is_minimal_separator_set(&set![4], &set![5], &set![], None, None)?);
            assert!(!graph.is_minimal_separator_set(&set![5], &set![4], &set![], None, None)?);

            assert!(graph.is_minimal_separator_set(&set![4], &set![5], &set![0], None, None)?);
            assert!(graph.is_minimal_separator_set(&set![5], &set![4], &set![0], None, None)?);

            Ok(())
        }

        // Test for `find_minimal_separator_set` method.

        #[test]
        fn find_minimal_separator_set_out_of_bounds_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_separator_set(&set![5], &set![1], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_out_of_bounds_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_separator_set(&set![0], &set![5], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_empty_x() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_separator_set(&set![], &set![1], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_empty_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_separator_set(&set![0], &set![], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_non_disjoint_x_y() -> Result<()> {
            let graph = DiGraph::empty(["A", "B", "C"])?;
            assert!(
                graph
                    .find_minimal_separator_set(&set![0], &set![0], None, None)
                    .is_err()
            );
            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_edge() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B"])?;
            graph.add_edge(0, 1)?;

            assert_eq!(
                graph.find_minimal_separator_set(&set![0], &set![1], None, None)?,
                None
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![0], None, None)?,
                None
            );

            graph.del_edge(0, 1)?;

            assert_eq!(
                graph.find_minimal_separator_set(&set![0], &set![1], None, None)?,
                Some(set![])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![0], None, None)?,
                Some(set![])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_chain() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            graph.add_edge(0, 1)?;
            graph.add_edge(1, 2)?;

            assert_eq!(
                graph.find_minimal_separator_set(&set![0], &set![2], None, None)?,
                Some(set![1])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![2], &set![0], None, None)?,
                Some(set![1])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_fork() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            graph.add_edge(0, 1)?;
            graph.add_edge(0, 2)?;

            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![2], None, None)?,
                Some(set![0])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![2], &set![1], None, None)?,
                Some(set![0])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_collider() -> Result<()> {
            let mut graph = DiGraph::empty(["A", "B", "C"])?;
            graph.add_edge(1, 0)?;
            graph.add_edge(2, 0)?;

            assert_eq!(
                graph.find_minimal_separator_set(&set![1], &set![2], None, None)?,
                Some(set![])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![2], &set![1], None, None)?,
                Some(set![])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_primer_figure_2_7() -> Result<()> {
            let mut graph = DiGraph::empty(["U", "W", "X", "Y", "Z"])?;
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert_eq!(
                graph.find_minimal_separator_set(&set![3], &set![4], None, None)?,
                Some(set![])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_primer_figure_2_8() -> Result<()> {
            let mut graph = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"])?;
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                graph.add_edge(graph.label_to_index(i)?, graph.label_to_index(j)?)?;
            }

            assert_eq!(
                graph.find_minimal_separator_set(&set![4], &set![5], None, None)?,
                Some(set![0])
            );
            assert_eq!(
                graph.find_minimal_separator_set(&set![5], &set![4], None, None)?,
                Some(set![0])
            );

            Ok(())
        }
    }
}

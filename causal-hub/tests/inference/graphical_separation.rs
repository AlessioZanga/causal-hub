#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::*,
        inference::GraphicalSeparation,
        models::{BN, DiGraph, Graph, Labelled},
        set,
        types::Result,
    };
    use dry::macro_for;
    use paste::paste;

    mod digraph {
        use super::*;

        // Tests for `is_separator_set` method.

        #[test]
        fn is_separator_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![5], &set![1], &set![]).is_err());
        }

        #[test]
        fn is_separator_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![0], &set![5], &set![]).is_err());
        }

        #[test]
        fn is_separator_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![0], &set![1], &set![5]).is_err());
        }

        #[test]
        fn is_separator_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![], &set![1], &set![]).is_err());
        }

        #[test]
        fn is_separator_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![0], &set![], &set![]).is_err());
        }

        #[test]
        fn is_separator_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![0], &set![0], &set![]).is_err());
        }

        #[test]
        fn is_separator_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![0], &set![1], &set![0]).is_err());
        }

        #[test]
        fn is_separator_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(g.is_separator_set(&set![0], &set![1], &set![1]).is_err());
        }

        #[test]
        fn is_separator_set_edge() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B"]);
            // Add edges to the g.
            g.add_edge(0, 1);

            assert!(!g.is_separator_set(&set![0], &set![1], &set![])?);
            assert!(!g.is_separator_set(&set![1], &set![0], &set![])?);

            // Remove the edge and test again.
            g.del_edge(0, 1);

            assert!(g.is_separator_set(&set![0], &set![1], &set![])?);
            assert!(g.is_separator_set(&set![1], &set![0], &set![])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_chain() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            assert!(!g.is_separator_set(&set![0], &set![2], &set![])?);
            assert!(!g.is_separator_set(&set![2], &set![0], &set![])?);
            assert!(g.is_separator_set(&set![0], &set![2], &set![1])?);
            assert!(g.is_separator_set(&set![2], &set![0], &set![1])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_fork() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert!(!g.is_separator_set(&set![1], &set![2], &set![])?);
            assert!(!g.is_separator_set(&set![2], &set![1], &set![])?);
            assert!(g.is_separator_set(&set![1], &set![2], &set![0])?);
            assert!(g.is_separator_set(&set![2], &set![1], &set![0])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_collider() -> Result<()> {
            // Initialize an empty g.
            let mut g = DiGraph::empty(["A", "B", "C"]);
            // Add edges to the g.
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            assert!(g.is_separator_set(&set![1], &set![2], &set![])?);
            assert!(g.is_separator_set(&set![2], &set![1], &set![])?);
            assert!(!g.is_separator_set(&set![1], &set![2], &set![0])?);
            assert!(!g.is_separator_set(&set![2], &set![1], &set![0])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_primer_figure_2_7() -> Result<()> {
            let mut g = DiGraph::empty(["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert!(g.is_separator_set(&set![3], &set![4], &set![])?);
            assert!(!g.is_separator_set(&set![3], &set![4], &set![1])?);
            assert!(!g.is_separator_set(&set![3], &set![4], &set![0])?);
            assert!(g.is_separator_set(&set![3], &set![4], &set![1, 2])?);
            assert!(g.is_separator_set(&set![3], &set![4], &set![2, 1])?);

            Ok(())
        }

        #[test]
        fn is_separator_set_primer_figure_2_8() -> Result<()> {
            let mut g = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"]);
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert!(!g.is_separator_set(&set![4], &set![5], &set![])?);
            assert!(!g.is_separator_set(&set![5], &set![4], &set![])?);

            assert!(g.is_separator_set(&set![4], &set![5], &set![0])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![0])?);

            assert!(!g.is_separator_set(&set![4], &set![5], &set![0, 2])?);
            assert!(!g.is_separator_set(&set![5], &set![4], &set![0, 2])?);
            assert!(!g.is_separator_set(&set![4], &set![5], &set![2, 0])?);
            assert!(!g.is_separator_set(&set![5], &set![4], &set![2, 0])?);

            assert!(g.is_separator_set(&set![4], &set![5], &set![0, 2, 3])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![0, 2, 3])?);
            assert!(g.is_separator_set(&set![4], &set![5], &set![0, 3, 2])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![0, 3, 2])?);
            assert!(g.is_separator_set(&set![4], &set![5], &set![2, 0, 3])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![2, 0, 3])?);
            assert!(g.is_separator_set(&set![4], &set![5], &set![2, 3, 0])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![2, 3, 0])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![3, 0, 2])?);
            assert!(g.is_separator_set(&set![4], &set![5], &set![3, 0, 2])?);
            assert!(g.is_separator_set(&set![5], &set![4], &set![3, 2, 0])?);
            assert!(g.is_separator_set(&set![4], &set![5], &set![3, 2, 0])?);

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
                        let pa_x = graph.parents(&x);
                        // Get the descendants of the vertex.
                        let de_x = graph.descendants(&x);
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
        fn is_minimal_separator_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![5], &set![1], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![0], &set![5], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_out_of_bounds_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![0], &set![1], &set![5], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![], &set![1], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![0], &set![], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![0], &set![0], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_non_disjoint_x_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![0], &set![1], &set![0], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_non_disjoint_y_z() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.is_minimal_separator_set(&set![0], &set![1], &set![1], None, None)
                    .is_err()
            );
        }

        #[test]
        fn is_minimal_separator_set_edge() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B"]);
            g.add_edge(0, 1);

            assert!(!g.is_minimal_separator_set(&set![0], &set![1], &set![], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![1], &set![0], &set![], None, None)?);

            g.del_edge(0, 1);

            assert!(g.is_minimal_separator_set(&set![0], &set![1], &set![], None, None)?);
            assert!(g.is_minimal_separator_set(&set![1], &set![0], &set![], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_chain() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            assert!(!g.is_minimal_separator_set(&set![0], &set![2], &set![], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![2], &set![0], &set![], None, None)?);
            assert!(g.is_minimal_separator_set(&set![0], &set![2], &set![1], None, None)?);
            assert!(g.is_minimal_separator_set(&set![2], &set![0], &set![1], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_fork() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert!(!g.is_minimal_separator_set(&set![1], &set![2], &set![], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![2], &set![1], &set![], None, None)?);
            assert!(g.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None)?);
            assert!(g.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_collider() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            assert!(g.is_minimal_separator_set(&set![1], &set![2], &set![], None, None)?);
            assert!(g.is_minimal_separator_set(&set![2], &set![1], &set![], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![1], &set![2], &set![0], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![2], &set![1], &set![0], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_primer_figure_2_7() -> Result<()> {
            let mut g = DiGraph::empty(["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert!(g.is_minimal_separator_set(&set![3], &set![4], &set![], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![1], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![0], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![1, 2], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![3], &set![4], &set![2, 1], None, None)?);

            Ok(())
        }

        #[test]
        fn is_minimal_separator_set_primer_figure_2_8() -> Result<()> {
            let mut g = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"]);
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert!(!g.is_minimal_separator_set(&set![4], &set![5], &set![], None, None)?);
            assert!(!g.is_minimal_separator_set(&set![5], &set![4], &set![], None, None)?);

            assert!(g.is_minimal_separator_set(&set![4], &set![5], &set![0], None, None)?);
            assert!(g.is_minimal_separator_set(&set![5], &set![4], &set![0], None, None)?);

            Ok(())
        }

        // Test for `find_minimal_separator_set` method.

        #[test]
        fn find_minimal_separator_set_out_of_bounds_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_separator_set(&set![5], &set![1], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_separator_set_out_of_bounds_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_separator_set(&set![0], &set![5], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_separator_set_empty_x() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_separator_set(&set![], &set![1], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_separator_set_empty_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_separator_set(&set![0], &set![], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_separator_set_non_disjoint_x_y() {
            let g = DiGraph::empty(["A", "B", "C"]);
            assert!(
                g.find_minimal_separator_set(&set![0], &set![0], None, None)
                    .is_err()
            );
        }

        #[test]
        fn find_minimal_separator_set_edge() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B"]);
            g.add_edge(0, 1);

            assert_eq!(
                g.find_minimal_separator_set(&set![0], &set![1], None, None)?,
                None
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![0], None, None)?,
                None
            );

            g.del_edge(0, 1);

            assert_eq!(
                g.find_minimal_separator_set(&set![0], &set![1], None, None)?,
                Some(set![])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![0], None, None)?,
                Some(set![])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_chain() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(1, 2);

            assert_eq!(
                g.find_minimal_separator_set(&set![0], &set![2], None, None)?,
                Some(set![1])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![2], &set![0], None, None)?,
                Some(set![1])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_fork() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![2], None, None)?,
                Some(set![0])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![2], &set![1], None, None)?,
                Some(set![0])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_collider() -> Result<()> {
            let mut g = DiGraph::empty(["A", "B", "C"]);
            g.add_edge(1, 0);
            g.add_edge(2, 0);

            assert_eq!(
                g.find_minimal_separator_set(&set![1], &set![2], None, None)?,
                Some(set![])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![2], &set![1], None, None)?,
                Some(set![])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_primer_figure_2_7() -> Result<()> {
            let mut g = DiGraph::empty(["U", "W", "X", "Y", "Z"]);
            for (i, j) in [("X", "Y"), ("X", "W"), ("Z", "W"), ("W", "U")] {
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert_eq!(
                g.find_minimal_separator_set(&set![3], &set![4], None, None)?,
                Some(set![])
            );

            Ok(())
        }

        #[test]
        fn find_minimal_separator_set_primer_figure_2_8() -> Result<()> {
            let mut g = DiGraph::empty(["T", "U", "W", "X", "Y", "Z"]);
            for (i, j) in [
                ("T", "Z"),
                ("T", "Y"),
                ("X", "Y"),
                ("X", "W"),
                ("Z", "W"),
                ("W", "U"),
            ] {
                g.add_edge(g.label_to_index(i)?, g.label_to_index(j)?);
            }

            assert_eq!(
                g.find_minimal_separator_set(&set![4], &set![5], None, None)?,
                Some(set![0])
            );
            assert_eq!(
                g.find_minimal_separator_set(&set![5], &set![4], None, None)?,
                Some(set![0])
            );

            Ok(())
        }
    }
}

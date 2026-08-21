#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{DiGraph, Graph, Labelled},
        set,
        types::Result,
    };

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    // `empty`

    #[test]
    fn empty_labels() -> Result<()> {
        let labels: Vec<String> = vec![];
        DiGraph::empty(labels)?;

        Ok(())
    }

    #[test]
    fn unique_labels() -> Result<()> {
        let labels = vec!["A", "A", "B"];
        match DiGraph::empty(labels) {
            Err(err) => assert_eq!(err.kind.to_string(), "Labels must be unique."),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    // `has_vertex`

    #[test]
    fn has_vertex() -> Result<()> {
        // Test for ...
        let data = [
            // ... zero vertices,
            (Vec::<&str>::new(), 0, false),
            // ... one vertex,
            (vec!["A"], 0, true),
            // ... multiple vertices,
            (vec!["A", "B", "C", "D"], 1, true),
            // ... out of bounds vertices,
            (LABELS.to_vec(), 5, false),
        ];

        // Test for each scenario.
        for (i, x, f) in data {
            let graph = DiGraph::empty(i)?;
            assert_eq!(graph.has_vertex(x), f);
        }

        Ok(())
    }

    // `add_vertex`

    #[test]
    fn add_vertex() -> Result<()> {
        // Test for ...
        let data = [
            // ... zero vertices,
            (Vec::<&str>::new(), ("A", 0)),
            // ... one vertex,
            (vec!["A"], ("A", 0)),
            // ... multiple vertices,
            (vec!["A", "B", "C", "D"], ("B", 1)),
            // ... random vertices,
            (vec!["E", "B", "D", "A", "F"], ("C", 2)),
        ];

        // Test for each scenario.
        for (i, (x, f)) in data {
            let mut graph = DiGraph::empty(i)?;
            assert_eq!(graph.add_vertex(x), f);
            // Assert the labels are still sorted.
            assert!(graph.labels().iter().is_sorted());
            // Assert the new vertex exists.
            assert!(graph.has_vertex(f));
        }

        Ok(())
    }

    #[test]
    fn add_vertex_existing() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);

        // Adding an existing vertex returns its index ...
        assert_eq!(graph.add_vertex("A"), 0);
        assert_eq!(graph.add_vertex("C"), 2);
        // ... and leaves the graph unchanged.
        assert_eq!(graph.vertices().len(), LABELS.len());
        assert!(graph.has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn add_vertex_preserves_edges() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "C"])?;
        assert!(graph.add_edge(0, 1)?); // A -> C

        // Insert "B" between "A" and "C".
        let i = graph.add_vertex("B");
        assert_eq!(i, 1);
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        // The former edge is preserved, shifted by the insertion.
        assert!(graph.has_edge(0, 2)?);
        // New edges through the new vertex can be added.
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(1, 2)?);
        assert_eq!(graph.edges(), set![(0, 1), (0, 2), (1, 2)]);

        Ok(())
    }

    // `del_vertex`

    #[test]
    fn del_vertex() -> Result<()> {
        // Test for ...
        let data = [
            // ... zero vertices,
            (Vec::<&str>::new(), 0, false),
            // ... one vertex,
            (vec!["A"], 0, true),
            // ... multiple vertices,
            (vec!["A", "B", "C", "D"], 1, true),
            // ... out of bounds vertices,
            (LABELS.to_vec(), 5, false),
        ];

        // Test for each scenario.
        for (i, x, f) in data {
            let mut graph = DiGraph::empty(i)?;
            let n = graph.vertices().len();
            assert_eq!(graph.del_vertex(x), f);
            // Assert the vertex count decreased.
            assert_eq!(graph.vertices().len(), n - usize::from(f));
        }

        Ok(())
    }

    #[test]
    fn del_vertex_removes_incident_edges() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?); // A -> B
        assert!(graph.add_edge(0, 4)?); // A -> E
        assert!(graph.add_edge(1, 2)?); // B -> C

        // Delete "B" (index 1).
        assert!(graph.del_vertex(1));
        // Labels shift: A=0, C=1, D=2, E=3.
        assert!(graph.labels().iter().eq(["A", "C", "D", "E"]));

        // The incident edges of "B" are removed ...
        assert!(!graph.has_edge(0, 1)?);
        // ... and the other edges are preserved, shifted by the deletion.
        assert!(graph.has_edge(0, 3)?);
        assert_eq!(graph.edges(), set![(0, 3)]);

        Ok(())
    }

    #[test]
    fn del_vertex_to_empty() -> Result<()> {
        let mut graph = DiGraph::complete(LABELS.to_vec())?;

        // Delete the vertices one by one.
        for i in 0..LABELS.len() {
            assert!(graph.del_vertex(0));
            assert_eq!(graph.vertices().len(), LABELS.len() - i - 1);
        }

        // The graph is now empty.
        assert!(graph.vertices().is_empty());
        assert!(graph.edges().is_empty());
        assert!(!graph.del_vertex(0));

        Ok(())
    }

    // `has_edge`

    #[test]
    fn has_edge() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "C", "B"])?;

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);
        assert!(!graph.has_edge(1, 0)?);
        assert!(!graph.has_edge(0, 2)?);

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_x() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        match graph.has_edge(5, 1) {
            Err(err) => assert_eq!(err.kind.to_string(), "Index `5` is out of bounds"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_y() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        match graph.has_edge(1, 5) {
            Err(err) => assert_eq!(err.kind.to_string(), "Index `5` is out of bounds"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    // `add_edge`

    #[test]
    fn add_edge() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        match graph.add_edge(5, 1) {
            Err(err) => assert_eq!(err.kind.to_string(), "Index `5` is out of bounds"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        match graph.add_edge(1, 5) {
            Err(err) => assert_eq!(err.kind.to_string(), "Index `5` is out of bounds"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    // `del_edge`

    #[test]
    fn del_edge() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.del_edge(0, 1)?);
        assert!(!graph.has_edge(0, 1)?);

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        match graph.del_edge(5, 1) {
            Err(err) => assert_eq!(err.kind.to_string(), "Index `5` is out of bounds"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        match graph.del_edge(1, 5) {
            Err(err) => assert_eq!(err.kind.to_string(), "Index `5` is out of bounds"),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    // Inherent methods.

    #[test]
    fn parents() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(1, 0)?);
        assert!(graph.add_edge(2, 0)?);
        assert!(graph.add_edge(3, 0)?);
        assert_eq!(graph.parents(&set![0])?, set![1, 2, 3]);
        assert_eq!(graph.parents(&set![1])?, set![]);
        assert_eq!(graph.parents(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn parents_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.parents(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn ancestors() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(1, 0)?);
        assert!(graph.add_edge(2, 0)?);
        assert!(graph.add_edge(3, 1)?);
        assert!(graph.add_edge(4, 2)?);
        assert_eq!(graph.ancestors(&set![0])?, set![1, 2, 4, 3]);
        assert_eq!(graph.ancestors(&set![1])?, set![3]);
        assert_eq!(graph.ancestors(&set![2])?, set![4]);
        assert_eq!(graph.ancestors(&set![3])?, set![]);
        assert_eq!(graph.ancestors(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn ancestors_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.ancestors(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn children() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(0, 2)?);
        assert!(graph.add_edge(0, 3)?);
        assert_eq!(graph.children(&set![0])?, set![1, 2, 3]);
        assert_eq!(graph.children(&set![1])?, set![]);
        assert_eq!(graph.children(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn children_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.children(&set![5]).is_err());

        Ok(())
    }

    #[test]
    fn descendants() -> Result<()> {
        let mut graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(0, 2)?);
        assert!(graph.add_edge(1, 3)?);
        assert!(graph.add_edge(2, 4)?);
        assert_eq!(graph.descendants(&set![0])?, set![1, 2, 4, 3]);
        assert_eq!(graph.descendants(&set![1])?, set![3]);
        assert_eq!(graph.descendants(&set![2])?, set![4]);
        assert_eq!(graph.descendants(&set![3])?, set![]);
        assert_eq!(graph.descendants(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn descendants_out_of_bounds() -> Result<()> {
        let graph = DiGraph::empty(LABELS.to_vec())?;
        assert!(graph.descendants(&set![5]).is_err());

        Ok(())
    }
}

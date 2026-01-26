#[cfg(test)]
mod tests {

    use causal_hub::{
        io::JsonIO,
        labels,
        models::{Graph, Labelled, UnGraph},
        set,
        types::Result,
    };
    use ndarray::prelude::*;

    const LABELS: [&str; 5] = ["A", "B", "C", "D", "E"];

    #[test]
    fn has_edge() -> Result<()> {
        let mut graph = UnGraph::empty(["A", "C", "B"])?;

        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 0)?);
        assert!(!graph.has_edge(0, 2)?);

        Ok(())
    }

    #[test]
    fn add_edge() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 0)?);

        Ok(())
    }

    #[test]
    fn del_edge() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.del_edge(0, 1)?);
        assert!(!graph.has_edge(0, 1)?);
        assert!(!graph.has_edge(1, 0)?);

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_x() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.has_edge(5, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn has_edge_out_of_bounds_y() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.has_edge(1, 5);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.add_edge(5, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn add_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.add_edge(1, 5);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_x() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.del_edge(5, 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn del_edge_out_of_bounds_y() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.del_edge(1, 5);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn neighbors() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?);
        assert!(graph.add_edge(0, 2)?);
        assert!(graph.add_edge(0, 3)?);
        assert_eq!(graph.neighbors(&set![0])?, set![1, 2, 3]);
        assert_eq!(graph.neighbors(&set![1])?, set![0]);
        assert_eq!(graph.neighbors(&set![4])?, set![]);

        Ok(())
    }

    #[test]
    fn neighbors_out_of_bounds() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.neighbors(&set![5]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `5` is out of bounds");

        Ok(())
    }

    #[test]
    fn unique_labels() -> Result<()> {
        let labels = vec!["A", "A", "B"];
        let res = UnGraph::empty(labels);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Labels must be unique.");

        Ok(())
    }

    #[test]
    fn empty_labels() -> Result<()> {
        let labels: Vec<String> = vec![];
        UnGraph::empty(labels)?;

        Ok(())
    }

    #[test]
    fn complete_graph() -> Result<()> {
        let graph = UnGraph::complete(LABELS.to_vec())?;

        // Check that all edges exist except self-loops.
        for i in 0..5 {
            for j in 0..5 {
                if i != j {
                    assert!(graph.has_edge(i, j)?, "Edge ({i}, {j}) should exist");
                } else {
                    assert!(
                        !graph.has_edge(i, j)?,
                        "Self-loop ({i}, {i}) should not exist"
                    );
                }
            }
        }

        Ok(())
    }

    #[test]
    fn complete_graph_sorted_labels() -> Result<()> {
        let graph = UnGraph::complete(["C", "A", "B"])?;

        // Labels should be sorted.
        assert!(graph.labels().iter().is_sorted());
        assert!(graph.labels().iter().eq(["A", "B", "C"]));

        // Graph should still be complete.
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(0, 2)?);
        assert!(graph.has_edge(1, 2)?);

        Ok(())
    }

    #[test]
    fn vertices() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let vertices = graph.vertices();

        assert_eq!(vertices.len(), 5);
        assert_eq!(vertices, set![0, 1, 2, 3, 4]);

        Ok(())
    }

    #[test]
    fn vertices_empty_graph() -> Result<()> {
        let labels: Vec<String> = vec![];
        let graph = UnGraph::empty(labels)?;
        let vertices = graph.vertices();

        assert!(vertices.is_empty());

        Ok(())
    }

    #[test]
    fn has_vertex() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;

        assert!(graph.has_vertex(0));
        assert!(graph.has_vertex(4));
        assert!(!graph.has_vertex(5));
        assert!(!graph.has_vertex(100));

        Ok(())
    }

    #[test]
    fn edges_empty_graph() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let edges = graph.edges();

        assert!(edges.is_empty());

        Ok(())
    }

    #[test]
    fn edges_with_some_edges() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        graph.add_edge(0, 1)?;
        graph.add_edge(2, 3)?;
        let edges = graph.edges();

        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(0, 1)));
        assert!(edges.contains(&(2, 3)));

        Ok(())
    }

    #[test]
    fn edges_complete_graph() -> Result<()> {
        let graph = UnGraph::complete(["A", "B", "C"])?;
        let edges = graph.edges();

        // Complete graph with 3 vertices has C(3,2) = 3 edges.
        assert_eq!(edges.len(), 3);
        assert!(edges.contains(&(0, 1)));
        assert!(edges.contains(&(0, 2)));
        assert!(edges.contains(&(1, 2)));

        Ok(())
    }

    #[test]
    fn add_edge_already_exists() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(graph.add_edge(0, 1)?); // First addition returns true.
        assert!(!graph.add_edge(0, 1)?); // Second addition returns false.
        assert!(!graph.add_edge(1, 0)?); // Reverse edge also returns false.

        Ok(())
    }

    #[test]
    fn del_edge_does_not_exist() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        assert!(!graph.del_edge(0, 1)?); // Deleting non-existent edge returns false.

        Ok(())
    }

    #[test]
    fn select_subgraph() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;
        graph.add_edge(2, 3)?;

        let subgraph = graph.select(&set![0, 1, 2])?;

        assert_eq!(subgraph.labels().len(), 3);
        // Check that edges within the subgraph are preserved.
        assert!(subgraph.has_edge(0, 1)?);
        assert!(subgraph.has_edge(1, 2)?);

        Ok(())
    }

    #[test]
    fn select_subgraph_preserves_only_relevant_edges() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        graph.add_edge(0, 4)?; // A-E
        graph.add_edge(1, 2)?; // B-C

        let subgraph = graph.select(&set![1, 2, 3])?; // B, C, D

        // Edge B-C should exist in subgraph.
        assert!(subgraph.has_edge(0, 1)?); // B is now 0, C is now 1.
        // Edge A-E should not exist as A and E are not in the subgraph.

        Ok(())
    }

    #[test]
    fn select_subgraph_out_of_bounds() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let res = graph.select(&set![0, 1, 10]);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "Vertex `10` is out of bounds");

        Ok(())
    }

    #[test]
    fn from_adjacency_matrix() -> Result<()> {
        let labels = labels!["A", "B", "C"];
        let adjacency_matrix = array![
            [false, true, false],
            [true, false, true],
            [false, true, false]
        ];

        let graph = UnGraph::from_adjacency_matrix(labels, adjacency_matrix)?;

        assert_eq!(graph.labels().len(), 3);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 2)?);
        assert!(!graph.has_edge(0, 2)?);

        Ok(())
    }

    #[test]
    fn from_adjacency_matrix_incompatible_shape() -> Result<()> {
        let labels = labels!["A", "B"];
        let adjacency_matrix = array![
            [false, true, false],
            [true, false, true],
            [false, true, false]
        ];

        let res = UnGraph::from_adjacency_matrix(labels, adjacency_matrix);

        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("Incompatible shape"));

        Ok(())
    }

    #[test]
    fn from_adjacency_matrix_non_square() -> Result<()> {
        let labels = labels!["A", "B", "C"];
        let adjacency_matrix = array![[false, true, false], [true, false, true]];

        let res = UnGraph::from_adjacency_matrix(labels, adjacency_matrix);

        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("Incompatible shape"));

        Ok(())
    }

    #[test]
    fn from_adjacency_matrix_non_symmetric() -> Result<()> {
        let labels = labels!["A", "B", "C"];
        let adjacency_matrix = array![
            [false, true, false],
            [false, false, true], // Not symmetric!
            [false, true, false]
        ];

        let res = UnGraph::from_adjacency_matrix(labels, adjacency_matrix);

        assert!(res.is_err());
        assert!(res.unwrap_err().to_string().contains("must be symmetric"));

        Ok(())
    }

    #[test]
    fn from_adjacency_matrix_unsorted_labels() -> Result<()> {
        let labels = labels!["C", "A", "B"];
        let adjacency_matrix = array![
            [false, true, false],
            [true, false, true],
            [false, true, false]
        ];

        let graph = UnGraph::from_adjacency_matrix(labels, adjacency_matrix)?;

        // Labels should be sorted after construction.
        assert!(graph.labels().iter().is_sorted());

        Ok(())
    }

    #[test]
    fn to_adjacency_matrix() -> Result<()> {
        let mut graph = UnGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;

        let adjacency_matrix = graph.to_adjacency_matrix();

        assert_eq!(adjacency_matrix.shape(), &[3, 3]);
        assert!(adjacency_matrix[[0, 1]]);
        assert!(adjacency_matrix[[1, 0]]);
        assert!(adjacency_matrix[[1, 2]]);
        assert!(adjacency_matrix[[2, 1]]);
        assert!(!adjacency_matrix[[0, 2]]);
        assert!(!adjacency_matrix[[0, 0]]);

        Ok(())
    }

    #[test]
    fn neighbors_multiple_vertices() -> Result<()> {
        let mut graph = UnGraph::empty(LABELS.to_vec())?;
        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 3)?;
        graph.add_edge(2, 4)?;

        // Neighbors of {0, 1} should include {2, 3} and also 1 and 0 themselves.
        let neighbors = graph.neighbors(&set![0, 1])?;
        assert!(neighbors.contains(&2));
        assert!(neighbors.contains(&3));
        assert!(neighbors.contains(&0)); // 0 is neighbor of 1
        assert!(neighbors.contains(&1)); // 1 is neighbor of 0

        Ok(())
    }

    // Serialization tests

    #[test]
    fn serialize_deserialize_empty_graph() -> Result<()> {
        let graph = UnGraph::empty(LABELS.to_vec())?;
        let json = graph.to_json_string()?;

        // Parse and check structure.
        let parsed: serde_json::Value = serde_json::from_str(&json)?;
        assert!(parsed["labels"].is_array());
        assert!(parsed["edges"].is_array());
        assert_eq!(parsed["type"], "ungraph");

        // Deserialize and check equality.
        let restored = UnGraph::from_json_string(&json)?;
        assert_eq!(restored.labels(), graph.labels());
        assert_eq!(restored.edges(), graph.edges());

        Ok(())
    }

    #[test]
    fn deserialize_missing_type_field() -> Result<()> {
        let json = r#"{"labels": ["A", "B"], "edges": []}"#;
        let res = UnGraph::from_json_string(json);

        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn deserialize_wrong_type_field() -> Result<()> {
        let json = r#"{"labels": ["A", "B"], "edges": [], "type": "digraph"}"#;
        let res = UnGraph::from_json_string(json);

        // The error message contains the type name.
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn deserialize_unknown_vertex() -> Result<()> {
        let json = r#"{"labels": ["A", "B"], "edges": [["A", "C"]], "type": "ungraph"}"#;
        let res = UnGraph::from_json_string(json);

        assert!(res.is_err());
        assert!(
            res.unwrap_err()
                .to_string()
                .contains("label does not exist")
        );

        Ok(())
    }
}

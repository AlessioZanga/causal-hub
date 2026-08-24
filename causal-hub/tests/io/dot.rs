#[cfg(test)]
mod tests {
    use causal_hub::{
        io::DotIO,
        models::{DiGraph, Graph, HasLabels, UnGraph},
        types::Result,
    };

    #[test]
    fn digraph_to_dot_string() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;

        let dot = graph.to_dot_string()?;

        // Directionality must be declared as a digraph.
        assert!(dot.starts_with("digraph "));
        // All vertices must be present.
        assert!(dot.contains("\"A\""));
        assert!(dot.contains("\"B\""));
        assert!(dot.contains("\"C\""));
        // Edges must be serialized with the directed operator.
        assert!(dot.contains("\"A\" -> \"B\""));
        assert!(dot.contains("\"B\" -> \"C\""));
        // No undirected operator must appear.
        assert!(!dot.contains("--"));

        Ok(())
    }

    #[test]
    fn digraph_round_trip() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C", "D"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;
        graph.add_edge(0, 3)?;

        let dot = graph.to_dot_string()?;
        let parsed = DiGraph::from_dot_string(&dot)?;

        assert_eq!(graph, parsed);
        Ok(())
    }

    #[test]
    fn undigraph_round_trip() -> Result<()> {
        let mut graph = UnGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;

        let dot = graph.to_dot_string()?;
        assert!(dot.starts_with("graph "));
        assert!(dot.contains("\"A\" -- \"B\""));

        let parsed = UnGraph::from_dot_string(&dot)?;
        assert_eq!(graph.labels(), parsed.labels());
        assert_eq!(graph.edges(), parsed.edges());
        Ok(())
    }

    #[test]
    fn parse_directed_dot() -> Result<()> {
        let dot = "digraph G {\n\
            \tA -> B;\n\
            \tB -> C;\n\
        }\n";
        let graph = DiGraph::from_dot_string(dot)?;
        assert_eq!(graph.vertices().len(), 3);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 2)?);
        assert!(!graph.has_edge(1, 0)?);
        Ok(())
    }

    #[test]
    fn chain_path_produces_pairwise_edges() -> Result<()> {
        let dot = "digraph {\n\tA -> B -> C;\n}\n";
        let graph = DiGraph::from_dot_string(dot)?;
        assert_eq!(graph.vertices().len(), 3);
        assert!(graph.has_edge(0, 1)?);
        assert!(graph.has_edge(1, 2)?);
        Ok(())
    }

    #[test]
    fn vertex_label_attribute_is_used() -> Result<()> {
        let dot = "digraph {\n\
            \t1 [label=\"X\"];\n\
            \t2 [label=\"Y\"];\n\
            \t1 -> 2;\n\
        }\n";
        let graph = DiGraph::from_dot_string(dot)?;
        assert!(graph.labels().contains("X"));
        assert!(graph.labels().contains("Y"));
        let x = graph.label_to_index("X")?;
        let y = graph.label_to_index("Y")?;
        assert!(graph.has_edge(x, y)?);
        Ok(())
    }

    #[test]
    fn direction_mismatch_is_error() -> Result<()> {
        let undirected = "graph {\n\tA -- B;\n}\n";
        // Parsing an undirected DOT into a directed graph must fail.
        assert!(DiGraph::from_dot_string(undirected).is_err());
        // And vice versa.
        let directed = "digraph {\n\tA -> B;\n}\n";
        assert!(UnGraph::from_dot_string(directed).is_err());
        Ok(())
    }

    #[test]
    fn file_round_trip() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B"])?;
        graph.add_edge(0, 1)?;

        let dir = std::env::temp_dir();
        let path = dir.join(format!("causal_hub_dot_test_{}.dot", std::process::id()));

        graph.to_dot_file(&path.display().to_string())?;
        let parsed = DiGraph::from_dot_file(&path.display().to_string())?;
        let _ = std::fs::remove_file(&path);

        assert_eq!(graph, parsed);
        Ok(())
    }
}

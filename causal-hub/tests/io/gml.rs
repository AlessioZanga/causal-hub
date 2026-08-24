#[cfg(test)]
mod tests {
    use causal_hub::{
        io::GmlIO,
        models::{DiGraph, Graph, HasLabels, UnGraph},
        types::Result,
    };

    #[test]
    fn digraph_to_gml_string() -> Result<()> {
        // Build a small directed graph.
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;

        let gml = graph.to_gml_string()?;

        // Directionality must be declared as directed.
        assert!(gml.contains("directed 1"));
        // All vertices must be present.
        assert!(gml.contains("label \"A\""));
        assert!(gml.contains("label \"B\""));
        assert!(gml.contains("label \"C\""));
        // Edges must be serialized.
        assert!(gml.contains("source 0"));
        assert!(gml.contains("target 1"));
        assert!(gml.contains("source 1"));
        assert!(gml.contains("target 2"));

        Ok(())
    }

    #[test]
    fn digraph_round_trip() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B", "C", "D"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;
        graph.add_edge(0, 3)?;

        let gml = graph.to_gml_string()?;
        let parsed = DiGraph::from_gml_string(&gml)?;

        assert_eq!(graph, parsed);
        Ok(())
    }

    #[test]
    fn undigraph_round_trip() -> Result<()> {
        let mut graph = UnGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(1, 2)?;

        let gml = graph.to_gml_string()?;
        assert!(gml.contains("graphType \"graph\""));

        let parsed = UnGraph::from_gml_string(&gml)?;
        assert_eq!(graph.labels(), parsed.labels());
        assert_eq!(graph.edges(), parsed.edges());
        Ok(())
    }

    #[test]
    fn parse_directed_gml() -> Result<()> {
        let gml = "graph [\n\
            \tdirected 1\n\
            \tnode [\n\t\tid 0\n\t\tlabel \"X\"\n\t]\n\
            \tnode [\n\t\tid 1\n\t\tlabel \"Y\"\n\t]\n\
            \tedge [\n\t\tsource 0\n\t\ttarget 1\n\t]\n\
        ]\n";
        let graph = DiGraph::from_gml_string(gml)?;
        assert_eq!(graph.vertices().len(), 2);
        assert!(graph.has_edge(0, 1)?);
        assert!(!graph.has_edge(1, 0)?);
        Ok(())
    }

    #[test]
    fn direction_mismatch_is_error() -> Result<()> {
        let undirected = "graph [\n\
            \tgraphType \"graph\"\n\
            \tnode [\n\t\tid 0\n\t\tlabel \"X\"\n\t]\n\
            \tnode [\n\t\tid 1\n\t\tlabel \"Y\"\n\t]\n\
            \tedge [\n\t\tsource 0\n\t\ttarget 1\n\t]\n\
        ]\n";
        // Parsing an undirected GML into a directed graph must fail.
        assert!(DiGraph::from_gml_string(undirected).is_err());
        // And vice versa.
        let directed = "graph [\n\tdirected 1\n\tnode [\n\t\tid 0\n\t\tlabel \"X\"\n\t]\n\tnode [\n\t\tid 1\n\t\tlabel \"Y\"\n\t]\n\tedge [\n\t\tsource 0\n\t\ttarget 1\n\t]\n]\n";
        assert!(UnGraph::from_gml_string(directed).is_err());
        Ok(())
    }

    #[test]
    fn file_round_trip() -> Result<()> {
        let mut graph = DiGraph::empty(["A", "B"])?;
        graph.add_edge(0, 1)?;

        let dir = std::env::temp_dir();
        let path = dir.join(format!("causal_hub_gml_test_{}.gml", std::process::id()));

        graph.to_gml_file(&path.display().to_string())?;
        let parsed = DiGraph::from_gml_file(&path.display().to_string())?;
        let _ = std::fs::remove_file(&path);

        assert_eq!(graph, parsed);
        Ok(())
    }
}

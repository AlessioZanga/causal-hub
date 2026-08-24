mod parser;
use std::sync::Arc;

pub use parser::*;

use crate::{
    models::{DiGraph, Graph, HasLabels, UnGraph},
    types::{Error, Result},
};

/// Convert a directed graph into the graph-agnostic GML representation.
impl From<&DiGraph> for GML {
    fn from(graph: &DiGraph) -> Self {
        let vertices = graph.labels().clone();
        let labels: Vec<&String> = graph.labels().iter().collect();
        let edges = graph
            .edges()
            .iter()
            .map(|&(x, y)| (labels[x].clone(), labels[y].clone()))
            .collect();
        Self {
            graph_type: "digraph".to_string(),
            vertices,
            edges,
        }
    }
}

/// Build a directed graph from the graph-agnostic GML representation.
impl TryFrom<GML> for DiGraph {
    type Error = Error;

    fn try_from(gml: GML) -> Result<Self> {
        if gml.graph_type != "digraph" {
            return Err(Error::InvalidParameter(
                "gml graph type",
                "expected 'digraph' for a directed graph",
            ));
        }
        let mut graph = DiGraph::empty(gml.vertices.clone())?;
        for (stats, t) in gml.edges {
            let x = graph.label_to_index(&stats)?;
            let y = graph.label_to_index(&t)?;
            graph.add_edge(x, y)?;
        }
        Ok(graph)
    }
}

/// Convert an undirected graph into the graph-agnostic GML representation.
impl From<&UnGraph> for GML {
    fn from(graph: &UnGraph) -> Self {
        let vertices = graph.labels().clone();
        let labels: Vec<&String> = graph.labels().iter().collect();
        let edges = graph
            .edges()
            .iter()
            .map(|&(x, y)| (labels[x].clone(), labels[y].clone()))
            .collect();
        Self {
            graph_type: "graph".to_string(),
            vertices,
            edges,
        }
    }
}

/// Build an undirected graph from the graph-agnostic GML representation.
impl TryFrom<GML> for UnGraph {
    type Error = Error;

    fn try_from(gml: GML) -> Result<Self> {
        if gml.graph_type != "graph" {
            return Err(Error::InvalidParameter(
                "gml graph type",
                "expected 'graph' for an undirected graph",
            ));
        }
        let mut graph = UnGraph::empty(gml.vertices.clone())?;
        for (stats, t) in gml.edges {
            let x = graph.label_to_index(&stats)?;
            let y = graph.label_to_index(&t)?;
            graph.add_edge(x, y)?;
        }
        Ok(graph)
    }
}

impl GmlIO for DiGraph {
    fn from_gml_string(gml: &str) -> Result<Self> {
        DiGraph::try_from(GML::from_string(gml)?)
    }

    fn to_gml_string(&self) -> Result<String> {
        serialize(&GML::from(self))
    }

    fn from_gml_file(path: &str) -> Result<Self> {
        let string =
            std::fs::read_to_string(path).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Self::from_gml_string(&string)
    }

    fn to_gml_file(&self, path: &str) -> Result<()> {
        let string = self.to_gml_string()?;
        std::fs::write(path, string).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Ok(())
    }
}

impl GmlIO for UnGraph {
    fn from_gml_string(gml: &str) -> Result<Self> {
        UnGraph::try_from(GML::from_string(gml)?)
    }

    fn to_gml_string(&self) -> Result<String> {
        serialize(&GML::from(self))
    }

    fn from_gml_file(path: &str) -> Result<Self> {
        let string =
            std::fs::read_to_string(path).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Self::from_gml_string(&string)
    }

    fn to_gml_file(&self, path: &str) -> Result<()> {
        let string = self.to_gml_string()?;
        std::fs::write(path, string).map_err(|evidence| Error::Io(Arc::new(evidence)))?;
        Ok(())
    }
}

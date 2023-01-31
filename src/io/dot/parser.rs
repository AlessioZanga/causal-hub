use std::{collections::BTreeMap, path::Path};

use itertools::Itertools;
use pest::{
    error::Error as ParserError,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use super::attributes::{EdgeAttributes, GraphAttributes, VertexAttributes};

impl<'a> Extend<Pair<'a, Rule>> for VertexAttributes {
    fn extend<T: IntoIterator<Item = Pair<'a, Rule>>>(&mut self, iter: T) {
        // Map into iter.
        iter.into_iter()
            // Match inner rules.
            .map(|pair| pair.into_inner())
            // Map attribute to pair.
            .map(|mut pair| (pair.next().unwrap().as_str(), pair.next().unwrap().as_str()))
            // Insert into statements.
            .for_each(|(key, value)| {
                self.insert_raw_parts(key, value);
            });
    }
}

impl<'a> From<Pair<'a, Rule>> for VertexAttributes {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Allocate attributes.
        let mut attributes = Self::default();

        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::attributes));
        // Match inner rules.
        attributes.extend(pair.into_inner());

        attributes
    }
}

impl From<VertexAttributes> for String {
    fn from(attributes: VertexAttributes) -> Self {
        attributes
            .into_iter()
            .map(|x| x.into())
            .map(|(key, value)| format!("{key} = {value};"))
            .join(" ")
    }
}

impl<'a> Extend<Pair<'a, Rule>> for EdgeAttributes {
    fn extend<T: IntoIterator<Item = Pair<'a, Rule>>>(&mut self, iter: T) {
        // Map into iter.
        iter.into_iter()
            // Match inner rules.
            .map(|pair| pair.into_inner())
            // Map attribute to pair.
            .map(|mut pair| (pair.next().unwrap().as_str(), pair.next().unwrap().as_str()))
            // Insert into statements.
            .for_each(|(key, value)| {
                self.insert_raw_parts(key, value);
            });
    }
}

impl<'a> From<Pair<'a, Rule>> for EdgeAttributes {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Allocate attributes.
        let mut attributes = Self::default();

        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::attributes));
        // Match inner rules.
        attributes.extend(pair.into_inner());

        attributes
    }
}

impl From<EdgeAttributes> for String {
    fn from(attributes: EdgeAttributes) -> Self {
        attributes
            .into_iter()
            .map(|x| x.into())
            .map(|(key, value)| format!("{key} = {value};"))
            .join(" ")
    }
}

impl<'a> Extend<Pair<'a, Rule>> for GraphAttributes {
    fn extend<T: IntoIterator<Item = Pair<'a, Rule>>>(&mut self, iter: T) {
        // Map into iter.
        iter.into_iter()
            // Match inner rules.
            .map(|pair| pair.into_inner())
            // Map attribute to pair.
            .map(|mut pair| (pair.next().unwrap().as_str(), pair.next().unwrap().as_str()))
            // Insert into statements.
            .for_each(|(key, value)| {
                self.insert_raw_parts(key, value);
            });
    }
}

impl<'a> From<Pair<'a, Rule>> for GraphAttributes {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Allocate attributes.
        let mut attributes = Self::default();

        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::attributes));
        // Match inner rules.
        attributes.extend(pair.into_inner());

        attributes
    }
}

impl From<GraphAttributes> for String {
    fn from(attributes: GraphAttributes) -> Self {
        attributes
            .into_iter()
            .map(|x| x.into())
            .map(|(key, value)| format!("{key} = {value};"))
            .join(" ")
    }
}

#[derive(Clone, Debug, Default)]
pub struct GlobalAttributes {
    /// Global graphs attributes.
    pub graphs: GraphAttributes,
    /// Global vertices attributes.
    pub vertices: VertexAttributes,
    /// Global edges attributes.
    pub edges: EdgeAttributes,
}

impl<'a> Extend<Pair<'a, Rule>> for GlobalAttributes {
    fn extend<T: IntoIterator<Item = Pair<'a, Rule>>>(&mut self, iter: T) {
        // Match inner rules.
        let mut inner = iter.into_iter();
        // Match attribute type.
        match inner.next().unwrap().as_rule() {
            Rule::graph_type => self.graphs.extend(inner.next().unwrap().into_inner()),
            Rule::vertex_type => self.vertices.extend(inner.next().unwrap().into_inner()),
            Rule::path_type => self.edges.extend(inner.next().unwrap().into_inner()),
            _ => unreachable!(),
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for GlobalAttributes {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Allocate attributes.
        let mut attributes = Self::default();

        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::global_attributes));
        // Match inner rules.
        attributes.extend(pair.into_inner());

        attributes
    }
}

impl From<GlobalAttributes> for String {
    fn from(global_attributes: GlobalAttributes) -> Self {
        // Initialize output string.
        let mut dot = String::new();

        // Get `graphs` global attributes.
        let graphs: String = global_attributes.graphs.into();
        // Check if there are global attributes.
        if !graphs.is_empty() {
            // Add global attributes.
            dot += &format!("\tgraph [ {graphs} ]");
        }

        // Get `vertices` global attributes.
        let vertices: String = global_attributes.vertices.into();
        // Check if there are global attributes.
        if !vertices.is_empty() {
            // Add spacing if needed.
            if !dot.is_empty() {
                dot += &format!("\n{dot}");
            }
            // Add global attributes.
            dot += &format!("\tnode [ {vertices} ]");
        }

        // Get `edges` global attributes.
        let edges: String = global_attributes.edges.into();
        // Check if there are global attributes.
        if !edges.is_empty() {
            // Add spacing if needed.
            if !dot.is_empty() {
                dot += &format!("\n{dot}");
            }
            // Add global attributes.
            dot += &format!("\tedge [ {edges} ]");
        }

        dot
    }
}

#[derive(Clone, Debug, Default)]
pub struct Vertex {
    /// Vertex id.
    pub id: String,
    /// Vertex port, if any.
    pub port: Option<String>,
    /// Vertex attributes.
    pub attributes: VertexAttributes,
}

impl<'a> From<Pair<'a, Rule>> for Vertex {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::vertex));
        // Match inner rules.
        let mut inner = pair.into_inner();

        // Assert rule match.
        let id_port = inner.next().unwrap();
        assert!(matches!(id_port.as_rule(), Rule::vertex_id));
        // Match inner rules.
        let mut id_port = id_port.into_inner();
        let id = id_port.next().unwrap().as_str().into();
        let port = id_port.next().map(|x| x.as_str().into());

        // Match inner rules.
        let attributes = inner.next().map(|x| x.into()).unwrap_or_default();

        Self {
            id,
            port,
            attributes,
        }
    }
}

impl From<Vertex> for String {
    fn from(vertex: Vertex) -> Self {
        // Add vertex id.
        let mut dot = vertex.id;

        // Check vertex port.
        if let Some(port) = vertex.port {
            // Add vertex port.
            dot += &format!(" {port}");
        }

        // Get attributes.
        let attributes: String = vertex.attributes.into();
        // Check if there are attributes.
        if !attributes.is_empty() {
            // Add attributes.
            dot += &format!(" [ {attributes} ]");
        }

        dot
    }
}

#[derive(Clone, Debug, Default)]
pub struct Edge {
    /// Edge id as vertices pair.
    pub id: (String, String),
    /// Edge operation direction.
    pub op: String,
    /// Edge attributes.
    pub attributes: EdgeAttributes,
}

impl From<Edge> for String {
    fn from(edge: Edge) -> Self {
        // Add edge id and direction.
        let mut dot = format!("{} {} {}", edge.id.0, edge.op, edge.id.1);

        // Get attributes.
        let attributes: String = edge.attributes.into();
        // Check if there are attributes.
        if !attributes.is_empty() {
            // Add attributes.
            dot += &format!(" [ {attributes} ]");
        }

        dot
    }
}

#[derive(Default)]
struct _Path {
    /// Edges.
    pub edges: Vec<Edge>,
}

impl<'a> From<Pairs<'a, Rule>> for _Path {
    fn from(mut pairs: Pairs<'a, Rule>) -> Self {
        // Allocate path.
        let mut path = Self::default();

        // Match inner rules.
        let path_id = pairs.next().unwrap();
        // Assert rule match.
        assert!(matches!(path_id.as_rule(), Rule::path_id));
        // Match inner rules.
        let edges = path_id
            .into_inner()
            .tuple_windows()
            .step_by(2)
            .map(|(from, op, to)| {
                // Get `from` vertex id.
                let from = match from.as_rule() {
                    // TODO: Add support for subgraphs.
                    Rule::subgraph => todo!("Subgraphs not supported yet"),
                    Rule::vertex_id => from.into_inner().next().unwrap().as_str().into(),
                    _ => unreachable!(),
                };
                // Assert edge operator direction.
                assert!(matches!(op.as_rule(), Rule::path_direction));
                // Get edge operator direction.
                let op = op.as_str().into();
                // Get `to` vertex id.
                let to = match to.as_rule() {
                    // TODO: Add support for subgraphs.
                    Rule::subgraph => todo!("Subgraphs not supported yet"),
                    Rule::vertex_id => to.into_inner().next().unwrap().as_str().into(),
                    _ => unreachable!(),
                };

                ((from, to), op)
            });
        // Match inner rules.
        let attributes = pairs.next().map(|x| x.into()).unwrap_or_default();

        // Insert edges.
        path.edges.extend(
            edges
                .zip(std::iter::repeat(attributes))
                .map(|((id, op), attributes)| Edge { id, op, attributes }),
        );

        path
    }
}

#[derive(Default)]
struct _Statements {
    /// Local graph attributes.
    pub attributes: GraphAttributes,
    /// Global graph attributes.
    pub global_attributes: GlobalAttributes,
    /// Map of vertices IDs to vertices attributes.
    pub vertices: BTreeMap<String, Vertex>,
    /// Map of edges pairs to vertices attributes.
    pub edges: BTreeMap<(String, String), Edge>,
}

impl<'a> From<Pairs<'a, Rule>> for _Statements {
    fn from(pairs: Pairs<'a, Rule>) -> Self {
        // Allocate statements.
        let mut statements = Self::default();

        // Match inner rules.
        pairs.for_each(|pair| match pair.as_rule() {
            Rule::attribute => {
                // Match inner rules.
                let mut pair = pair.into_inner();
                // Map attribute to pair.
                let (key, value) = (pair.next().unwrap().as_str(), pair.next().unwrap().as_str());
                // Insert into statements.
                statements.attributes.insert_raw_parts(key, value);
            }
            Rule::global_attributes => statements.global_attributes.extend(pair.into_inner()),
            // TODO: Add support for subgraphs.
            Rule::subgraph => {}
            Rule::vertex => {
                // Match vertex rule.
                let v = Vertex::from(pair);
                // Insert into statements.
                statements.vertices.insert(v.id.clone(), v);
            }
            Rule::path => {
                // Match path rule.
                let path = _Path::from(pair.into_inner());
                // Insert into statements.
                statements
                    .edges
                    .extend(path.edges.into_iter().map(|e| (e.id.clone(), e)));
            }
            _ => unreachable!(),
        });

        statements
    }
}

#[derive(Clone, Debug, Default)]
pub struct Graph {
    /// Graph strict attribute, if set.
    pub strict: bool,
    /// Graph ID, if any.
    pub id: Option<String>,
    /// Local graph attributes.
    pub attributes: GraphAttributes,
    /// Global graph attributes.
    pub global_attributes: GlobalAttributes,
    /// Map of vertices IDs to vertices attributes.
    pub vertices: BTreeMap<String, Vertex>,
    /// Map of edges pairs to vertices attributes.
    pub edges: BTreeMap<(String, String), Edge>,
}

impl<'a> From<Pair<'a, Rule>> for Graph {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::graph));
        // Match inner rules.
        let mut inner = pair.into_inner();

        // Assert rule match.
        let strict = inner.next().unwrap();
        assert!(matches!(strict.as_rule(), Rule::strict));
        // Match inner rules.
        let strict = !strict.as_str().is_empty();

        // Assert rule match.
        let graph_type = inner.next().unwrap();
        assert!(matches!(graph_type.as_rule(), Rule::graph_type));

        // Assert rule match.
        let id = inner.next().unwrap();
        assert!(matches!(id.as_rule(), Rule::graph_id));
        // Match inner rules.
        let id = id.into_inner().next().map(|x| x.as_str().into());

        // Assert rule match.
        let statements = inner.next().unwrap();
        assert!(matches!(statements.as_rule(), Rule::statements));
        // Match inner rules.
        let statements = _Statements::from(statements.into_inner());

        Self {
            strict,
            id,
            attributes: statements.attributes,
            global_attributes: statements.global_attributes,
            vertices: statements.vertices,
            edges: statements.edges,
        }
    }
}

impl From<Graph> for String {
    fn from(graph: Graph) -> Self {
        // Allocate output string.
        let mut dot = String::new();

        // Add `strict` attribute.
        if graph.strict {
            dot += "strict ";
        }

        // Add graph type.
        dot += "graph ";
        // Concat `id` with proper spacing.
        if let Some(id) = graph.id {
            dot += &(id + " ");
        }

        // Open brackets.
        dot += "{\n";

        // Add local attributes.
        for (key, value) in graph.attributes.into_iter().map(|x| x.into()) {
            dot += &format!("\t{key} = {value};\n");
        }

        // Get global attributes.
        let global_attributes: String = graph.global_attributes.into();
        // Add global attributes.
        if !global_attributes.is_empty() {
            dot += &format!("{global_attributes}\n");
        }

        // Add vertices.
        for vertex in graph.vertices.into_values().map(String::from) {
            dot += &format!("\t{vertex}\n");
        }
        // Add edges.
        for edge in graph.edges.into_values().map(String::from) {
            dot += &format!("\t{edge}\n");
        }

        // Close brackets.
        dot += "}\n";

        dot
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiGraph {
    /// Graph strict attribute, if set.
    pub strict: bool,
    /// Graph ID, if any.
    pub id: Option<String>,
    /// Local graph attributes.
    pub attributes: GraphAttributes,
    /// Global graph attributes.
    pub global_attributes: GlobalAttributes,
    /// Map of vertices IDs to vertices attributes.
    pub vertices: BTreeMap<String, Vertex>,
    /// Map of edges pairs to vertices attributes.
    pub edges: BTreeMap<(String, String), Edge>,
}

impl<'a> From<Pair<'a, Rule>> for DiGraph {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Assert rule match.
        assert!(matches!(pair.as_rule(), Rule::digraph));
        // Match inner rules.
        let mut inner = pair.into_inner();

        // Assert rule match.
        let strict = inner.next().unwrap();
        assert!(matches!(strict.as_rule(), Rule::strict));
        // Match inner rules.
        let strict = !strict.as_str().is_empty();

        // Assert rule match.
        let digraph_type = inner.next().unwrap();
        assert!(matches!(digraph_type.as_rule(), Rule::digraph_type));

        // Assert rule match.
        let id = inner.next().unwrap();
        assert!(matches!(id.as_rule(), Rule::graph_id));
        // Match inner rules.
        let id = id.into_inner().next().map(|x| x.as_str().into());

        // Assert rule match.
        let statements = inner.next().unwrap();
        assert!(matches!(statements.as_rule(), Rule::statements));
        // Match inner rules.
        let statements = _Statements::from(statements.into_inner());
        // Move statements.
        let (attributes, global_attributes, vertices, edges) = (
            statements.attributes,
            statements.global_attributes,
            statements.vertices,
            statements.edges,
        );

        Self {
            strict,
            id,
            attributes,
            global_attributes,
            vertices,
            edges,
        }
    }
}

impl From<DiGraph> for String {
    fn from(graph: DiGraph) -> Self {
        // Allocate output string.
        let mut dot = String::new();

        // Add `strict` attribute.
        if graph.strict {
            dot += "strict ";
        }

        // Add graph type.
        dot += "digraph ";
        // Concat `id` with proper spacing.
        if let Some(id) = graph.id {
            dot += &(id + " ");
        }

        // Open brackets.
        dot += "{\n";

        // Add local attributes.
        for (key, value) in graph.attributes.into_iter().map(|x| x.into()) {
            dot += &format!("\t{key} = {value};\n");
        }
        // Get global attributes.
        let global_attributes: String = graph.global_attributes.into();
        // Add global attributes.
        if !global_attributes.is_empty() {
            dot += &format!("{global_attributes}\n");
        }
        // Add vertices.
        for vertex in graph.vertices.into_values().map(String::from) {
            dot += &format!("\t{vertex}\n");
        }
        // Add edges.
        for edge in graph.edges.into_values().map(String::from) {
            dot += &format!("\t{edge}\n");
        }

        // Close brackets.
        dot += "}\n";

        dot
    }
}

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot/grammar.pest"]
pub enum DOT {
    Graph(Graph),
    DiGraph(DiGraph),
}

impl DOT {
    #[allow(clippy::result_large_err)]
    pub fn read(path: &Path) -> Result<Self, ParserError<Rule>> {
        // Read file to string.
        let string = std::fs::read_to_string(path)
            .unwrap_or_else(|_| format!("Failed to read file: \"{}\"", path.display()));

        Self::try_from(string)
    }
}

impl<'a> From<Pair<'a, Rule>> for DOT {
    fn from(pair: Pair<'a, Rule>) -> Self {
        // Match inner rules.
        match pair.as_rule() {
            Rule::graph => DOT::Graph(Graph::from(pair)),
            Rule::digraph => DOT::DiGraph(DiGraph::from(pair)),
            _ => unreachable!(),
        }
    }
}

impl From<DOT> for String {
    fn from(dot: DOT) -> Self {
        match dot {
            DOT::Graph(dot) => dot.into(),
            DOT::DiGraph(dot) => dot.into(),
        }
    }
}

impl TryFrom<String> for DOT {
    type Error = ParserError<Rule>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Parse the given dot string.
        let dot = DOT::parse(Rule::file, string.trim())?;
        // Match inner rules. TODO: Match more than one graph.
        let dot: DOT = dot.map(|x| x.into()).next().unwrap();

        Ok(dot)
    }
}

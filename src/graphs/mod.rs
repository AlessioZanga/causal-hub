/// Algorithms on graphs.
pub mod algorithms;

mod base;
pub use base::{directions, BaseGraph};

mod default;
pub use default::DefaultGraph;

mod error;
pub use error::ErrorGraph;

mod partial_ord;
pub use partial_ord::PartialOrdGraph;

mod directed;
pub use directed::DirectedGraph;

mod undirected;
pub use undirected::UndirectedGraph;

mod structs;
pub use structs::{DirectedDenseAdjacencyMatrixGraph, UndirectedDenseAdjacencyMatrixGraph};

/// Default undirected graph implementation based on dense adjacency matrix.
pub type Graph = UndirectedDenseAdjacencyMatrixGraph;

/// Default directed graph implementation based on dense adjacency matrix.
pub type DiGraph = DirectedDenseAdjacencyMatrixGraph;

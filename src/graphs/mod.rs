/// Algorithms on graphs.
pub mod algorithms;

/// Structures on graphs.
pub mod structs;

mod acyclic;
pub use acyclic::*;

mod base;
pub use base::*;

mod default;
pub use default::*;

mod direction;
pub use direction::*;

mod error;
pub use error::*;

mod partial_ord;
pub use partial_ord::*;

mod subgraph;
pub use subgraph::*;

/// Default undirected graph implementation based on dense adjacency matrix.
pub type Graph = structs::UndirectedDenseAdjacencyMatrixGraph;

/// Default directed graph implementation based on dense adjacency matrix.
pub type DiGraph = structs::DirectedDenseAdjacencyMatrixGraph;

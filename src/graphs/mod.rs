/// Algorithms on graphs.
pub mod algorithms;

/// Structures on graphs.
pub mod structs;

mod base;
pub use base::*;

mod direction;
pub use direction::*;

mod partial_ord;
pub use partial_ord::*;

mod path;
pub use path::*;

mod subgraph;
pub use subgraph::*;

/// Default undirected graph implementation based on dense adjacency matrix.
pub type Graph = structs::UndirectedDenseAdjacencyMatrixGraph;

/// Default directed graph implementation based on dense adjacency matrix.
pub type DiGraph = structs::DirectedDenseAdjacencyMatrixGraph;

/// Default mixed graph implementation based on two dense adjacency matrices.
pub type PDGraph = structs::PartiallyDenseAdjacencyMatrixGraph;

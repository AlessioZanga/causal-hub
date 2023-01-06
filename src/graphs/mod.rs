/// Algorithms on graphs.
pub mod algorithms;

mod base;
pub use base::*;

mod default;
pub use default::*;

mod error;
pub use error::*;

mod partial_ord;
pub use partial_ord::*;

mod subgraph;
pub use subgraph::*;

mod directed;
pub use directed::*;

mod undirected;
pub use undirected::*;

mod structs;
pub use structs::*;

/// Default undirected graph implementation based on dense adjacency matrix.
pub type Graph = UndirectedDenseAdjacencyMatrixGraph;

/// Default directed graph implementation based on dense adjacency matrix.
pub type DiGraph = DirectedDenseAdjacencyMatrixGraph;

pub mod undirected_dense_adjacency_matrix;
pub use undirected_dense_adjacency_matrix::{UGraph, UndirectedDenseAdjacencyMatrix};

pub mod partially_directed_dense_adjacency_matrix;
pub use partially_directed_dense_adjacency_matrix::{
    PGraph, PartiallyDirectedDenseAdjacencyMatrix,
};

pub mod directed_dense_adjacency_matrix;
pub use directed_dense_adjacency_matrix::{DGraph, DirectedDenseAdjacencyMatrix};

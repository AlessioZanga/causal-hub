#![allow(unused_imports, dead_code)] // FIXME: remove this line
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::FusedIterator,
};

use serde::{Deserialize, Serialize};

use crate::graphs::structs::UndirectedDenseAdjacencyMatrixGraph;

//TODO: define `PartiallyGraph` specific macros

/// Partially directed graph trait.
pub trait PartiallyGraph:
    Clone + Debug + Display + Hash + Send + Sync + Serialize + for<'a> Deserialize<'a>
{
    /// Data type.
    type Data;

    /// Directional type.
    type Direction;

    /// Labels iterator type.
    type LabelsIter<'a>: Iterator<Item = &'a str> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Vertices iterator type.
    type VerticesIter<'a>: Clone + Iterator<Item = usize> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Edges iterator type.
    type EdgesIter<'a>: Iterator<Item = (usize, usize)> + ExactSizeIterator + FusedIterator
    where
        Self: 'a;

    /// Adjacents vertices iterator type.
    type AdjacentsIter<'a>: Iterator<Item = usize> + FusedIterator
    where
        Self: 'a;

    /// Specilized new constructor. Pay attention: multiple types of edges between two nodes is not allowed
    fn new_spec<V, I, J>(vertices: I, undirected_edges: J, directed_edges: J) -> Self
    where
        V: Into<String>,
        I: IntoIterator<Item = V>,
        J: IntoIterator<Item = (V, V)>;

    /// Specilized edge iterator. Parameter `which` can be either `u` for undirected or `d` for directed edge type.
    fn edges_of_type(&self, which: char) -> Self::EdgesIter<'_>; //TODO: create macro

    /// Specialized size of the graph. Parameter `which` can be either `u` for undirected or `d` for directed edge type.
    fn size_of_type(&self, which: char) -> usize;

    /// Type of the edge. It returns `None` if such edge doesn't exist, an `Option<char>` on the contrary. `char` can be `u` for undirected or `d` for directed edge type.
    fn type_of_edge(&self, x: usize, y: usize) -> Option<char>;

    /// Specilized edge adder. Parameter `which` can be either `u` for undirected or `d` for directed edge type.
    fn add_edge_of_type(&mut self, x: usize, y: usize, which: char) -> bool;

    //TODO: Improve documentation
}

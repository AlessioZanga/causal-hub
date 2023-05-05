use itertools::Itertools;

use crate::{
    graphs::{directions, IntoUndirected},
    prelude::{DirectedGraph, UndirectedGraph},
    Pa, V,
};

/// Build a moral graph.
///
/// # Examples
///
/// ```
/// use itertools::Itertools;
///
/// use causal_hub::prelude::*;
/// use causal_hub::graphs::algorithms::moral::*;
///
/// // Build a new directed graph.
/// let g = DiGraph::new(
///     ["A", "B", "C", "D", "E"],
///     [("A", "C"), ("B", "C")]
/// );
///
/// // Build the associated moral graph.
/// let h: Graph = moralize(g.clone());
///
/// // Assert previous parents are connected.
/// for x in V!(g) {
///     for e in Pa!(g, x).combinations(2) {
///         assert!(h.has_edge_by_index(e[0], e[1]));
///     }
/// }
/// ```
///
pub fn moralize<G, H>(g: G) -> H
where
    G: DirectedGraph<Direction = directions::Directed> + IntoUndirected<UndirectedGraph = H>,
    H: UndirectedGraph<Direction = directions::Undirected>,
{
    // Make an undirected copy of the current graph.
    let mut h = g.clone().into_undirected();
    // For each vertex ...
    for x in V!(g) {
        // ... for each pair of parents ...
        for e in Pa!(g, x).combinations(2) {
            // ... add an edge between them.
            h.add_edge_by_index(e[0], e[1]);
        }
    }

    h
}

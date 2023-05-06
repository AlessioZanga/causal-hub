use itertools::Itertools;

use crate::{
    graphs::{directions, PathGraph, UndirectedGraph},
    types::FxIndexSet,
    Ne, V,
};

/// Maximum Cardinality Search (MCS).
pub struct MaximumCardinalitySearch;

impl MaximumCardinalitySearch {
    /// Compute a perfect elimination order and triangulate the graph.
    pub fn call<G>(mut g: G) -> (Vec<usize>, G)
    where
        G: UndirectedGraph<Direction = directions::Undirected> + PathGraph,
    {
        // Get the associated fill-in.
        let (a, f) = Self::fill_in(&g);
        // Apply the fill-in.
        for (x, y) in f {
            // Add the missing edges.
            g.add_edge_by_index(x, y);
        }

        (a, g)
    }

    /// Compute an elimination order by indices.
    pub fn elimination_order<G>(g: &G) -> Vec<usize>
    where
        G: UndirectedGraph<Direction = directions::Undirected> + PathGraph,
    {
        // Initialize the elimination order.
        let mut a: FxIndexSet<_> = Default::default();

        // While there are still unlabeled vertices.
        while a.len() < g.order() {
            // Get an unlabeled vertex x ...
            let x = V!(g)
                .filter(|x| !a.contains(x))
                // ... with maximum number of labeled neighbors. Solve dual to keep order.
                .min_by_key(|&x| Ne!(g, x).filter(|y| !a.contains(y)).count())
                .unwrap();
            // Set x as labeled.
            a.insert(x);
        }

        a.into_iter().collect()
    }

    /// Compute a perfect elimination order and a fill-in edge set by indices.
    pub fn fill_in<G>(g: &G) -> (Vec<usize>, Vec<(usize, usize)>)
    where
        G: UndirectedGraph<Direction = directions::Undirected> + PathGraph,
    {
        // Initialize the elimination order.
        let mut a: FxIndexSet<_> = Default::default();
        // Initialize the fill-in edge set.
        let mut f: FxIndexSet<_> = Default::default();

        // While there are still unlabeled vertices.
        while a.len() < g.order() {
            // Get an unlabeled vertex x ...
            let x = V!(g)
                .filter(|x| !a.contains(x))
                // ... with maximum number of labeled neighbors. Solve dual to keep order.
                .min_by_key(|&x| Ne!(g, x).filter(|y| !a.contains(y)).count())
                .unwrap();
            // Set x as labeled.
            a.insert(x);

            // If Ne(k) \cap \alpha ...
            if Ne!(g, x)
                .filter(|x| a.contains(x))
                // ... does not induce a complete subgraph ...
                .combinations(2)
                .map(|e| (e[0], e[1]))
                .filter(|&(x, y)| !g.has_edge_by_index(x, y))
                // ... add the missing edges into the fill-in edge set ...
                .fold(false, |acc, e| acc | f.insert(e))
            {
                // ... and initialize the elimination order.
                a.clear();
            }
        }

        (a.into_iter().collect(), f.into_iter().collect())
    }
}

/// Alias for the Maximum Cardinality Search algorithm.
pub type MCS = MaximumCardinalitySearch;

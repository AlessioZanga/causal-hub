use itertools::Itertools;

use crate::{
    graphs::{directions, PathGraph, UndirectedGraph},
    types::FxIndexSet,
    Ne, V,
};

/// Maximum Cardinality Search (MCS).
pub struct MaximumCardinalitySearch<'a, G> {
    g: &'a G,
}

impl<'a, G> MaximumCardinalitySearch<'a, G>
where
    G: UndirectedGraph<Direction = directions::Undirected> + PathGraph,
{
    /// Construct a new Maximum Cardinality Search functor.
    pub const fn new(g: &'a G) -> Self {
        Self { g }
    }

    /// Compute a perfect elimination order and triangulate the graph.
    pub fn call(&self) -> (Vec<usize>, G) {
        // Get the associated fill-in.
        let (a, f) = self.fill_in();
        // Clone the graph.
        let mut h = self.g.clone();
        // Apply the fill-in.
        for (x, y) in f {
            // Add the missing edges.
            h.add_edge_by_index(x, y);
        }

        (a, h)
    }

    /// Compute an elimination order by indices.
    pub fn elimination_order(&self) -> Vec<usize> {
        // Initialize the elimination order.
        let mut a: FxIndexSet<_> = Default::default();

        // While there are still unlabeled vertices.
        while a.len() < self.g.order() {
            // Get an unlabeled vertex x ...
            let x = V!(self.g)
                .filter(|x| !a.contains(x))
                // ... with maximum number of labeled neighbors. Solve dual to keep order.
                .min_by_key(|&x| Ne!(self.g, x).filter(|y| !a.contains(y)).count())
                .unwrap();
            // Set x as labeled.
            a.insert(x);
        }

        a.into_iter().collect()
    }

    /// Compute a perfect elimination order and a fill-in edge set by indices.
    pub fn fill_in(&self) -> (Vec<usize>, Vec<(usize, usize)>) {
        // Initialize the elimination order.
        let mut a: FxIndexSet<_> = Default::default();
        // Initialize the fill-in edge set.
        let mut f: FxIndexSet<_> = Default::default();

        // While there are still unlabeled vertices.
        while a.len() < self.g.order() {
            // Get an unlabeled vertex x ...
            let x = V!(self.g)
                .filter(|x| !a.contains(x))
                // ... with maximum number of labeled neighbors. Solve dual to keep order.
                .min_by_key(|&x| Ne!(self.g, x).filter(|y| !a.contains(y)).count())
                .unwrap();
            // Set x as labeled.
            a.insert(x);

            // If Ne(k) \cap \alpha ...
            if Ne!(self.g, x)
                .filter(|x| a.contains(x))
                // ... does not induce a complete subgraph ...
                .combinations(2)
                .map(|e| (e[0], e[1]))
                .filter(|&(x, y)| !self.g.has_edge_by_index(x, y))
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
pub type MCS<'a, G> = MaximumCardinalitySearch<'a, G>;

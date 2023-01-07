use std::collections::BTreeSet;

use super::IntoMoralGraph;
use crate::{
    graphs::directions,
    prelude::{BaseGraph, DirectedGraph, UndirectedGraph, CC},
    An, V,
};

pub trait Independence {
    fn is_independent<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>;
}

pub struct GraphicalIndependence<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    g: &'a G,
}

impl<'a, G, D> GraphicalIndependence<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    pub fn new(g: &'a G) -> Self {
        Self { g }
    }
}

impl<'a, G, D> From<&'a G> for GraphicalIndependence<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}

/* Implement u-separation */
impl<'a, G> Independence for GraphicalIndependence<'a, G, directions::Undirected>
where
    G: BaseGraph<Direction = directions::Undirected> + UndirectedGraph,
{
    fn is_independent<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>,
    {
        // Check that X and Y are non-empty.
        let x: BTreeSet<_> = x.into_iter().collect();
        let y: BTreeSet<_> = y.into_iter().collect();
        assert!(!x.is_empty() && !y.is_empty(), "X and Y must be non-empty");

        // Check that X, Y and Z are disjoint, if not panic.
        let z: BTreeSet<_> = z.into_iter().collect();
        assert!(
            x.is_disjoint(&y) && y.is_disjoint(&z) && z.is_disjoint(&x),
            "X, Y and Z must be disjoint sets"
        );

        // Check that X, Y and Z are in V, if not panic.
        let v: BTreeSet<_> = V!(self.g).collect();
        assert!(
            x.is_subset(&v) && y.is_subset(&v) && z.is_subset(&v),
            "X, Y and Z must be subsets of V"
        );

        // Remove vertices in Z from the graph.
        let h = self.g.subgraph_by_vertices(&x | &y);
        // Re-map vertices identifiers.
        let x = x.into_iter().map(|x| h.vertex(self.g.label(x))).collect();
        let y = y.into_iter().map(|y| h.vertex(self.g.label(y))).collect();

        // Compute the connected components of the modified graph.
        let mut cc = CC::from(&h);

        // Check if there exists at least one connected component C s.t.
        //          |C \cap X| > 0 && |C \cap Y| > 0 .
        cc.any(|c| !(&c & &x).is_empty() && !(&c & &y).is_empty())
    }
}

/* Implement d-separation */
impl<'a, G> Independence for GraphicalIndependence<'a, G, directions::Directed>
where
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph + IntoMoralGraph,
{
    fn is_independent<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>,
    {
        // Collect vertices into sets.
        let x: BTreeSet<_> = x.into_iter().collect();
        let y: BTreeSet<_> = y.into_iter().collect();
        let z: BTreeSet<_> = z.into_iter().collect();

        // Compute S = X \cup Y \cup Z.
        let s = &(&x | &y) | &z;
        // Compute the ancestors of S.
        let an_s = s.iter().flat_map(|&s| An!(self.g, s)).collect();
        // Compute the ancestral set of S.
        let an_s = &s | &an_s;

        // Compute the upward closure w.r.t. the ancestral set of S.
        let g_s = self.g.subgraph_by_vertices(an_s);
        // Compute the moralized upward closure.
        let h = g_s.into_moral();
        // Re-map vertices identifiers.
        let x = x.into_iter().map(|x| h.vertex(self.g.label(x)));
        let y = y.into_iter().map(|y| h.vertex(self.g.label(y)));
        let z = z.into_iter().map(|z| h.vertex(self.g.label(z)));

        // Then D-SEP_G(X, Y, Z) iff U-SEP_H(X, Y, Z).
        GraphicalIndependence::from(&h).is_independent(x, y, z)
    }
}

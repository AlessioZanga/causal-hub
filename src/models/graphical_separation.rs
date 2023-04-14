use std::fmt::Debug;

use super::{GeneralizedIndependence, Independence, MoralGraph};
use crate::{
    graphs::directions,
    prelude::{BaseGraph, DirectedGraph, UndirectedGraph, CC},
    types::FxIndexSet,
    utils::UnionFind,
    Adj, An, Ch, Ne, V,
};

/// Graphical independence struct
#[derive(Clone, Debug)]
pub struct GraphicalSeparation<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    g: &'a G,
}

impl<'a, G, D> GraphicalSeparation<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Build a new graphical independence struct.
    ///
    /// # Panics
    ///
    /// If $\mathbf{X}$, $\mathbf{Y}$ and $\mathbf{Z}$
    /// are not disjoint subsets of $\mathbf{V}$.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Build a new directed graph.
    /// let g = DiGraph::new(
    ///     ["A", "B", "C", "D", "E", "F"],
    ///     [
    ///         ("A", "C"),
    ///         ("B", "C"),
    ///         ("C", "D"),
    ///         ("C", "E"),
    ///     ]
    /// );
    ///
    /// // Build d-separation query struct.
    /// let q = GSeparation::from(&g);
    ///
    /// // Assert A _||_ B | { } .
    /// assert!(q.are_independent([0], [1], []));
    /// // Assert A _||_ B | { C } .
    /// assert!(!q.are_independent([0], [1], [2]));
    /// // Assert A _||_ D | { } .
    /// assert!(!q.are_independent([0], [3], []));
    /// // Assert A _||_ D | { C } .
    /// assert!(q.are_independent([0], [3], [2]));
    /// // Assert { A, B } _||_ { D, E } | { C } .
    /// assert!(q.are_independent([0, 1], [3, 4], [2]));
    /// ```
    ///
    #[inline]
    pub const fn new(g: &'a G) -> Self {
        Self { g }
    }
}

impl<'a, G, D> From<&'a G> for GraphicalSeparation<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    #[inline]
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}

/* Implement u-separation */
impl<'a, G> Independence for GraphicalSeparation<'a, G, directions::Undirected>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    #[inline]
    fn is_independent(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // TODO: Implement more efficient non-generalized version.
        <Self as GeneralizedIndependence>::are_independent(self, [x], [y], z.iter().cloned())
    }
}

impl<'a, G> GeneralizedIndependence for GraphicalSeparation<'a, G, directions::Undirected>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    /// Checks whether $\mathbf{X} \mathrlap{\thinspace\perp}{\perp}_{\mathcal{G}} \mathbf{Y} \mid \mathbf{Z}$ holds or not.
    fn are_independent<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>,
    {
        // Check that X and Y are non-empty.
        let x: FxIndexSet<_> = x.into_iter().collect();
        let y: FxIndexSet<_> = y.into_iter().collect();
        assert!(!x.is_empty() && !y.is_empty(), "X and Y must be non-empty");

        // Check that X, Y and Z are disjoint, if not panic.
        let z: FxIndexSet<_> = z.into_iter().collect();
        assert!(
            x.is_disjoint(&y) && y.is_disjoint(&z) && z.is_disjoint(&x),
            "X, Y and Z must be disjoint sets"
        );

        // Check that X, Y and Z are in V, if not panic.
        let v: FxIndexSet<_> = V!(self.g).collect();
        assert!(
            x.is_subset(&v) && y.is_subset(&v) && z.is_subset(&v),
            "X, Y and Z must be subsets of V"
        );

        // Clone current graph.
        let mut h = self.g.clone();

        // Compute the set of out-going edges of Z.
        let e_z = z
            .into_iter()
            .flat_map(|z| Ne!(self.g, z).map(move |w| (z, w)));
        // Disconnect vertices in Z from the rest of the graph.
        for (z, w) in e_z {
            h.del_edge_by_index(z, w);
        }

        // Initialize union-find.
        let mut union_find = UnionFind::new(h.order());
        // Add X to union-find.
        let root_x = *x.first().unwrap();
        union_find.extend(x);
        // Add X to union-find.
        let root_y = *y.first().unwrap();
        union_find.extend(y);

        // Compute the connected components of the modified graph.
        let mut cc = CC::from(&h);

        // Check if there exists no connected component C s.t.
        //          |C \cap X| > 0 && |C \cap Y| > 0 .
        !cc.any(|c| {
            // Add current connected component to union-find.
            union_find.extend(c);
            // Check if X and Y are in the same set.
            union_find.contains(root_x, root_y)
        })
    }
}

/* Implement d-separation */
impl<'a, G> Independence for GraphicalSeparation<'a, G, directions::Directed>
where
    G: DirectedGraph<Direction = directions::Directed> + MoralGraph,
{
    #[inline]
    fn is_independent(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // TODO: Implement more efficient non-generalized version.
        <Self as GeneralizedIndependence>::are_independent(self, [x], [y], z.iter().cloned())
    }
}

impl<'a, G> GeneralizedIndependence for GraphicalSeparation<'a, G, directions::Directed>
where
    G: DirectedGraph<Direction = directions::Directed> + MoralGraph,
{
    /// Checks whether $\mathbf{X} \mathrlap{\thinspace\perp}{\perp}_{\mathcal{G}} \mathbf{Y} \mid \mathbf{Z}$ holds or not.
    fn are_independent<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>,
    {
        // Check that X and Y are non-empty.
        let x: FxIndexSet<_> = x.into_iter().collect();
        let y: FxIndexSet<_> = y.into_iter().collect();
        assert!(!x.is_empty() && !y.is_empty(), "X and Y must be non-empty");

        // Check that X, Y and Z are disjoint, if not panic.
        let z: FxIndexSet<_> = z.into_iter().collect();
        assert!(
            x.is_disjoint(&y) && y.is_disjoint(&z) && z.is_disjoint(&x),
            "X, Y and Z must be disjoint sets"
        );

        // Compute S = X \cup Y \cup Z.
        let s = &(&x | &y) | &z;

        // Check that X, Y and Z are in V, if not panic.
        let v: FxIndexSet<_> = V!(self.g).collect();
        assert!(s.is_subset(&v), "X, Y and Z must be subsets of V");

        // Clone current graph.
        let mut h = self.g.to_undirected();

        // Compute the ancestors of S.
        let an_s: FxIndexSet<_> = s.iter().flat_map(|&s| An!(self.g, s)).collect();
        // Compute the ancestral set of S.
        let an_s = &s | &an_s;

        // Compute the set of out-going edges of V \ An_S.
        let e_s = (&v - &an_s)
            .into_iter()
            .flat_map(|s| Adj!(self.g, s).flat_map(move |t| [(s, t), (t, s)]));
        // Disconnect vertices in V \ S from the rest of the graph, i.e. compute the upward closure.
        for (s, t) in e_s {
            h.del_edge_by_index(s, t);
        }

        // Compute the set of out-going edges of Z.
        let e_z = z
            .into_iter()
            .flat_map(|z| Ch!(self.g, z).map(move |w| (z, w)));
        // Disconnect vertices in Z from the rest of the graph, i.e. compute the moral graph.
        for (z, w) in e_z {
            h.del_edge_by_index(z, w);
        }

        // Initialize union-find.
        let mut union_find = UnionFind::new(h.order());
        // Add X to union-find.
        let root_x = *x.first().unwrap();
        union_find.extend(x);
        // Add X to union-find.
        let root_y = *y.first().unwrap();
        union_find.extend(y);

        // Compute the connected components of the modified graph.
        let mut cc = CC::from(&h);

        // Check if there exists no connected component C s.t.
        //          |C \cap X| > 0 && |C \cap Y| > 0 .
        !cc.any(|c| {
            // Add current connected component to union-find.
            union_find.extend(c);
            // Check if X and Y are in the same set.
            union_find.contains(root_x, root_y)
        })
    }
}

use std::{collections::VecDeque, fmt::Debug};

use super::MoralGraph;
use crate::{
    graphs::{
        algorithms::components::CC, Directed, DirectedGraph, Graph, Undirected, UndirectedGraph,
    },
    stats::{ConditionalIndependenceTest, GeneralizedConditionalIndependenceTest},
    types::FxIndexSet,
    utils::UnionFind,
    Adj, An, Ch, Ne, Pa, L, V,
};

#[derive(Clone, Debug)]
pub struct GraphicalSeparation<'a, G, D>
where
    G: Graph<Direction = D>,
{
    g: &'a G,
}

impl<'a, G, D> GraphicalSeparation<'a, G, D>
where
    G: Graph<Direction = D>,
{
    #[inline]
    pub const fn new(g: &'a G) -> Self {
        Self { g }
    }
}

impl<'a, G, D> From<&'a G> for GraphicalSeparation<'a, G, D>
where
    G: Graph<Direction = D>,
{
    #[inline]
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}

/* Implement u-separation */
impl<'a, G> ConditionalIndependenceTest for GraphicalSeparation<'a, G, Undirected>
where
    G: UndirectedGraph<Direction = Undirected>,
{
    type LabelsIter<'b> = G::LabelsIter<'b> where G: 'b, Self: 'b;

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        L!(self.g)
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // TODO: Implement more efficient non-generalized version.
        GeneralizedConditionalIndependenceTest::call(self, [x], [y], z.iter().cloned())
    }
}

impl<'a, G> GeneralizedConditionalIndependenceTest for GraphicalSeparation<'a, G, Undirected>
where
    G: UndirectedGraph<Direction = Undirected>,
{
    type LabelsIter<'b> = G::LabelsIter<'b> where G: 'b, Self: 'b;

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        L!(self.g)
    }

    fn call<I, J, K>(&self, x: I, y: J, z: K) -> bool
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
            h.del_edge(z, w);
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
impl<'a, G> ConditionalIndependenceTest for GraphicalSeparation<'a, G, Directed>
where
    G: DirectedGraph<Direction = Directed> + MoralGraph,
{
    type LabelsIter<'b> = G::LabelsIter<'b> where G: 'b, Self: 'b;

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        L!(self.g)
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Phase I - Get all ancestors of Z.
        let z: FxIndexSet<_> = z.iter().cloned().collect();
        let an_z: FxIndexSet<_> = z.iter().flat_map(|&z| An!(self.g, z)).collect();

        // Phase II - Traverse the active trail from X to Y.

        // Initialize the set of to-be-visited vertices.
        let mut to_be_visited = VecDeque::with_capacity(2 * self.g.order());
        to_be_visited.push_back((x, true));
        // Initialize the set of visited vertices.
        let mut visited = FxIndexSet::<(usize, bool)>::default();
        visited.reserve(2 * self.g.order());

        // While there are vertices to be visited.
        while let Some((w, d)) = to_be_visited.pop_front() {
            // Check if current vertex is Y.
            if w == y {
                return false;
            }
            // Check if current vertex has already been visited.
            if visited.contains(&(w, d)) {
                continue;
            }
            // Add current vertex to visited set.
            visited.insert((w, d));
            // Trail up through W if W not in Z.
            if d && !z.contains(&w) {
                // Add parents of W to to-be-visited set.
                to_be_visited.extend(Pa!(self.g, w).map(|w| (w, true)));
                // Add children of W to to-be-visited set.
                to_be_visited.extend(Ch!(self.g, w).map(|w| (w, false)));
            // Trail down through W.
            } else if !d {
                // If W is not in Z, add children of W to to-be-visited set.
                if !z.contains(&w) {
                    to_be_visited.extend(Ch!(self.g, w).map(|w| (w, false)));
                }
                // If W is in the ancestral set of Z, add parents of W to to-be-visited set.
                if an_z.contains(&w) {
                    to_be_visited.extend(Pa!(self.g, w).map(|w| (w, true)));
                }
            }
        }

        true
    }
}

impl<'a, G> GeneralizedConditionalIndependenceTest for GraphicalSeparation<'a, G, Directed>
where
    G: DirectedGraph<Direction = Directed> + MoralGraph,
{
    type LabelsIter<'b> = G::LabelsIter<'b> where G: 'b, Self: 'b;

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        L!(self.g)
    }

    fn call<I, J, K>(&self, x: I, y: J, z: K) -> bool
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
            h.del_edge(s, t);
        }

        // Compute the set of out-going edges of Z.
        let e_z = z
            .into_iter()
            .flat_map(|z| Ch!(self.g, z).map(move |w| (z, w)));
        // Disconnect vertices in Z from the rest of the graph, i.e. compute the moral graph.
        for (z, w) in e_z {
            h.del_edge(z, w);
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

pub type GSeparation<'a, G, D> = GraphicalSeparation<'a, G, D>;

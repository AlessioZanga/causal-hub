use itertools::Itertools;
use rayon::prelude::*;

use super::MeekRules;
use crate::{
    graphs::{
        DirectedGraph, Graph, PartiallyDirected, PartiallyDirectedGraph, UGraph, Undirected,
        UndirectedGraph,
    },
    stats::ConditionalIndependenceTest,
    types::{FxIndexSet, SepSets},
    Adj, E, L, V,
};

#[derive(Clone, Debug)]

pub struct PCStable<'a, T>
where
    T: ConditionalIndependenceTest,
{
    test: &'a T,
}

impl<'a, T> PCStable<'a, T>
where
    T: ConditionalIndependenceTest + 'a,
{
    #[inline]
    pub const fn new(test: &'a T) -> Self {
        Self { test }
    }

    pub fn skeleton<U>(&self) -> (U, SepSets)
    where
        U: UndirectedGraph<Direction = Undirected>,
    {
        // Set complete graph
        let mut g = U::complete(L!(self.test));
        // Initialize set of separating sets
        let mut sepsets = SepSets::default();
        // Initialize stopping criterion
        let mut flag = true;
        // Initialize size of conditioning set
        let mut c = 0;

        while flag {
            // Unset the flag.
            flag = false;

            // Map and collect each edge in:
            // 1. The edge
            // 2. Its separation set (if any)
            // 3. A flag indicating if exists at least one set of adjacents with cardinality `c`
            let e_prime: Vec<(usize, usize, FxIndexSet<usize>)> = E!(g)
                .filter_map(|(x, y)| {
                    // Take set of adjacents with cardinality `c`
                    iter_set::union(
                        Adj!(g, x).filter(|&v| v != y).combinations(c),
                        Adj!(g, y).filter(|&v| v != x).combinations(c),
                    )
                    // If there exists at least one, set the flag to true
                    .inspect(|_| flag = true)
                    // Assign each edge its related sepset
                    .find_map(|z| {
                        if self.test.call(x, y, &z) {
                            Some((x, y, z.into_iter().collect()))
                        } else {
                            None
                        }
                    })
                })
                .collect();

            // Remove d-separated edges of current iteration and collect separation set
            for (x, y, z) in e_prime {
                sepsets.insert((x, y), z.clone());
                sepsets.insert((y, x), z);
                g.del_edge(x, y);
            }

            // Increase size of conditioning set
            c += 1;
        }

        (g, sepsets)
    }

    pub fn call<P>(&self) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Perform skeleton discovery
        let (g, sepsets): (UGraph, _) = self.skeleton();
        // Cast the graph to a partially directed graph
        let mut g = P::new(L!(g), E!(g).map(|(x, y)| (&g[x], &g[y])));

        // Create the set of unshielded triples (x, y, z) in which (x, z) is not d-separated by y
        let triples: Vec<_> = V!(g)
            .flat_map(|y| {
                std::iter::repeat(y)
                    .zip(Adj!(g, y).combinations(2))
                    .map(|(y, xz)| (xz[0], y, xz[1]))
                    .filter(|&(x, y, z)| !g.has_edge(x, z) && !sepsets[&(x, z)].contains(&y))
            })
            .collect();

        // For every unshielded triple ...
        for (x, y, z) in triples {
            // ... if both edges are undirected ...
            if g.has_undirected_edge(x, y) && g.has_undirected_edge(z, y) {
                // ... the triple is a v-structure.
                g.set_directed_edge(x, y);
                g.set_directed_edge(z, y);
            }
        }

        // Orient edges according to orientation rules.
        let g = MeekRules::apply_until_3(g);

        g
    }
}

impl<'a, T> PCStable<'a, T>
where
    T: ConditionalIndependenceTest + Sync,
{
    pub fn par_skeleton<U>(&self) -> (U, SepSets)
    where
        U: UndirectedGraph<Direction = Undirected> + Sync,
    {
        // Set complete graph
        let mut g = U::complete(L!(self.test));
        // Initialize set of separating sets
        let mut sepsets = SepSets::default();
        // Initialize stopping criterion
        let mut flag = true;
        // Initialize size of conditioning set
        let mut c = 0;

        while flag {
            // Unset the flag.
            flag = false;

            // Map and collect each edge in:
            // 1. The edge
            // 2. Its separation set (if any)
            // 3. A flag indicating if exists at least one set of adjacents with cardinality `c`
            let e_prime: Vec<(Option<(usize, usize, FxIndexSet<usize>)>, bool)> = E!(g)
                .collect_vec()
                .into_par_iter()
                .map(|(x, y)| {
                    // Unset the flag.
                    let mut f = false;

                    // Take set of adjacents with cardinality `c`
                    let xyz = iter_set::union(
                        Adj!(g, x).filter(|&v| v != y).combinations(c),
                        Adj!(g, y).filter(|&v| v != x).combinations(c),
                    )
                    // If there exists at least one, set the flag to true
                    .inspect(|_| f = true)
                    // Assign each edge its related sepset
                    .find_map(|z| {
                        if self.test.call(x, y, &z) {
                            Some((x, y, z.into_iter().collect()))
                        } else {
                            None
                        }
                    });

                    (xyz, f)
                })
                .collect();

            // Remove d-separated edges of current iteration and collect separation set
            for (xyz, f) in e_prime {
                if let Some((x, y, z)) = xyz {
                    sepsets.insert((x, y), z.clone());
                    sepsets.insert((y, x), z);
                    g.del_edge(x, y);
                }
                flag |= f;
            }

            // Increase size of conditioning set
            c += 1;
        }

        (g, sepsets)
    }

    pub fn par_call<P>(&self) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
        P::UndirectedGraph: Sync,
    {
        // Perform skeleton discovery.
        let (g, sepsets): (<P as DirectedGraph>::UndirectedGraph, _) = self.par_skeleton();
        // Cast the graph to a partially directed graph
        let mut g = P::new(L!(g), E!(g).map(|(x, y)| (&g[x], &g[y])));

        // Create the set of unshielded triples (x, y, z) in which (x, z) is not d-separated by y
        let triples: Vec<_> = V!(g)
            .flat_map(|y| {
                std::iter::repeat(y)
                    .zip(Adj!(g, y).combinations(2))
                    .map(|(y, xz)| (xz[0], y, xz[1]))
                    .filter(|&(x, y, z)| !g.has_edge(x, z) && !sepsets[&(x, z)].contains(&y))
            })
            .collect();

        // For every unshielded triple ...
        for (x, y, z) in triples {
            // ... if both edges are undirected ...
            if g.has_undirected_edge(x, y) && g.has_undirected_edge(z, y) {
                // ... the triple is a v-structure.
                g.set_directed_edge(x, y);
                g.set_directed_edge(z, y);
            }
        }

        // Orient edges according to orientation rules.
        let g = MeekRules::apply_until_3(g);

        g
    }
}

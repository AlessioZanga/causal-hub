use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    graphs::{
        DirectedGraph, Graph, PartiallyDirected, PartiallyDirectedGraph, UGraph, Undirected,
        UndirectedGraph,
    },
    stats::ConditionalIndependenceTest,
    types::{FxIndexSet, SepSets},
    Adj, Ch, Ne, Pa, E, V,
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
        let mut g = U::complete(self.test.labels());
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

    pub fn rule_1<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            if Pa!(g, x).next().is_none() {
                continue;
            }
            for z in Ne!(g, x).collect_vec() {
                if iter_set::intersection(Adj!(g, z), Pa!(g, x))
                    .next()
                    .is_none()
                {
                    g.set_directed_edge(x, z);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    pub fn rule_2<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            if Pa!(g, x).next().is_none() {
                continue;
            }
            for z in Ch!(g, x).collect_vec() {
                for y in iter_set::intersection(Ne!(g, z), Pa!(g, x)).collect_vec() {
                    g.set_directed_edge(y, z);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    pub fn rule_3<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            for z in Ne!(g, x).collect_vec() {
                let intersection = iter_set::intersection(Ne!(g, z), Pa!(g, x));
                // Look for a non-adjacent couple of parents of `x`
                if intersection
                    .combinations(2)
                    .any(|ab| !g.is_adjacent(ab[0], ab[1]))
                {
                    g.set_directed_edge(z, x);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    pub fn rule_4<P>(mut g: P) -> (P, bool)
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Flag returning `false` if some orientation takes place
        let mut is_closed = true;

        for x in V!(g).collect_vec() {
            if Pa!(g, x).next().is_none() {
                continue;
            }
            for z in Ne!(g, x).collect_vec() {
                if iter_set::intersection(
                    Ne!(g, z),
                    Pa!(g, x).flat_map(|parent| Pa!(g, parent).filter(|&y| !g.is_adjacent(y, x))),
                )
                .next()
                .is_some()
                {
                    g.set_directed_edge(z, x);
                    is_closed = false;
                }
            }
        }

        (g, is_closed)
    }

    #[inline]
    pub fn apply_rules_until_3<P>(mut g: P) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        let mut is_closed = false;

        let (mut _1, mut _2, mut _3);
        while !is_closed {
            (g, _1) = Self::rule_1(g);
            (g, _2) = Self::rule_2(g);
            (g, _3) = Self::rule_3(g);
            is_closed = _1 && _2 && _3;
        }

        g
    }

    #[inline]
    pub fn apply_rules_until_4<P>(mut g: P) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        let mut is_closed = false;

        let (mut _1, mut _2, mut _3, mut _4);
        while !is_closed {
            (g, _1) = Self::rule_1(g);
            (g, _2) = Self::rule_2(g);
            (g, _3) = Self::rule_3(g);
            (g, _4) = Self::rule_4(g);
            is_closed = _1 && _2 && _3 && _4;
        }

        g
    }

    pub fn call<P>(&self) -> P
    where
        P: PartiallyDirectedGraph<Direction = PartiallyDirected>,
    {
        // Perform skeleton discovery
        let (g, sepsets): (UGraph, _) = self.skeleton();
        // Cast the graph to a partially directed graph
        let mut g = P::new(g.labels(), g.edges().map(|(x, y)| (&g[x], &g[y])));

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
        let g = Self::apply_rules_until_3(g);

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
        let mut g = U::complete(self.test.labels());
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
        let mut g = P::new(g.labels(), g.edges().map(|(x, y)| (&g[x], &g[y])));

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
        let g = Self::apply_rules_until_3(g);

        g
    }
}

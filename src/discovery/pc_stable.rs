use itertools::Itertools;
use rayon::prelude::*;

use crate::prelude::*;

#[derive(Clone, Debug)]
/// PC-Stable functor.
pub struct PCStable<'a, T>
where
    T: ConditionalIndependenceTest<'a>,
{
    test: &'a T,
}

impl<'a, T> PCStable<'a, T>
where
    T: ConditionalIndependenceTest<'a>,
{
    /// Construct a new PC-Stable functor.
    pub fn new(test: &'a T) -> Self {
        Self { test }
    }

    /// Private function. It performs skeleton discovery given a test.
    #[inline]
    fn skeleton(&self) -> (UGraph, SepSets) {
        // Set complete graph
        let mut g = UGraph::complete(self.test.labels());
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
                    .find_map(|z| match self.test.call(x, y, &z) {
                        true => Some((x, y, z.into_iter().collect())),
                        _ => None,
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

    /// Private function. It performs parallel skeleton discovery given a test.
    #[inline]
    #[allow(clippy::type_complexity)]
    fn par_skeleton(&self) -> (UGraph, SepSets) {
        // Set complete graph
        let mut g = UGraph::complete(self.test.labels());
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
                .par_bridge()
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
                    .find_map(|z| match self.test.call(x, y, &z) {
                        true => Some((x, y, z.into_iter().collect())),
                        _ => None,
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

    /// Perform skeleton discovery given test.
    #[inline]
    pub fn call_skeleton(&self) -> UGraph {
        self.skeleton().0
    }

    /// Perform parallel skeleton discovery given test.
    #[inline]
    pub fn par_call_skeleton(&self) -> UGraph {
        self.par_skeleton().0
    }

    /// Perform discovery given a test.
    /// Firstly, it performs skeleton discovery and then orients v-structures leveraging discovery implied separation sets.
    #[inline]
    pub fn call(&self) -> PGraph {
        // Perform skeleton discovery
        let (g, sepsets) = self.skeleton();
        // FIXME: Cast the graph to a partially directed graph
        let mut g: PGraph = todo!(); // g.into();

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
            // ... if one of the edges is already directed ...
            if !g.has_undirected_edge(x, y) || !g.has_undirected_edge(z, y) {
                // ... skip this triple.
                continue;
            }
            // Otherwise, the triple is a v-structure.
            g.set_directed_edge(x, y);
            g.set_directed_edge(z, y);
        }

        g
    }

    /// Perform parallel discovery given a test.
    /// Firstly, it performs parallel skeleton discovery and then orients v-structures leveraging discovery implied separation sets.
    #[inline]
    pub fn par_call(&self) -> PGraph {
        // Perform skeleton discovery
        let (g, sepsets) = self.par_skeleton();
        // FIXME: Cast the graph to a partially directed graph
        let mut g: PGraph = todo!(); // g.into();

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
            // ... if one of the edges is already directed ...
            if !g.has_undirected_edge(x, y) || !g.has_undirected_edge(z, y) {
                // ... skip this triple.
                continue;
            }
            // Otherwise, the triple is a v-structure.
            g.set_directed_edge(x, y);
            g.set_directed_edge(z, y);
        }

        g
    }
}

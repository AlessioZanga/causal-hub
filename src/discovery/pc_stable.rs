use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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
        Self {
            test,
        }
    }

    /// Private function. It performs skeleton discovery given a test.
    fn skeleton(&self) -> (Graph, SepSets) {
        // Set complete graph
        let mut g = Graph::complete(self.test.labels());
        // Initialize set of separating sets
        let mut sepsets: SepSets = HashMap::new();
        // Initialize stopping criterion
        let mut flag = true;
        // Initialize size of conditioning set
        let mut c = 0;

        while flag {
            flag = false;
            // For each iteration, initialize the list of to-be-removed edges
            let mut e_prime = Vec::with_capacity(g.size());
            // For every edge ...
            for (x, y) in E!(g) {
                // ... take sets of adjacents with cardinality `c`
                let adj = iter_set::union(
                    Adj!(g, x).filter(|&v| v != y).combinations(c),
                    Adj!(g, y).filter(|&v| v != x).combinations(c),
                );
                // If there is at least one set ...
                for z in adj {
                    // ... continue
                    flag = true;
                    // If such set d-separates `(x, y)` ...
                    if self.test.call(x, y, &z) {
                        // ... remove the edge
                        e_prime.push((x, y));
                        //
                        let z: HashSet<_> = z.into_iter().collect();
                        // Collect `(x, y)` separation set
                        sepsets.insert((x, y), z.clone());
                        sepsets.insert((y, x), z);
                        // Change edge
                        break;
                    }
                }
            }
            // Remove edges of current iteration
            for (x, y) in e_prime {
                g.del_edge_by_index(x, y);
            }
            // Increase size of conditioning set
            c += 1;
        }
        (g, sepsets)
    }

    /// Perform skeleton discovery given test.
    pub fn call_skeleton(&self) -> Graph {
        self.skeleton().0
    }

    /// Perform discovery given a test.
    /// Firstly, it performs skeleton discovery and then orients v-structures leveraging discovery implied separation sets.
    pub fn call(&self) -> PDGraph {
        // Perform skeleton discovery
        let (g, sepsets) = self.skeleton();
        // Cast it to a partially directed graph
        let mut g: PDGraph = g.into();
        // Create the set of unshielded triples
        let mut triples: HashSet<(usize, usize, usize)> = HashSet::new();
        for y in V!(g) {
            for (x, z) in Adj!(g, y)
                .combinations(2)
                .map(|xz| (xz[0], xz[1]))
                .filter(|(x, z)| !g.has_edge_by_index(*x, *z))
            {
                triples.insert((x, y, z));
            }
        }
        // For every unshielded triple ...
        for (x, y, z) in triples.into_iter() {
            // ... if `y` doesn't d-separates `(x, y)` ...
            if !sepsets[&(x, z)].contains(&y) {
                // ... the triple is a v-structure
                g.orient_edge(x, y);
                g.orient_edge(z, y);
            }
        }
        g
    }
}

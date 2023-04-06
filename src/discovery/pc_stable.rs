use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::prelude::*;

/// Skeleton function, as part of PC-Stable algorithm.
/// Inputs: test over data and a complete graph over data nodes.
pub fn skeleton<T, G>(test: &T, mut g: G) -> (G, SepSet, HashSet<(usize, usize, usize)>)
where
    T: ConditionalIndependenceTest,
    G: BaseGraph<Direction = directions::Undirected>,
{
    // Initialize set of separating sets
    let mut sepsets: SepSet = HashMap::new();
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
            // If there is at least of set ...
            for z in adj {
                // ... continue
                flag = true;
                // If such set d-separates `(x, y)` ...
                if test.call(x, y, &z) {
                    // ... remove the edge
                    e_prime.push((x, y));
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
            g.del_edge(x, y);
        }
        // Increase size of conditioning set
        c += 1;
    }
    // Create a set with `g` unshielded triples
    let mut triples: HashSet<(usize, usize, usize)> = HashSet::new();
    for y in V!(g) {
        for (x, z) in Adj!(g, y)
            .combinations(2)
            .map(|xz| (xz[0], xz[1]))
            .filter(|(x, z)| !g.has_edge(*x, *z))
        {
            triples.insert((x, y, z));
        }
    }

    (g, sepsets, triples)
}

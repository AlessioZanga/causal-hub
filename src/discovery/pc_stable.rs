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
    fn skeleton(&self) -> (Graph, SepSets) {
        // Set complete graph
        let mut g = Graph::complete(self.test.labels());
        // Initialize set of separating sets
        let mut sepsets = SepSets::default();
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
                // If there is at least one set with cardinality `c` ...
                for z in adj {
                    // ... continue
                    flag = true;
                    // If such set d-separates `(x, y)` ...
                    if self.test.call(x, y, &z) {
                        // ... remove the edge
                        e_prime.push((x, y));
                        // Collect `(x, y)` separation set
                        let z: FxIndexSet<_> = z.into_iter().collect();
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

    /// Private function. It performs parallel skeleton discovery given a test.
    #[inline]
    fn par_skeleton(&self) -> (Graph, SepSets) {
        // Set complete graph
        let mut g = Graph::complete(self.test.labels());
        // Initialize set of separating sets
        let mut sepsets = SepSets::default();
        // Initialize size of conditioning set
        let mut c = 0;

        'a: loop {
            // Map and collect each edge in:
            // 1. The edge
            // 2. Its separation set (if any)
            // 3. A flag indicating if exists at least one set of adjacents with cardinality `c`
            let to_be_removed: Vec<(usize, usize, Option<FxIndexSet<usize>>, bool)> = E!(g)
                .par_bridge()
                .map(|(x, y)| {
                    // If there exists at least one candidate sepset.
                    let mut flag = false;

                    // Take superset of adjacents with cardinality `c`
                    let sepset = iter_set::union(
                        Adj!(g, x).filter(|&v| v != y).combinations(c),
                        Adj!(g, y).filter(|&v| v != x).combinations(c),
                    )
                    // Assign each subset a flag indicating if it d-separates `(x, y)`
                    .inspect(|_| flag = true)
                    .find_map(|z| match self.test.call(x, y, &z) {
                        true => Some(z.into_iter().collect()),
                        _ => None,
                    });

                    (x, y, sepset, flag)
                })
                .collect();

            // If there are no adjacents with cardinality `c`, then break the iteration
            if to_be_removed.par_iter().all(|(_, _, _, flag)| !*flag) {
                break 'a;
            }

            // Remove d-separated edges of current iteration and collect separation set
            for (x, y, z, _) in to_be_removed {
                match z {
                    Some(z) => {
                        sepsets.insert((x, y), z.clone());
                        sepsets.insert((y, x), z);
                        g.del_edge_by_index(x, y);
                    }
                    _ => continue,
                }
            }

            // Increase size of conditioning set
            c += 1;
        }
        (g, sepsets)
    }

    /// Perform skeleton discovery given test.
    #[inline]
    pub fn call_skeleton(&self) -> Graph {
        self.skeleton().0
    }

    /// Perform parallel skeleton discovery given test.
    #[inline]
    pub fn par_call_skeleton(&self) -> Graph {
        self.par_skeleton().0
    }

    /// Perform discovery given a test.
    /// Firstly, it performs skeleton discovery and then orients v-structures leveraging discovery implied separation sets.
    #[inline]
    pub fn call(&self) -> PDGraph {
        // Perform skeleton discovery
        let (g, sepsets) = self.skeleton();
        // Cast the graph to a partially directed graph
        let mut g: PDGraph = g.into();
        // Create the set of unshielded triples
        let triples = V!(g)
            .flat_map(|y| std::iter::repeat(y).zip(Adj!(g, y).combinations(2)))
            .map(|(y, xz)| (xz[0], y, xz[1]))
            .filter(|(x, _, z)| !g.has_edge_by_index(*x, *z))
            .collect_vec();

        // For every unshielded triple ...
        for (x, y, z) in triples {
            // ... if `y` doesn't d-separates `(x, y)` ...
            if !sepsets[&(x, z)].contains(&y) {
                // ... and both edges are undirected ...
                if !g.has_undirected_edge_by_index(x, y) || !g.has_undirected_edge_by_index(y, z) {
                    continue;
                }
                // ... then the triple is a v-structure
                g.orient_edge(x, y);
                g.orient_edge(z, y);
            }
        }
        g
    }

    /// Perform parallel discovery given a test.
    /// Firstly, it performs parallel skeleton discovery and then orients v-structures leveraging discovery implied separation sets.
    #[inline]
    pub fn par_call(&self) -> PDGraph {
        // Perform skeleton discovery
        let (g, sepsets) = self.par_skeleton();
        // Cast the graph to a partially directed graph
        let mut g: PDGraph = g.into();

        // Create the set of unshielded triples
        let triples: Vec<_> = V!(g)
            .par_bridge()
            .flat_map(|y| {
                std::iter::repeat(y)
                    .zip(Adj!(g, y).combinations(2))
                    .map(|(y, xz)| (xz[0], y, xz[1]))
                    .par_bridge()
                    .filter(|&(x, y, z)| {
                        // TODO: // ... if `y` d-separates `(x, z)` ...
                        !g.has_edge_by_index(x, z) && !sepsets[&(x, z)].contains(&y)
                    })
            })
            .collect();

        // For every unshielded triple ...
        for (x, y, z) in triples {
            // ... if one of the edges is already directed ...
            if !g.has_undirected_edge_by_index(x, y) || !g.has_undirected_edge_by_index(z, y) {
                // ... skip this triple.
                continue;
            }
            // Otherwise, the triple is a v-structure.
            g.orient_edge(x, y);
            g.orient_edge(z, y);
        }

        g
    }
}

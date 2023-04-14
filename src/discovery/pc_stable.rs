use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::prelude::*;

/// Skeleton function, as part of PC-Stable algorithm.
/// `g` must be a complete undirected graph.
pub fn skeleton<T, G>(test: &T, mut g: G) -> (G, SepSets, HashSet<(usize, usize, usize)>)
where
    T: ConditionalIndependenceTest,
    G: BaseGraph<Direction = directions::Undirected>,
{
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

/// Orient v-structures of an undirected graph
pub fn orient_vstructures<D, G, P>(
    g: G,
    sepsets: SepSets,
    triples: HashSet<(usize, usize, usize)>,
) -> P
where
    G: BaseGraph<Data = D, Direction = directions::Undirected> + Into<P>,
    P: PartiallyDirectedGraph<Data = D, Direction = directions::PartiallyDirected>,
{
    let mut g: P = g.into();
    // For every unshielded triple ...
    for (x, y, z) in triples.into_iter() {
        // ... if `y` doesn't d-separates `(x, y)` ...
        if !sepsets.get(&(x, z)).unwrap().iter().any(|&v| v == y) {
            // ... the triple is a v-structure
            g.orient_edge(x, y);
            g.orient_edge(z, y);
        }
    }
    g
}

/// Meek's rule 1
pub fn meek_1<P>(g: &mut P) -> bool
where
    P: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    // Flag returning `false` if some orientation takes place
    let mut is_closed = true;
    for x in V!(g).collect::<Vec<_>>() {
        if Pa!(g, x).next().is_none() {continue}
        for z in Ne!(g, x).collect::<Vec<_>>() {
            if iter_set::intersection(Adj!(g, z), Pa!(g, x))
                    .next()
                    .is_none()
            {
                g.orient_edge(x, z);
                is_closed = false;
            }
        }
    }
    is_closed
}

/// Meek's rule 2
pub fn meek_2<P>(g: &mut P) -> bool
where
    P: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    // Flag returning `false` if some orientation takes place
    let mut is_closed = true;
    for x in V!(g).collect::<Vec<_>>() {
        if Pa!(g, x).next().is_none() {continue}
        for z in Ch!(g, x).collect::<Vec<_>>() {
            for y in iter_set::intersection(Ne!(g, z), Pa!(g, x)).collect::<Vec<_>>() {
                g.orient_edge(y, z);
                is_closed = false;
            }
        }
    }
    is_closed
}

/// Meek's rule 3
pub fn meek_3<P>(g: &mut P) -> bool
where
    P: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    // Flag returning `false` if some orientation takes place
    let mut is_closed = true;
    for x in V!(g).collect::<Vec<_>>() {
        for z in Ne!(g, x).collect::<Vec<_>>() {
            if iter_set::intersection(Ne!(g, z), Pa!(g, x)).count() >= 2 {
                g.orient_edge(z, x);
                is_closed = false;
            }
        }
    }
    is_closed
}

/// Meek's rule 4
pub fn meek_4<P>(g: &mut P) -> bool
where
    P: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    // Flag returning `false` if some orientation takes place
    let mut is_closed = true;
    for x in V!(g).collect::<Vec<_>>() {
        if Pa!(g, x).next().is_none() {continue}
        for z in Ne!(g, x).collect::<Vec<_>>() {
            if iter_set::intersection(Adj!(g, z), Pa!(g, x))
                .next()
                .is_some()
                && iter_set::intersection(Ne!(g, z), Pa!(g, x).flat_map(|parent| Pa!(g, parent)))
                    .next()
                    .is_some()
            {
                g.orient_edge(z, x);
                is_closed = false;
            }
        }
    }
    is_closed
}

/// Meek's procedure
pub fn meek_procedure<P>(mut g: P) -> P
where
    P: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    let mut is_closed = false;
    while !is_closed {
        is_closed = meek_1(&mut g);
        is_closed &= meek_2(&mut g);
        is_closed &= meek_3(&mut g);
        is_closed &= meek_4(&mut g);
    }
    g
}

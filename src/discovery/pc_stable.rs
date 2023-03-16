use crate::prelude::*;
use itertools::Itertools;

/// Skeleton function, as part of PC-Stable algorithm. Inputs: test over data and a complete graph over data nodes.
pub fn skeleton(test: &ChiSquared, mut g: Graph) -> Graph {
    let mut flag = true;
    let mut c = 0;
    while flag {
        flag = false;
        let g_prime = g.clone();
        for (x, y) in g_prime.edges() {
            let adj = iter_set::union(
                g_prime.adjacents(x).filter(|v| *v != y).combinations(c),
                g_prime.adjacents(y).filter(|v| *v != x).combinations(c),
            );
            for z in adj {
                flag = true;
                if !test.call(x, y, &z) {
                    g.del_edge(x, y);
                    break;
                }
            }
        }
        c += 1;
    }
    g
}

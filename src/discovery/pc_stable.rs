use crate::prelude::*;
use itertools::Itertools;

/// Skeleton function, as part of PC-Stable algorithm. Inputs: test over data and a complete graph over data nodes.
pub fn skeleton<T, G>(test: &T, mut g: G) -> G
where
    T: ConditionalIndependenceTest,
    G: BaseGraph<Direction = directions::Undirected>,
{
    let mut flag = true;
    let mut c = 0;
    while flag {
        flag = false;
        let mut e_prime = Vec::with_capacity(g.size());
        for (x, y) in E!(g) {
            let adj = iter_set::union(
                Adj!(g, x).filter(|&v| v != y).combinations(c),
                Adj!(g, y).filter(|&v| v != x).combinations(c),
            );
            for z in adj {
                flag = true;
                if test.call(x, y, &z) {
                    e_prime.push((x, y));
                    break;
                }
            }
        }
        for (x, y) in e_prime {
            g.del_edge(x, y);
        }
        c += 1;
    }
    g
}

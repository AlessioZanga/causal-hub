use std::marker::PhantomData;

use is_sorted::IsSorted;

use crate::{
    graphs::{directions, UndirectedGraph},
    Ne, V,
};

/// Bron-Kerbosh (BK) algorithm with pivoting.
pub struct BronKerboschWithPivoting<G> {
    _g: PhantomData<G>,
}

impl<G> BronKerboschWithPivoting<G>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    /// Find max-cliques of the given graph G.
    #[inline]
    pub fn call(g: &G) -> Vec<Vec<usize>> {
        Self::eval(g, vec![], V!(g).collect(), vec![])
    }

    /// Recursive call of Bron-Kerbosh algorithm.
    fn eval(g: &G, r: Vec<usize>, mut p: Vec<usize>, mut x: Vec<usize>) -> Vec<Vec<usize>> {
        // Assert R, P and X are sorted.
        debug_assert!(r.iter().is_sorted());
        debug_assert!(p.iter().is_sorted());
        debug_assert!(x.iter().is_sorted());

        // If G is null ...
        if g.order() == 0 {
            // ... return.
            return vec![];
        }

        // If P and X are empty ...
        if p.is_empty() && x.is_empty() {
            // ... return R.
            return vec![r];
        }

        // Choose a pivot vertex u in P \cup X with maximum degree.
        let u = *iter_set::union(&p, &x)
            .max_by_key(|&&y| g.get_degree_by_index(y))
            .unwrap();

        // Initialize the results.
        let mut q = vec![];

        // For each v in P \ Ne(g, u);
        for v in iter_set::difference(p.clone(), Ne!(g, u)) {
            // Compute R \cup {v}, P \cap Ne(g, v) and X \cap Ne(g, v).
            let r_prime = iter_set::union(r.clone(), [v]).collect();
            let p_prime = iter_set::intersection(p.clone(), Ne!(g, v)).collect();
            let x_prime = iter_set::intersection(x.clone(), Ne!(g, v)).collect();

            // Recursive call on R \cup {v}, P \cap Ne(g, v) and X \cap Ne(g, v).
            q.extend(Self::eval(g, r_prime, p_prime, x_prime));

            // Compute P \ {v} and X \cup {v}.
            p.remove(p.binary_search(&v).unwrap());
            x.insert(x.binary_search(&v).unwrap_err(), v);
        }

        q
    }
}

/// Alias for the Bron-Kerbosh algorithm.
pub type BK<G> = BronKerboschWithPivoting<G>;

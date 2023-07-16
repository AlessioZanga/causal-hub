use is_sorted::IsSorted;
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    graphs::{directions, UndirectedGraph},
    Ne, V,
};

/// Bron-Kerbosh (BK) algorithm.
///
/// # References
///
/// - [Apurba Das, Seyed-Vahid Sanei-Mehri, and Srikanta Tirthapura. (2020). Shared-memory Parallel Maximal Clique Enumeration from Static and Dynamic Graphs. ACM Trans. Parallel Comput.](https://doi.org/10.1145/3380936)
/// - [Y. -W. Wei, W. -M. Chen and H. -H. Tsai, (2021), Accelerating the Bron-Kerbosch Algorithm for Maximal Clique Enumeration Using GPUs, in IEEE Transactions on Parallel and Distributed Systems](https://doi.org/10.1109/TPDS.2021.3067053)
///
pub struct BronKerbosch<'a, G> {
    g: &'a G,
}

impl<'a, G> BronKerbosch<'a, G>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    /// Construct a new Bron-Kerbosch functor.
    pub const fn new(g: &'a G) -> Self {
        Self { g }
    }

    /// Find max-cliques of the given graph G.
    #[inline]
    pub fn call(&self) -> Vec<Vec<usize>> {
        // FIXME: Get the degeneracy order of G.
        let v = V!(self.g).collect_vec();

        v.iter()
            .enumerate()
            .flat_map(|(i, &v_i)| {
                // Compute Ne(g, v_i).
                let ne_v_i = Ne!(self.g, v_i).collect_vec();
                // Compute V[(i + 1)..|V|] \cap Ne(g, v_i) and V[0..i] \cap Ne(g, v_i).
                let p_i = iter_set::intersection(&v[(i + 1)..], &ne_v_i);
                let x_i = iter_set::intersection(&v[0..i], &ne_v_i);

                self.eval(vec![v_i], p_i.copied().collect(), x_i.copied().collect())
            })
            .collect()
    }

    /// Recursive call of Bron-Kerbosh algorithm.
    fn eval(&self, r: Vec<usize>, mut p: Vec<usize>, mut x: Vec<usize>) -> Vec<Vec<usize>> {
        // Assert R, P and X are sorted.
        debug_assert!(r.iter().is_sorted());
        debug_assert!(p.iter().is_sorted());
        debug_assert!(x.iter().is_sorted());

        // If P and X are empty ...
        if p.is_empty() && x.is_empty() {
            // ... return R.
            return vec![r];
        }

        // Choose a pivot vertex u in P \cup X with maximum |P \cap Ne(g, v)|.
        let u = *iter_set::union(&p, &x)
            .max_by_key(|&&v| {
                let ne_v = Ne!(self.g, v).collect_vec();
                iter_set::intersection(&p, &ne_v).count()
            })
            .unwrap();

        // Compute P \ Ne(g, u).
        let q = iter_set::difference(p.iter().copied(), Ne!(self.g, u)).collect_vec();

        // For each v in P \ Ne(g, u);
        q.iter()
            .flat_map(|v| {
                // Compute Ne(g, v).
                let ne_v = Ne!(self.g, *v).collect_vec();

                // Compute R \cup {v}, P \cap Ne(g, v) and X \cap Ne(g, v).
                let r_prime = iter_set::union(&r, [v]);
                let p_prime = iter_set::intersection(&p, &ne_v);
                let x_prime = iter_set::intersection(&x, &ne_v);

                // Recursive call on R \cup {v}, P \cap Ne(g, v) and X \cap Ne(g, v).
                let q = self.eval(
                    r_prime.copied().collect(),
                    p_prime.copied().collect(),
                    x_prime.copied().collect(),
                );

                // Compute P \ {v} and X \cup {v}.
                p.remove(p.binary_search(v).unwrap());
                x.insert(x.binary_search(v).unwrap_err(), *v);

                q
            })
            .collect()
    }

    /// Find max-cliques of the given graph G in parallel.
    #[inline]
    pub fn par_call(&self) -> Vec<Vec<usize>> {
        // FIXME: Get the degeneracy order of G.
        let v = V!(self.g).collect_vec();

        v.par_iter()
            .enumerate()
            .flat_map(|(i, &v_i)| {
                // Compute Ne(g, v_i).
                let ne_v_i = Ne!(self.g, v_i).collect_vec();
                // Compute V[(i + 1)..|V|] \cap Ne(g, v_i) and V[0..i] \cap Ne(g, v_i).
                let p_i = iter_set::intersection(&v[(i + 1)..], &ne_v_i);
                let x_i = iter_set::intersection(&v[0..i], &ne_v_i);

                self.par_eval(vec![v_i], p_i.copied().collect(), x_i.copied().collect())
            })
            .collect()
    }

    /// Recursive call of Bron-Kerbosh algorithm in parallel.
    fn par_eval(&self, r: Vec<usize>, p: Vec<usize>, x: Vec<usize>) -> Vec<Vec<usize>> {
        // Assert R, P and X are sorted.
        debug_assert!(r.iter().is_sorted());
        debug_assert!(p.iter().is_sorted());
        debug_assert!(x.iter().is_sorted());

        // If P and X are empty ...
        if p.is_empty() && x.is_empty() {
            // ... return R.
            return vec![r];
        }

        // Choose a pivot vertex u in P \cup X with maximum |P \cap Ne(g, v)|.
        let u = *iter_set::union(&p, &x)
            .max_by_key(|&&v| {
                let ne_v = Ne!(self.g, v).collect_vec();
                iter_set::intersection(&p, &ne_v).count()
            })
            .unwrap();

        // Compute P \ Ne(g, u).
        let q = iter_set::difference(p.iter().copied(), Ne!(self.g, u)).collect_vec();

        // For each v in Q ...
        q.par_iter()
            .enumerate()
            .flat_map(|(i, v_i)| {
                // Compute P \ Q[0..i] and X \cup Q[0..i].
                let p_i = iter_set::difference(&p, &q[0..i]);
                let x_i = iter_set::union(&x, &q[0..i]);

                // Compute Ne(g, v).
                let ne_v_i = Ne!(self.g, *v_i).collect_vec();

                // Compute R \cup {v}, P \cap Ne(g, v) and X \cap Ne(g, v).
                let r_prime = iter_set::union(&r, [v_i]);
                let p_prime = iter_set::intersection(p_i, &ne_v_i);
                let x_prime = iter_set::intersection(x_i, &ne_v_i);

                // Recursive call on R \cup {v}, P \cap Ne(g, v) and X \cap Ne(g, v).
                self.par_eval(
                    r_prime.copied().collect(),
                    p_prime.copied().collect(),
                    x_prime.copied().collect(),
                )
            })
            .collect()
    }
}

/// Alias for the Bron-Kerbosh algorithm.
pub type BK<'a, G> = BronKerbosch<'a, G>;

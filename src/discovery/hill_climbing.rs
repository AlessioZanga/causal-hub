use std::{
    collections::{BTreeSet, HashSet},
    marker::PhantomData,
};

use itertools::iproduct;
use log::debug;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

use super::{score_types, DecomposableScoringCriterion, PriorKnowledge, ScoringCriterion};
use crate::{
    data::DataSet,
    graphs::PathGraph,
    prelude::{directions, BaseGraph, DirectedGraph, BFS},
    Ch, Pa, E, V,
};

/// Local cache type.
type C<K> = FxHashMap<K, f64>;
/// Local cache update type.
type CU<K> = Vec<(K, f64)>;
/// Local edge key cache type.
type KE = (usize, Vec<usize>);

/// Local edge space type
type E = BTreeSet<(usize, usize)>;
/// Local operations edge space type.
type ES = (
    E, // To-be-added space,
    E, // To-be-deleted space,
    E, // To-be-reversed space.
);

#[derive(Clone, Copy, Debug)]
/// Local edge pseudo-enumerator for generics.
struct Op;
/// Set value of constants.
impl Op {
    /// Add edge operation.
    const ADD: usize = 0;
    /// Delete edge operation.
    const DEL: usize = 1;
    /// Reverse edge operation.
    const REV: usize = 2;
}
/// Local action (operation, edge) type.
type A = Option<(usize, usize, usize)>;

#[derive(Clone, Debug)]
/// Hill-climbing functor.
pub struct HillClimbing<D, K, G, S, ST, const PARALLEL: bool>
where
    D: DataSet,
    K: PriorKnowledge,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    max_iter: usize,
    _d: PhantomData<D>,
    _k: PhantomData<K>,
    g: Option<G>,
    s: S,
}

impl<D, K, G, S, ST, const PARALLEL: bool> HillClimbing<D, K, G, S, ST, PARALLEL>
where
    D: DataSet,
    K: PriorKnowledge,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    /// Construct a new hill-climbing functor given the scoring criterion $\mathcal{S}$.
    #[inline]
    pub const fn new(s: S) -> Self {
        Self {
            max_iter: usize::MAX,
            _d: PhantomData,
            _k: PhantomData,
            g: None,
            s,
        }
    }

    /// Set initial graph $\mathcal{G}$.
    #[inline]
    pub fn with_initial_graph(mut self, g: G) -> Self {
        // Set initial graph.
        self.g = Some(g);

        self
    }

    /// Set max iterations.
    #[inline]
    pub fn with_max_iter(mut self, max_iter: usize) -> Self {
        // Set hyper parameter.
        self.max_iter = max_iter;

        self
    }
}

impl<D, K, G, S, ST, const PARALLEL: bool> HillClimbing<D, K, G, S, ST, PARALLEL>
where
    D: DataSet,
    K: PriorKnowledge,
    G: BaseGraph,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    /// Apply edge operation to given graph.
    #[inline]
    fn apply(mut g: G, x: usize, y: usize, a: usize) -> G {
        // Apply operation.
        match a {
            Op::ADD => g.add_edge(x, y),
            Op::DEL => g.del_edge(x, y),
            Op::REV => g.del_edge(x, y) && g.add_edge(y, x),
            _ => panic!("Unknown operation code"),
        };

        // Log apply operation.
        debug!(
            "apply op: {}({}, {})",
            match a {
                Op::ADD => "Add",
                Op::DEL => "Del",
                Op::REV => "Rev",
                _ => panic!("Unknown operation code"),
            },
            g.label(x),
            g.label(y),
        );

        g
    }

    /// Update edge space for each edge operation.
    #[inline]
    fn update((mut add, mut del, mut rev): ES, x: usize, y: usize, a: usize) -> ES {
        // Apply operation.
        match a {
            Op::ADD => {
                add.remove(&(x, y));
                add.remove(&(y, x));
                del.insert((x, y));
                rev.insert((x, y));
            }
            Op::DEL => {
                add.insert((x, y));
                add.insert((y, x));
                del.remove(&(x, y));
                rev.remove(&(x, y));
            }
            Op::REV => {
                del.remove(&(x, y));
                del.insert((y, x));
                rev.remove(&(x, y));
                rev.insert((y, x));
            }
            _ => panic!("Unknown operation code"),
        };

        (add, del, rev)
    }
}

impl<D, K, G, S, ST, const PARALLEL: bool> HillClimbing<D, K, G, S, ST, PARALLEL>
where
    D: DataSet,
    K: PriorKnowledge,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    #[inline]
    fn init(&self, d: &D, k: &K) -> (G, ES) {
        // Check if initial graph has been provided.
        let mut g = match self.g.as_ref() {
            // If initial graph is provided ...
            Some(g) => g.clone(),
            // If no initial graph is provided, initialize an empty one.
            None => G::empty(d.labels()),
        };

        // Check coherence with data set ...
        assert!(
            g.labels().eq(d.labels()),
            "Graph labels must be equal to data set labels"
        );
        // Check coherence of graph and prior knowledge.
        assert!(
            g.labels().eq(k.labels()),
            "Graph labels must be equal to prior knowledge labels"
        );

        // Check that every edge in the graph is not in forbidden.
        assert!(
            !E!(g).any(|(x, y)| k.has_forbidden(x, y)),
            "Graph edges must not be in the forbidden list"
        );
        // Check that every edge in the required list is in the graph.
        assert!(k
            .required()
            .iter()
            .all(|&(x, y)| g.has_edge(x, y) || g.add_edge(x, y)));

        // Check acyclicity.
        assert!(g.is_acyclic(), "Prior knowledge must not add any cycle");

        // Get number of variables.
        let n = d.labels().len();
        // Get current edge set.
        let e: HashSet<_> = E!(g).collect();

        // Initialize potential edges to be added.
        let add: BTreeSet<_> = iproduct!(0..n, 0..n)
            // Remove any edge (X, Y) s.t. X == Y, is present in the initial graph, or is in the forbidden list.
            .filter(|&(x, y)| {
                x != y && !e.contains(&(x, y)) && !e.contains(&(y, x)) && !k.has_forbidden(x, y)
            })
            .collect();

        // Initialize potential edges to be deleted.
        let del: BTreeSet<_> = e
            .iter()
            .filter(|(x, y)| !k.has_required(*x, *y))
            .cloned()
            .collect(); // Remove any edge in the required list.

        // Initialize potential edges to be reversed.
        let rev = del
            .iter()
            .filter(|(x, y)| !k.has_forbidden(*y, *x))
            .cloned()
            .collect(); // Remove any reversed edge in the forbidden list.

        (g, (add, del, rev))
    }

    /// Check if edge operation is consistent with prior knowledge and acyclicity.
    #[inline]
    fn is_valid<const OP: usize>(g: &G, x: usize, y: usize) -> bool {
        // Check validity depending on operation.
        let is_valid = match OP {
            // (X, Y) not in F, pi(Y, X) not in G.
            Op::ADD => !g.has_path(y, x),
            // (X, Y) in R.
            Op::DEL => true,
            // (Y, X) not in F, (X, Y) in R, pi(X, Y) not in G.
            Op::REV => !Ch!(g, x)
                .filter(|&z| z != y)
                .any(|z| BFS::from((g, z)).any(|w| w == y)),
            // Unknown operation code.
            _ => panic!("Unknown operation code"),
        };

        // Check if invalid.
        if !is_valid {
            // Log invalid.
            debug!(
                "op: {}({}, {}), invalid",
                match OP {
                    Op::ADD => "Add",
                    Op::DEL => "Del",
                    Op::REV => "Rev",
                    _ => panic!("Unknown operation code"),
                },
                g.label(x),
                g.label(y),
            );
        }

        is_valid
    }
}

/* Implement Hill-Climbing for Decomposable Scoring Criteria */
impl<D, K, G, S, const PARALLEL: bool> HillClimbing<D, K, G, S, score_types::Decomposable, PARALLEL>
where
    D: DataSet,
    K: PriorKnowledge,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    S: DecomposableScoringCriterion<D, G>,
{
    /// Compute delta score, if not already in cache, returning cache update.
    #[inline]
    fn cache(&self, c: &C<KE>, d: &D, x: usize, z: &[usize]) -> (f64, CU<KE>) {
        // Check if score is already in cache.
        match c.get(&(x, z.to_vec())) {
            // If so, return cached values.
            Some(s) => (*s, CU::default()),
            // If not, then ...
            None => {
                // Compute vertex score.
                let s = DecomposableScoringCriterion::call(&self.s, d, x, z);

                (s, CU::from_iter([((x, z.to_vec()), s)]))
            }
        }
    }

    /// Evaluate delta score of edge operation on given graph.
    #[inline]
    fn eval<const OP: usize>(&self, c: &C<KE>, d: &D, g: &G, x: usize, y: usize) -> (f64, CU<KE>) {
        // Get current Y score.
        let mut pa_y: Vec<_> = Pa!(g, y).collect();
        let (s_y, mut c_y) = self.cache(c, d, y, &pa_y);
        // Compute delta score depending on operation.
        let (delta_star, c_star) = match OP {
            Op::ADD => {
                // Add X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap_err();
                pa_y.insert(i, x);
                // Compute delta score and merge cache.
                let (s_y_star, c_y_star) = self.cache(c, d, y, &pa_y);
                // Accumulate cache updates.
                c_y.extend(c_y_star.into_iter());

                (s_y_star - s_y, c_y)
            }
            Op::DEL => {
                // Remove X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute delta score and merge cache.
                let (s_y_star, c_y_star) = self.cache(c, d, y, &pa_y);
                // Merge cache updates.
                c_y.extend(c_y_star.into_iter());

                (s_y_star - s_y, c_y)
            }
            Op::REV => {
                // Get current X score.
                let mut pa_x: Vec<_> = Pa!(g, x).collect();
                let (s_x, c_x) = self.cache(c, d, x, &pa_x);
                // Merge cache updates.
                c_y.extend(c_x.into_iter());
                // Add Y in-place by leveraging Pa(G, X) order.
                let i = pa_x.binary_search(&y).unwrap_err();
                pa_x.insert(i, y);
                let (s_x_star, c_x_star) = self.cache(c, d, x, &pa_x);
                // Merge cache updates.
                c_y.extend(c_x_star.into_iter());
                // Remove X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                let (s_y_star, c_y_star) = self.cache(c, d, y, &pa_y);
                // Merge cache updates.
                c_y.extend(c_y_star.into_iter());

                ((s_x_star - s_x) + (s_y_star - s_y), c_y)
            }
            _ => panic!("Unknown operation code"),
        };

        // Log current operation delta.
        debug!(
            "op: {}({}, {}), delta: {}",
            match OP {
                Op::ADD => "Add",
                Op::DEL => "Del",
                Op::REV => "Rev",
                _ => panic!("Unknown operation code"),
            },
            g.label(x),
            g.label(y),
            delta_star
        );

        (delta_star, c_star)
    }

    /// Search for best operation given current graph and edges space.
    #[inline]
    fn search<const OP: usize>(
        &self,
        (op, delta): (A, f64),
        mut c: C<KE>,
        d: &D,
        g: &G,
        edges: &E,
    ) -> (A, f64, C<KE>) {
        // Select operation with best delta score, while merging cache updates.
        let best_merge =
            |(op, (delta, mut u_star)): (A, (f64, CU<KE>)),
             (op_star, (delta_star, c_star)): (A, (f64, CU<KE>))| {
                // Merge cache updates.
                u_star.extend(c_star.into_iter());
                // Check if difference is meaningful.
                let diff = delta_star - delta;
                let diff = diff * !(f64::abs(diff) < f64::sqrt(f64::EPSILON)) as u8 as f64;
                // Return best operation.
                match diff > 0. {
                    true => (op_star, (delta_star, u_star)),
                    false => (op, (delta, u_star)),
                }
            };

        // For each possible edge operation ...
        let (op, (delta, u)) = match PARALLEL {
            // Search in parallel.
            true => edges
                .par_iter()
                // Check if operation is valid.
                .filter(|(x, y)| Self::is_valid::<OP>(g, *x, *y))
                // Compute current operation delta score and cache updates.
                .map(|(x, y)| (Some((*x, *y, OP)), self.eval::<OP>(&c, d, g, *x, *y)))
                // Check if operation improves current solution.
                .reduce(|| (op, (delta, CU::default())), best_merge),
            // Same as before but sequentially.
            false => edges
                .iter()
                .filter(|(x, y)| Self::is_valid::<OP>(g, *x, *y))
                .map(|(x, y)| (Some((*x, *y, OP)), self.eval::<OP>(&c, d, g, *x, *y)))
                .fold((op, (delta, CU::default())), best_merge),
        };

        // Merge cache updates.
        c.extend(u.into_iter());

        (op, delta, c)
    }

    /// Perform discovery given data set $\mathbf{D}$ and prior knowledge $\mathbf{K}$.
    pub fn call(&self, d: &D, k: &K) -> G {
        // Initialize delta scores cache.
        let mut c = C::<KE>::default();

        // Initialize graph from D and K.
        let (mut g, (mut add, mut del, mut rev)) = self.init(d, k);
        // Compute the initial score.
        let mut s_g: f64 = V!(g)
            // For each vertex.
            .map(|x| {
                // Get vertex parents.
                let z: Vec<_> = Pa!(g, x).collect();
                // Compute vertex score.
                let s = DecomposableScoringCriterion::call(&self.s, d, x, &z);
                // Insert into the cache.
                c.insert((x, z), s);

                s
            })
            // Sum the partial scores.
            .sum();

        // Initialize iterations counter.
        let mut i = 0;
        // Initialize the increasing score flag.
        let mut flag = true;

        // While score increase and at maximum `max_iter` times.
        while flag && i < self.max_iter {
            // Reset the flag.
            flag = false;
            // Log current iteration.
            debug!("i: {}, max_iter: {}", i, self.max_iter);

            // Initialize current best operation.
            let (mut op, mut delta) = (None, 0.);

            // For each possible edge addition ...
            (op, delta, c) = self.search::<{ Op::ADD }>((op, delta), c, d, &g, &add);
            // For each possible edge deletion ...
            (op, delta, c) = self.search::<{ Op::DEL }>((op, delta), c, d, &g, &del);
            // For each possible edge reversal ...
            (op, delta, c) = self.search::<{ Op::REV }>((op, delta), c, d, &g, &rev);

            // If best operation exists.
            if let Some((x, y, a)) = op {
                // Apply operation to current solution.
                (g, s_g) = (Self::apply(g, x, y, a), s_g + delta);
                // Update search space.
                (add, del, rev) = Self::update((add, del, rev), x, y, a);
                // Set the flag.
                flag = true;
            }

            // Increment counter.
            i += 1;
        }

        g
    }
}

/* Implement Hill-Climbing for Non-Decomposable Scoring Criteria */
impl<D, K, G, S, const PARALLEL: bool>
    HillClimbing<D, K, G, S, score_types::NonDecomposable, PARALLEL>
where
    D: DataSet,
    K: PriorKnowledge,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    S: ScoringCriterion<D, G, ScoreType = score_types::NonDecomposable>,
{
    /// Compute delta score, if not already in cache, returning cache update.
    #[inline]
    fn cache(&self, c: &C<G>, d: &D, g: &G) -> (f64, CU<G>) {
        // Check if score is already in cache.
        match c.get(g) {
            // If so, return cached values.
            Some(s) => (*s, CU::default()),
            // If not, then ...
            None => {
                // Compute vertex score.
                let s = ScoringCriterion::call(&self.s, d, g);

                (s, CU::from_iter([(g.clone(), s)]))
            }
        }
    }

    /// Evaluate delta score of edge operation on given graph.
    #[inline]
    fn eval<const OP: usize>(&self, c: &C<G>, d: &D, g: &G, x: usize, y: usize) -> (f64, CU<G>) {
        // Get current Y score.
        let (s_g, mut c_g) = self.cache(c, d, g);
        // Compute delta score depending on operation.
        let (delta_star, c_star) = match OP {
            Op::ADD => {
                // Add X in-place by leveraging Pa(G, Y) order.
                let mut g_star = g.clone();
                g_star.add_edge(x, y);
                // Compute delta score and merge cache.
                let (s_g_star, c_g_star) = self.cache(c, d, &g_star);
                // Accumulate cache updates.
                c_g.extend(c_g_star.into_iter());

                (s_g_star - s_g, c_g)
            }
            Op::DEL => {
                // Remove X in-place by leveraging Pa(G, Y) order.
                let mut g_star = g.clone();
                g_star.del_edge(x, y);
                // Compute delta score and merge cache.
                let (s_g_star, c_g_star) = self.cache(c, d, &g_star);
                // Merge cache updates.
                c_g.extend(c_g_star.into_iter());

                (s_g_star - s_g, c_g)
            }
            Op::REV => {
                // Reverse X in-place by leveraging Pa(G, Y) order.
                let mut g_star = g.clone();
                g_star.del_edge(x, y);
                g_star.add_edge(y, x);
                // Compute delta score and merge cache.
                let (s_g_star, c_g_star) = self.cache(c, d, &g_star);
                // Merge cache updates.
                c_g.extend(c_g_star.into_iter());

                (s_g_star - s_g, c_g)
            }
            _ => panic!("Unknown operation code"),
        };

        // Log current operation delta.
        debug!(
            "op: {}({}, {}), delta: {}",
            match OP {
                Op::ADD => "Add",
                Op::DEL => "Del",
                Op::REV => "Rev",
                _ => panic!("Unknown operation code"),
            },
            g.label(x),
            g.label(y),
            delta_star
        );

        (delta_star, c_star)
    }

    /// Search for best operation given current graph and edges space.
    #[inline]
    fn search<const OP: usize>(
        &self,
        (op, delta): (A, f64),
        mut c: C<G>,
        d: &D,
        g: &G,
        edges: &E,
    ) -> (A, f64, C<G>) {
        // Select operation with best delta score, while merging cache updates.
        let best_merge =
            |(op, (delta, mut u_star)): (A, (f64, CU<G>)),
             (op_star, (delta_star, c_star)): (A, (f64, CU<G>))| {
                // Merge cache updates.
                u_star.extend(c_star.into_iter());
                // Check if difference is meaningful.
                let diff = delta_star - delta;
                let diff = diff * !(f64::abs(diff) < f64::sqrt(f64::EPSILON)) as u8 as f64;
                // Return best operation.
                match diff > 0. {
                    true => (op_star, (delta_star, u_star)),
                    false => (op, (delta, u_star)),
                }
            };

        // For each possible edge operation ...
        let (op, (delta, u)) = match PARALLEL {
            // Search in parallel.
            true => edges
                .par_iter()
                // Check if operation is valid.
                .filter(|(x, y)| Self::is_valid::<OP>(g, *x, *y))
                // Compute current operation delta score and cache updates.
                .map(|(x, y)| (Some((*x, *y, OP)), self.eval::<OP>(&c, d, g, *x, *y)))
                // Check if operation improves current solution.
                .reduce(|| (op, (delta, CU::default())), best_merge),
            // Same as before but sequentially.
            false => edges
                .iter()
                .filter(|(x, y)| Self::is_valid::<OP>(g, *x, *y))
                .map(|(x, y)| (Some((*x, *y, OP)), self.eval::<OP>(&c, d, g, *x, *y)))
                .fold((op, (delta, CU::default())), best_merge),
        };

        // Merge cache updates.
        c.extend(u.into_iter());

        (op, delta, c)
    }

    /// Perform discovery given data set $\mathbf{D}$ and prior knowledge $\mathbf{K}$.
    pub fn call(&self, d: &D, k: &K) -> G {
        // Initialize delta scores cache.
        let mut c = C::<G>::default();

        // Initialize graph from D and K.
        let (mut g, (mut add, mut del, mut rev)) = self.init(d, k);
        // Compute the initial score.
        let mut s_g: f64 = ScoringCriterion::call(&self.s, d, &g);
        // Update cache.
        c.insert(g.clone(), s_g);

        // Initialize iterations counter.
        let mut i = 0;
        // Initialize the increasing score flag.
        let mut flag = true;

        // While score increase and at maximum `max_iter` times.
        while flag && i < self.max_iter {
            // Reset the flag.
            flag = false;
            // Log current iteration.
            debug!("i: {}, max_iter: {}", i, self.max_iter);

            // Initialize current best operation.
            let (mut op, mut delta) = (None, 0.);

            // For each possible edge addition ...
            (op, delta, c) = self.search::<{ Op::ADD }>((op, delta), c, d, &g, &add);
            // For each possible edge deletion ...
            (op, delta, c) = self.search::<{ Op::DEL }>((op, delta), c, d, &g, &del);
            // For each possible edge reversal ...
            (op, delta, c) = self.search::<{ Op::REV }>((op, delta), c, d, &g, &rev);

            // If best operation exists.
            if let Some((x, y, a)) = op {
                // Apply operation to current solution.
                (g, s_g) = (Self::apply(g, x, y, a), s_g + delta);
                // Update search space.
                (add, del, rev) = Self::update((add, del, rev), x, y, a);
                // Set the flag.
                flag = true;
            }

            // Increment counter.
            i += 1;
        }

        g
    }
}

/// Alias for the single-thread Hill-Climbing algorithm.
pub type HC<D, K, G, S, ST> = HillClimbing<D, K, G, S, ST, false>;
/// Alias for the multi-thread Hill-Climbing algorithm.
pub type ParallelHC<D, K, G, S, ST> = HillClimbing<D, K, G, S, ST, true>;

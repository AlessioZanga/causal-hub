use std::{
    collections::{hash_map::Entry, BTreeSet},
    marker::PhantomData,
};

use itertools::iproduct;
use log::debug;
use rustc_hash::FxHashMap;

use super::{score_types, DecomposableScoringCriterion, ScoringCriterion};
use crate::{
    data::DataSet,
    graphs::PathGraph,
    prelude::{directions, BaseGraph, DirectedGraph, BFS},
    Ch, Pa, E, V,
};

/// Local cache type.
type C = FxHashMap<(usize, Vec<usize>), f64>;

/// Local edge set type
type E = BTreeSet<(usize, usize)>;

/// Local edge space type.
type ES = (
    E, // To-be-added space,
    E, // To-be-deleted space,
    E, // To-be-reversed space.
);

#[derive(Clone, Copy, Debug)]
/// Local edge pseudo-enumerator for generics.
struct Op;

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
pub struct HillClimbing<D, K, G, S, ST>
where
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    max_iter: usize,
    _d: PhantomData<D>,
    _k: PhantomData<K>,
    g: Option<G>,
    s: S,
}

impl<D, K, G, S, ST> HillClimbing<D, K, G, S, ST>
where
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    /// Construct a new hill-climbing functor given the scoring criterion $\mathcal{S}$.
    pub fn new(s: S) -> Self {
        Self {
            max_iter: usize::MAX,
            _d: Default::default(),
            _k: Default::default(),
            g: None,
            s,
        }
    }

    /// Set initial graph $\mathcal{G}$.
    pub fn with_initial_graph(mut self, g: G) -> Self {
        // Set initial graph.
        self.g = Some(g);

        self
    }

    /// Set max iterations.
    pub fn with_max_iter(mut self, max_iter: usize) -> Self {
        // Set hyper parameter.
        self.max_iter = max_iter;

        self
    }
}

impl<D, K, G, S, ST> HillClimbing<D, K, G, S, ST>
where
    G: BaseGraph,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    /// Apply edge operation to given graph.
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

impl<D, K, G, S, ST> HillClimbing<D, K, G, S, ST>
where
    D: DataSet,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    fn init(&self, d: &D, _k: &K) -> G {
        // Check if initial graph has been provided.
        let g = match self.g.clone() {
            // If initial graph is provided ...
            Some(g) => {
                // ... check coherence with data set ...
                assert!(
                    g.labels().eq(d.labels()),
                    "Graph labels must be equal to data set labels"
                );
                // ... and acyclicity.
                assert!(g.is_acyclic(), "Graph must be acyclic");

                g
            }
            // If no initial graph is provided, initialize an empty one.
            None => G::empty(d.labels()),
        };

        // TODO: Check coherence of graph and prior knowledge.
        // TODO: Add prior knowledge to the graph and check acyclicity.

        g
    }

    /// Check if edge operation is consistent with prior knowledge and acyclicity.
    fn is_valid<const OP: usize>(_k: &K, g: &G, x: usize, y: usize) -> bool {
        // Check validity depending on operation. TODO: Check prior knowledge.
        let is_valid = match OP {
            // (X, Y) not in E, pi(Y, X) not in G.
            Op::ADD => !g.has_path(y, x),
            // (X, Y) in E.
            Op::DEL => true,
            // (Y, X) not in E, (X, Y) in E, pi(X, Y) not in G.
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

impl<D, K, G, S> HillClimbing<D, K, G, S, score_types::Decomposable>
where
    D: DataSet,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    S: DecomposableScoringCriterion<D, G>,
{
    /// Compute delta score, if not already in cache.
    fn cache(&self, c: &mut C, d: &D, x: usize, z: &[usize]) -> f64 {
        // Check if score is already in cache.
        match c.entry((x, z.to_vec())) {
            // If so, return cached values.
            Entry::Occupied(e) => *e.get(),
            // If not, then ...
            Entry::Vacant(e) => {
                // Compute vertex score.
                let s = DecomposableScoringCriterion::call(&self.s, d, x, z);
                // Insert into the cache.
                e.insert(s);

                s
            }
        }
    }

    /// Evaluate delta score of edge operation on given graph.
    fn eval<const OP: usize>(&self, c: &mut C, d: &D, g: &G, x: usize, y: usize) -> f64 {
        // Get current Y score.
        let mut pa_y: Vec<_> = Pa!(g, y).collect();
        let s_y = self.cache(c, d, y, &pa_y);
        // Compute delta score depending on operation.
        let delta = match OP {
            Op::ADD => {
                // Add X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap_err();
                pa_y.insert(i, x);
                // Compute delta score.
                self.cache(c, d, y, &pa_y) - s_y
            }
            Op::DEL => {
                // Remove X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute delta score.
                self.cache(c, d, y, &pa_y) - s_y
            }
            Op::REV => {
                // Get current X score.
                let mut pa_x: Vec<_> = Pa!(g, x).collect();
                let s_x = self.cache(c, d, x, &pa_x);
                // Add Y in-place by leveraging Pa(G, X) order.
                let i = pa_x.binary_search(&y).unwrap_err();
                pa_x.insert(i, y);
                let s_star_x = self.cache(c, d, x, &pa_x);
                // Remove X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                let s_star_y = self.cache(c, d, y, &pa_y);
                // Compute delta score.
                (s_star_x - s_x) + (s_star_y - s_y)
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
            delta
        );

        delta
    }

    /// Search for best operation given current graph and edges space.
    fn search<const OP: usize>(
        &self,
        (mut op, mut delta): (A, f64),
        c: &mut C,
        d: &D,
        k: &K,
        g: &G,
        edges: &E,
    ) -> (A, f64) {
        // For each possible edge operation ...
        for &(x, y) in edges {
            // Check if operation is valid.
            if !Self::is_valid::<OP>(k, g, x, y) {
                continue;
            }
            // Compute current operation delta score.
            let delta_star = self.eval::<OP>(c, d, g, x, y);
            // Check if operation improves current solution.
            if delta_star > delta {
                // Set best operation.
                (op, delta) = (Some((x, y, OP)), delta_star);
            }
        }

        (op, delta)
    }

    /// Perform discovery given data set $\mathbf{D}$ and prior knowledge $\mathbf{K}$.
    pub fn call(&self, d: &D, k: &K) -> G {
        // Get number of variables.
        let n = d.labels().len();
        // Initialize delta scores cache.
        let mut c = C::default();

        // Initialize graph from D and K.
        let mut g = self.init(d, k);
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

        // Get current edge set.
        let e: BTreeSet<_> = E!(g).collect();
        let add: BTreeSet<_> = iproduct!(0..n, 0..n).filter(|(x, y)| x != y).collect();
        // Initialize potential edges to be added.
        let mut add = &(&add - &e) - &e.iter().cloned().map(|(x, y)| (y, x)).collect();
        // Initialize potential edges to be deleted.
        let mut del = e.clone();
        // Initialize potential edges to be reversed.
        let mut rev = e;

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
            (op, delta) = self.search::<{ Op::ADD }>((op, delta), &mut c, d, k, &g, &add);
            // For each possible edge deletion ...
            (op, delta) = self.search::<{ Op::DEL }>((op, delta), &mut c, d, k, &g, &del);
            // For each possible edge reversal ...
            (op, delta) = self.search::<{ Op::REV }>((op, delta), &mut c, d, k, &g, &rev);

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

impl<D, K, G, S> HillClimbing<D, K, G, S, score_types::NonDecomposable>
where
    D: DataSet,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    S: ScoringCriterion<D, G, ScoreType = score_types::NonDecomposable>,
{
    /// Perform discovery given data set $\mathbf{D}$ and prior knowledge $\mathbf{K}$.
    pub fn call(&self, _d: &D, _k: &K) -> G {
        todo!() // FIXME:
    }
}

/// Alias for the Hill-Climbing algorithm.
pub type HC<D, K, G, S, ST> = HillClimbing<D, K, G, S, ST>;

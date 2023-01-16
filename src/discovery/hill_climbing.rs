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
    graphs::{DefaultGraph, PathGraph},
    prelude::{directions, BaseGraph, DirectedGraph, BFS},
    Ch, Pa, E, V,
};

/// Local cache type.
type C = FxHashMap<(usize, Vec<usize>), f64>;

#[derive(Clone, Copy, Debug)]
struct Op;

impl Op {
    const ADD: usize = 0;
    const DEL: usize = 1;
    const REV: usize = 2;
}

#[derive(Clone, Debug)]
pub struct HillClimbing<D, K, G, S, ST>
where
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    max_iter: usize,
    max_degree: usize,
    _d: PhantomData<D>,
    _k: PhantomData<K>,
    g: Option<G>,
    s: S,
}

impl<D, K, G, S, ST> HillClimbing<D, K, G, S, ST>
where
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    pub fn new(s: S) -> Self {
        Self {
            max_iter: usize::MAX,
            max_degree: usize::MAX,
            _d: Default::default(),
            _k: Default::default(),
            g: None,
            s,
        }
    }

    pub fn with_initial_graph(mut self, g: G) -> Self {
        // Set initial graph.
        self.g = Some(g);

        self
    }

    pub fn with_max_iter(mut self, max_iter: usize) -> Self {
        // Set hyper parameter.
        self.max_iter = max_iter;

        self
    }

    pub fn with_max_degree(mut self, max_degree: usize) -> Self {
        // Set hyper parameter.
        self.max_degree = max_degree;

        self
    }
}

impl<D, K, G, S, ST> HillClimbing<D, K, G, S, ST>
where
    D: DataSet,
    G: DefaultGraph + PathGraph,
    S: ScoringCriterion<D, G, ScoreType = ST>,
{
    fn init(&self, d: &D, k: &K) -> G {
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
}

impl<D, K, G, S> HillClimbing<D, K, G, S, score_types::Decomposable>
where
    D: DataSet,
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph + PathGraph,
    S: DecomposableScoringCriterion<D, G>,
{
    fn cache(&self, c: &mut C, d: &D, x: usize, z: &[usize]) -> f64 {
        // Check if score is already in cache.
        match c.entry((x, z.iter().cloned().collect())) {
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

    fn eval<const A: usize>(&self, c: &mut C, d: &D, g: &G, x: usize, y: usize) -> f64 {
        // Get Y parents.
        let mut pa_y: Vec<_> = Pa!(g, y).collect();
        // Get current Y score.
        let s_y = self.cache(c, d, y, &pa_y);

        // Compute score delta depending on operation.
        let delta = match A {
            Op::ADD => {
                // Add X in-place leveraging sorted Pa(G, Y).
                let i = pa_y.binary_search(&x).unwrap_err();
                pa_y.insert(i, x);
                // Compute delta score.
                self.cache(c, d, y, &pa_y) - s_y
            }
            Op::DEL => {
                // Remove X in-place leveraging sorted Pa(G, Y).
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute delta score.
                self.cache(c, d, y, &pa_y) - s_y
            }
            Op::REV => {
                // Get X parents.
                let mut pa_x: Vec<_> = Pa!(g, x).collect();
                // Get current X score.
                let s_x = self.cache(c, d, x, &pa_x);

                // Add Y in-place leveraging sorted Pa(G, X).
                let i = pa_x.binary_search(&y).unwrap_err();
                pa_x.insert(i, y);
                // Compute Y score.
                let s_star_x = self.cache(c, d, x, &pa_x);

                // Remove X in-place leveraging sorted Pa(G, Y).
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute Y score.
                let s_star_y = self.cache(c, d, y, &pa_y);

                // Compute delta score.
                (s_star_x - s_x) + (s_star_y - s_y)
            }
            _ => panic!("Unknown operation code"),
        };

        // Log current operation delta.
        debug!(
            "op: {}({}, {}), delta: {}",
            match A {
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

    fn is_valid<const A: usize>(g: &G, x: usize, y: usize, k: &K) -> bool {
        // Check validity depending on operation. TODO: Check prior knowledge.
        let is_valid = match A {
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
                match A {
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

    fn update(
        add: &mut BTreeSet<(usize, usize)>,
        del: &mut BTreeSet<(usize, usize)>,
        rev: &mut BTreeSet<(usize, usize)>,
        x: usize,
        y: usize,
        a: usize,
    ) {
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
    }

    pub fn call(&self, d: &D, k: &K) -> G {
        // Get number of variables.
        let n = d.labels().len();
        // Initialize delta scores cache.
        let mut c = C::default();

        // Initialize graph from D and K.
        let mut g_max = self.init(d, k);
        // Compute the initial score.
        let mut s_g_max: f64 = V!(g_max)
            // For each vertex.
            .map(|x| {
                // Get vertex parents.
                let z: Vec<_> = Pa!(g_max, x).collect();
                // Compute vertex score.
                let s = DecomposableScoringCriterion::call(&self.s, d, x, &z);
                // Insert into the cache.
                c.insert((x, z), s);

                s
            })
            // Sum the partial scores.
            .sum();

        // Get current edge set.
        let e: BTreeSet<_> = E!(g_max).collect();
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
            // Log current iteration.
            debug!("i: {}, max_iter: {}", i, self.max_iter);

            // Reset the flag.
            flag = false;
            // Initialize current best operation.
            let (mut op, mut delta) = (None, 0.);
            // Initialize current solution.
            let (mut g, mut s_g) = (g_max.clone(), s_g_max);

            // For each possible edge addition ...
            for &(x, y) in &add {
                // Check if operation is valid.
                if !Self::is_valid::<{ Op::ADD }>(&g, x, y, k) {
                    continue;
                }
                // Compute current operation delta score.
                let delta_star = self.eval::<{ Op::ADD }>(&mut c, d, &g, x, y);
                // Check if operation improves current solution.
                if delta_star > delta && (s_g + delta_star) > s_g_max {
                    // Set best operation.
                    (op, delta) = (Some((x, y, Op::ADD)), delta_star);
                }
            }

            // For each possible edge deletion ...
            for &(x, y) in &del {
                // Check if operation is valid.
                if !Self::is_valid::<{ Op::DEL }>(&g, x, y, k) {
                    continue;
                }
                // Compute current operation delta score.
                let delta_star = self.eval::<{ Op::DEL }>(&mut c, d, &g, x, y);
                // Check if operation improves current solution.
                if delta_star > delta && (s_g + delta_star) > s_g_max {
                    // Set best operation.
                    (op, delta) = (Some((x, y, Op::DEL)), delta_star);
                }
            }

            // For each possible edge reversal ...
            for &(x, y) in &rev {
                // Check if operation is valid.
                if !Self::is_valid::<{ Op::REV }>(&g, x, y, k) {
                    continue;
                }
                // Compute current operation delta score.
                let delta_star = self.eval::<{ Op::REV }>(&mut c, d, &g, x, y);
                // Check if operation improves current solution.
                if delta_star > delta && (s_g + delta_star) > s_g_max {
                    // Set best operation.
                    (op, delta) = (Some((x, y, Op::REV)), delta_star);
                }
            }

            // If best operation exists.
            if let Some((x, y, a)) = op {
                // Apply operation to current solution.
                (g, s_g) = (Self::apply(g, x, y, a), s_g + delta);
                // Update search space.
                Self::update(&mut add, &mut del, &mut rev, x, y, a);
                // Update best solution.
                (g_max, s_g_max) = (g, s_g);
                // Set the flag.
                flag = true;
            }

            // Increment counter.
            i += 1;
        }

        g_max
    }
}

impl<D, K, G, S> HillClimbing<D, K, G, S, score_types::NonDecomposable>
where
    D: DataSet,
    G: DefaultGraph + PathGraph,
    S: ScoringCriterion<D, G, ScoreType = score_types::NonDecomposable>,
{
    pub fn call(&self, d: &D, k: &K) -> G {
        todo!() // FIXME:
    }
}

pub type HC<D, K, G, S, ST> = HillClimbing<D, K, G, S, ST>;

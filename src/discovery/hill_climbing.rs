use std::{
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
};

use itertools::iproduct;

use super::{score_types, DecomposableScoringCriterion, ScoringCriterion};
use crate::{
    data::DataSet,
    graphs::{DefaultGraph, PathGraph},
    prelude::{directions, BaseGraph, DirectedGraph, BFS},
    Ch, Pa, V,
};

/// Local cache type.
type C = HashMap<(usize, Vec<usize>), f64>;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Del,
    Rev,
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
        // Check if score is already in cache. TODO: Avoid allocation if possible.
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

    fn eval(&self, c: &mut C, d: &D, g: &G, x: usize, y: usize, a: Op) -> f64 {
        // Get Y parents.
        let mut pa_y: Vec<_> = Pa!(g, y).collect();
        // Get current Y score.
        let s_y = self.cache(c, d, y, &pa_y);

        // Compute score delta depending on operation.
        match a {
            Op::Add => {
                // Add X in-place leveraging sorted Pa(G, Y).
                let i = pa_y.binary_search(&x).unwrap_err();
                pa_y.insert(i, x);
                // Compute delta score.
                s_y - self.cache(c, d, y, &pa_y)
            }
            Op::Del => {
                // Remove X in-place leveraging sorted Pa(G, Y).
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute delta score.
                s_y - self.cache(c, d, y, &pa_y)
            }
            Op::Rev => {
                // Get X parents.
                let mut pa_x: Vec<_> = Pa!(g, x).collect();
                // Get current X score.
                let s_x = self.cache(c, d, y, &pa_x);

                // Add Y in-place leveraging sorted Pa(G, X).
                let i = pa_x.binary_search(&y).unwrap_err();
                pa_x.insert(i, y);
                // Compute Y score.
                let s_star_x = self.cache(c, d, y, &pa_x);

                // Remove X in-place leveraging sorted Pa(G, Y).
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute Y score.
                let s_star_y = self.cache(c, d, y, &pa_y);

                // Compute delta score.
                s_y - s_star_y + s_x - s_star_x
            }
        }
    }

    fn is_valid(g: &G, x: usize, y: usize, a: Op, k: &K) -> bool {
        // Check the *edge* case.
        x != y &&
        // Check validity depending on operation. TODO: Check prior knowledge.
        (
            // (X, Y) not in E, pi(Y, X) not in G.
            (matches!(a, Op::Add) && !g.has_edge(x, y) && !g.has_path(y, x)) ||
            // (X, Y) in E.
            (matches!(a, Op::Del) && g.has_edge(x, y)) ||
            // (Y, X) not in E, (X, Y) in E, pi(X, Y) not in G.
            (
                matches!(a, Op::Rev) &&
                !g.has_edge(y, x) &&
                g.has_edge(x, y) &&
                !Ch!(g, x).filter(|&z| z != y)
                    .any(|z| BFS::from((g, z)).any(|w| w == y))
            )
        )
    }

    fn apply(mut g: G, x: usize, y: usize, a: Op) -> G {
        // Apply operation.
        match a {
            Op::Add => g.add_edge(x, y),
            Op::Del => g.del_edge(x, y),
            Op::Rev => g.add_edge(y, x) && g.del_edge(x, y),
        };

        g
    }

    pub fn call(&self, d: &D, k: &K) -> G {
        // Get number of variables.
        let n = d.labels().len();
        // Initialize delta scores cache.
        let mut c = C::new();

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

        // Initialize iterations counter.
        let mut i = 0;
        // Initialize the increasing score flag.
        let mut flag = true;

        // While score increase and at maximum `max_iter` times.
        while flag && i < self.max_iter {
            // Reset the flag.
            flag = false;
            // Initialize current solution.
            let (mut g, mut s_g) = (g_max.clone(), s_g_max);

            // For each possible edge addition, deletion or reversal ...
            for (a, x, y) in iproduct!([Op::Add, Op::Del, Op::Rev], 0..n, 0..n) {
                // Check if operation is valid.
                if !Self::is_valid(&g, x, y, a, k) {
                    continue;
                }

                // Compute current operation delta score.
                let delta = self.eval(&mut c, d, &g, x, y, a);

                // Check if operation improves current solution.
                if delta > 0. && (s_g + delta) > s_g_max {
                    // Apply operation to current solution.
                    g = Self::apply(g, x, y, a);
                    // Update current solution score.
                    s_g += delta;
                }
            }

            // If the score of the modified graph improves current solution.
            if s_g > s_g_max {
                // Update current solution.
                (g_max, s_g_max) = (g, s_g);
                // Set flag.
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

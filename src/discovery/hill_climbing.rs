use std::marker::PhantomData;

use itertools::{iproduct, Itertools};
use log::{debug, trace};
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

use super::{
    score_types::Decomposable, DecomposableScoringCriterion, PriorKnowledge, ScoringCriterion,
    ScoringCriterionCache as C,
};
use crate::{
    data::DataSet,
    graphs::{algorithms::traversal::BFS, Directed, DirectedGraph, Graph, PathGraph},
    types::FxIndexSet,
    Ch, Pa, E, L, V,
};

type CU<K> = Vec<(K, f64)>;

type KE = Option<(usize, Vec<usize>)>;

type E = FxIndexSet<(usize, usize)>;

type ES = (
    E, // To-be-added space,
    E, // To-be-deleted space,
    E, // To-be-reversed space.
);

#[derive(Clone, Copy, Debug)]

struct Op;

impl Op {
    const ADD: u8 = 0;

    const DEL: u8 = 1;

    const REV: u8 = 2;
}

type A = (usize, usize, u8);

#[derive(Clone, Debug)]
pub struct HillClimbing<'a, D, K, G, S, T>
where
    S: ScoringCriterion<D, G, T>,
{
    max_in_degree: usize,
    max_iter: usize,
    seed: Option<u64>,
    _d: PhantomData<D>,
    _k: PhantomData<K>,
    _t: PhantomData<T>,
    g: Option<G>,
    scoring_criterion: &'a S,
}

impl<'a, D, K, G, S, T> HillClimbing<'a, D, K, G, S, T>
where
    S: ScoringCriterion<D, G, T>,
{
    /// Construct a new hill-climbing functor given the scoring criterion $\mathcal{S}$.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::{prelude::*, polars::prelude::*};
    ///
    /// // Load data set from CSV file.
    /// let data_set = CsvReader::from_path("./tests/assets/asia.csv").unwrap().finish().unwrap();
    /// let data_set: CategoricalDataMatrix = data_set.into();
    /// // Initialize empty prior knowledge.
    /// let prior_knowledge = FR::new(data_set.labels_iter(), [], []);
    ///
    /// // Initialize scoring criterion.
    /// let scoring_criterion = BIC::new(&data_set);
    ///
    /// // Perform discovery.
    /// let pred_graph: DGraph = HC::new(&scoring_criterion)
    ///     .call(&data_set, &prior_knowledge);
    /// ```
    ///
    #[inline]
    pub fn new(scoring_criterion: &'a S) -> Self {
        // Get max in-degree or default to maximum in-degree.
        let max_in_degree = scoring_criterion.max_in_degree_hint().unwrap_or(usize::MAX);

        Self {
            max_in_degree,
            max_iter: usize::MAX,
            seed: None,
            _d: PhantomData,
            _k: PhantomData,
            _t: PhantomData,
            g: None,
            scoring_criterion,
        }
    }

    /// Set initial graph $\mathcal{G}$.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::{prelude::*, polars::prelude::*};
    ///
    /// // Load data set from CSV file.
    /// let data_set = CsvReader::from_path("./tests/assets/asia.csv").unwrap().finish().unwrap();
    /// let data_set: CategoricalDataMatrix = data_set.into();
    /// // Initialize empty prior knowledge.
    /// let prior_knowledge = FR::new(data_set.labels_iter(), [], []);
    ///
    /// // Initialize scoring criterion.
    /// let scoring_criterion = BIC::new(&data_set);
    ///
    /// // Construct initial graph.
    /// let init_graph = DiGraph::new(
    ///     data_set.labels_iter(),
    ///     [
    ///         ("bronc", "dysp"),
    ///         ("either", "dysp"),
    ///         ("either", "xray"),
    ///     ]
    /// );
    ///
    /// // Perform discovery with given initial graph.
    /// let pred_graph: DGraph = HC::new(&scoring_criterion)
    ///     .with_initial_graph(init_graph)
    ///     .call(&data_set, &prior_knowledge);
    /// ```
    ///
    #[inline]
    pub fn with_initial_graph(mut self, g: G) -> Self {
        // Set initial graph.
        self.g = Some(g);

        self
    }

    /// Set max in-degree.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::{prelude::*, polars::prelude::*};
    ///
    /// // Load data set from CSV file.
    /// let data_set = CsvReader::from_path("./tests/assets/asia.csv").unwrap().finish().unwrap();
    /// let data_set: CategoricalDataMatrix = data_set.into();
    /// // Initialize empty prior knowledge.
    /// let prior_knowledge = FR::new(data_set.labels_iter(), [], []);
    ///
    /// // Initialize scoring criterion.
    /// let scoring_criterion = BIC::new(&data_set);
    ///
    /// // Perform discovery with maximum in-degree of 3.
    /// let pred_graph: DGraph = HC::new(&scoring_criterion)
    ///     .with_max_in_degree(3)
    ///     .call(&data_set, &prior_knowledge);
    /// ```
    ///
    #[inline]
    pub const fn with_max_in_degree(mut self, max_in_degree: usize) -> Self {
        // Set hyper parameter.
        self.max_in_degree = max_in_degree;

        self
    }

    /// Set max iterations.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::{prelude::*, polars::prelude::*};
    ///
    /// // Load data set from CSV file.
    /// let data_set = CsvReader::from_path("./tests/assets/asia.csv").unwrap().finish().unwrap();
    /// let data_set: CategoricalDataMatrix = data_set.into();
    /// // Initialize empty prior knowledge.
    /// let prior_knowledge = FR::new(data_set.labels_iter(), [], []);
    ///
    /// // Initialize scoring criterion.
    /// let scoring_criterion = BIC::new(&data_set);
    ///
    /// // Perform discovery with maximum 10 iterations.
    /// let pred_graph: DGraph = HC::new(&scoring_criterion)
    ///     .with_max_iter(10)
    ///     .call(&data_set, &prior_knowledge);
    /// ```
    ///
    #[inline]
    pub const fn with_max_iter(mut self, max_iter: usize) -> Self {
        // Set hyper parameter.
        self.max_iter = max_iter;

        self
    }

    /// Enables columns shuffling by setting the seed.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::{prelude::*, polars::prelude::*};
    ///
    /// // Load data set from CSV file.
    /// let data_set = CsvReader::from_path("./tests/assets/asia.csv").unwrap().finish().unwrap();
    /// let data_set: CategoricalDataMatrix = data_set.into();
    /// // Initialize empty prior knowledge.
    /// let prior_knowledge = FR::new(data_set.labels_iter(), [], []);
    ///
    /// // Initialize scoring criterion.
    /// let scoring_criterion = BIC::new(&data_set);
    ///
    /// // Perform discovery with initial shuffling of search space order.
    /// let pred_graph: DGraph = HC::new(&scoring_criterion)
    ///     .with_shuffle(42)
    ///     .call(&data_set, &prior_knowledge);
    /// ```
    ///
    #[inline]
    pub const fn with_shuffle(mut self, seed: u64) -> Self {
        // Set random number generator seed.
        self.seed = Some(seed);

        self
    }
}

impl<'a, D, K, G, S, T> HillClimbing<'a, D, K, G, S, T>
where
    G: Graph,
    S: ScoringCriterion<D, G, T>,
{
    #[inline]
    fn apply(in_degree: &mut [usize], mut g: G, x: usize, y: usize, a: u8) -> G {
        // Apply operation.
        match a {
            Op::ADD => {
                assert!(g.add_edge(x, y));
                in_degree[y] += 1;
            }
            Op::DEL => {
                assert!(g.del_edge(x, y));
                in_degree[y] -= 1;
            }
            Op::REV => {
                assert!(g.del_edge(x, y));
                in_degree[y] -= 1;
                assert!(g.add_edge(y, x));
                in_degree[x] += 1;
            }
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
            &g[x],
            &g[y],
        );

        g
    }

    #[inline]
    fn update((mut add, mut del, mut rev): ES, x: usize, y: usize, a: u8) -> ES {
        // Apply operation.
        match a {
            Op::ADD => {
                // Remove performed action.
                assert!(add.remove(&(x, y)));
                // Add(X, Y) implies that (X, Y) is not in the
                // required list, therefore Del(X, Y) is valid.
                assert!(del.insert((x, y)));
                // If Add(Y, X) and Del(X, Y) are valid, then Rev(X, Y) is valid.
                // Since Del(X, Y) is valid by construction, check only Add(Y, X).
                if add.contains(&(y, x)) {
                    assert!(rev.insert((x, y)));
                }
            }
            Op::DEL => {
                // Del(X, Y) implies that (X, Y) is not in the
                // forbidden list, therefore Add(X, Y) is valid.
                assert!(add.insert((x, y)));
                // Remove performed action.
                assert!(del.remove(&(x, y)));
                // If Add(Y, X) and Del(X, Y) are valid, then Rev(X, Y) is valid.
                // Since Del(X, Y) is valid by construction, check only Add(Y, X).
                if add.contains(&(y, x)) {
                    assert!(rev.remove(&(x, y)));
                }
            }
            Op::REV => {
                // Remove performed action(s).
                assert!(add.remove(&(y, x)));
                assert!(del.remove(&(x, y)));
                assert!(rev.remove(&(x, y)));
                // Rev(X, Y) implies than (X, Y) is not in the
                // required list nor in the forbidden list,
                // therefore, Add(X, Y) is valid.
                assert!(add.insert((x, y)));
                // Rev(X, Y) implies than (Y, X) is not in the
                // required list nor in the forbidden list,
                // therefore, Del(Y, X) is valid.
                assert!(del.insert((y, x)));
                // If Rev(X, Y) is valid then Rev(Y, X) is valid.
                assert!(rev.insert((y, x)));
            }
            _ => panic!("Unknown operation code"),
        };

        (add, del, rev)
    }
}

impl<'a, D, K, G, S, T> HillClimbing<'a, D, K, G, S, T>
where
    D: DataSet,
    K: PriorKnowledge,
    G: DirectedGraph<Direction = Directed> + PathGraph,
    S: ScoringCriterion<D, G, T>,
{
    #[inline]
    fn init(&self, d: &D, k: &K) -> (ES, Vec<usize>, G) {
        // Check if initial graph has been provided.
        let mut g = match self.g.as_ref() {
            // If initial graph is provided ...
            Some(g) => g.clone(),
            // If no initial graph is provided, initialize an empty one.
            None => G::empty(d.labels_iter()),
        };

        // Check coherence with data set ...
        assert!(
            L!(g).eq(d.labels_iter()),
            "Graph labels must be equal to data set labels"
        );
        // Check coherence of graph and prior knowledge.
        assert!(
            L!(g).eq(k.labels()),
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
        let n = d.labels_iter().len();
        // Get columns index.
        let mut n = (0..n).collect_vec();
        // Check if random number generator has been set.
        if let Some(seed) = self.seed {
            // Initialize random number generator.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
            // Shuffle columns.
            n.shuffle(&mut rng);
            // Log shuffled columns.
            debug!(
                "Seed is set, shuffled columns as: [{}]",
                n.iter().map(|&x| &g[x]).format(", ")
            );
        }

        // Get current edge set.
        let e: E = E!(g).collect();
        // Initialize potential edges to be added.
        let add: E = iproduct!(n.clone(), n)
            // Remove any edge (X, Y) s.t. X == Y, is present in the initial graph, or is in the forbidden list.
            .filter(|&(x, y)| x != y && !e.contains(&(x, y)) && !k.has_forbidden(x, y))
            .collect();
        // Initialize potential edges to be deleted.
        let del: E = e
            .clone()
            .into_iter()
            // Remove any edge in the required list.
            .filter(|(x, y)| !k.has_required(*x, *y))
            .collect();
        // Initialize potential edges to be reversed.
        let rev: E = e
            .into_iter()
            // Remove any reversed edge in the forbidden list.
            .filter(|(x, y)| !k.has_required(*x, *y) && !k.has_forbidden(*y, *x))
            .collect();

        // Compute current in-degree.
        let in_degree = V!(g).map(|x| g.in_degree(x)).collect();

        ((add, del, rev), in_degree, g)
    }

    #[inline]
    fn is_valid<const OP: u8>(&self, in_degree: &[usize], g: &G, x: usize, y: usize) -> bool {
        // Check validity depending on operation.
        let is_valid = match OP {
            // |Pa(G, X)| < max_Pa, (X, Y) not in F, pi(Y, X) not in G.
            Op::ADD => in_degree[y] < self.max_in_degree && !g.has_edge(y, x) && !g.has_path(y, x),
            // (X, Y) in R.
            Op::DEL => true,
            // |Pa(G, X)| < max_Pa, (Y, X) not in F, (X, Y) in R, pi(X, Y) not in G.
            Op::REV => {
                in_degree[x] < self.max_in_degree
                    && !Ch!(g, x)
                        .filter(|&z| z != y)
                        .any(|z| BFS::from((g, z)).any(|w| w == y))
            }
            // Unknown operation code.
            _ => panic!("Unknown operation code"),
        };

        // Check if invalid.
        if !is_valid {
            // Log invalid.
            trace!(
                "op: {}({}, {}), invalid",
                match OP {
                    Op::ADD => "Add",
                    Op::DEL => "Del",
                    Op::REV => "Rev",
                    _ => panic!("Unknown operation code"),
                },
                &g[x],
                &g[y],
            );
        }

        is_valid
    }
}

/* Implement Hill-Climbing for Decomposable Scoring Criteria */
impl<'a, D, K, G, S> HillClimbing<'a, D, K, G, S, Decomposable>
where
    D: DataSet,
    K: PriorKnowledge,
    G: DirectedGraph<Direction = Directed> + PathGraph,
    S: DecomposableScoringCriterion<D, G>,
{
    #[inline]
    fn eval<const OP: u8>(
        &self,
        cache: &C<'a, D, G, S, Decomposable, (usize, Vec<usize>)>,
        g: &G,
        x: usize,
        y: usize,
    ) -> ((A, f64), CU<KE>) {
        // Get current Y score.s
        let mut pa_y = Pa!(g, y).collect_vec();
        let s_y = cache.call(y, &pa_y);
        // Compute delta score depending on operation.
        let (delta_star, s_star) = match OP {
            Op::ADD => {
                // Add X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap_err();
                pa_y.insert(i, x);
                // Compute score.
                let s_y_star = cache.call(y, &pa_y);

                (s_y_star.1 - s_y.1, vec![s_y_star, s_y])
            }
            Op::DEL => {
                // Remove X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute score.
                let s_y_star = cache.call(y, &pa_y);

                (s_y_star.1 - s_y.1, vec![s_y_star, s_y])
            }
            Op::REV => {
                // Get current X score.
                let mut pa_x = Pa!(g, x).collect_vec();
                let s_x = cache.call(x, &pa_x);

                // Add Y in-place by leveraging Pa(G, X) order.
                let i = pa_x.binary_search(&y).unwrap_err();
                pa_x.insert(i, y);
                // Compute score.
                let s_x_star = cache.call(x, &pa_x);

                // Remove X in-place by leveraging Pa(G, Y) order.
                let i = pa_y.binary_search(&x).unwrap();
                pa_y.remove(i);
                // Compute score.
                let s_y_star = cache.call(y, &pa_y);

                (
                    (s_x_star.1 - s_x.1) + (s_y_star.1 - s_y.1),
                    vec![s_x_star, s_x, s_y_star, s_y],
                )
            }
            _ => panic!("Unknown operation code"),
        };

        // Log current operation delta.
        trace!(
            "op: {}({}, {}), delta: {}",
            match OP {
                Op::ADD => "Add",
                Op::DEL => "Del",
                Op::REV => "Rev",
                _ => panic!("Unknown operation code"),
            },
            &g[x],
            &g[y],
            delta_star
        );

        (((x, y, OP), delta_star), s_star)
    }

    #[inline]
    fn search(
        &self,
        (add, del, rev): (&E, &E, &E),
        cache: &mut C<'a, D, G, S, Decomposable, (usize, Vec<usize>)>,
        in_degree: &[usize],
        g: &G,
    ) -> Option<(A, f64)> {
        // Compute operations deltas and cache fragments
        let (ops_deltas, fragments): (Vec<_>, Vec<_>) = add
            .iter()
            // Check if operation is valid, compute current operation delta score and cache fragments.
            .filter_map(|(x, y)| {
                if self.is_valid::<{ Op::ADD }>(in_degree, g, *x, *y) {
                    Some(self.eval::<{ Op::ADD }>(&cache, g, *x, *y))
                } else {
                    None
                }
            })
            .chain(del.iter().filter_map(|(x, y)| {
                if self.is_valid::<{ Op::DEL }>(in_degree, g, *x, *y) {
                    Some(self.eval::<{ Op::DEL }>(&cache, g, *x, *y))
                } else {
                    None
                }
            }))
            .chain(rev.iter().filter_map(|(x, y)| {
                if self.is_valid::<{ Op::REV }>(in_degree, g, *x, *y) {
                    Some(self.eval::<{ Op::REV }>(&cache, g, *x, *y))
                } else {
                    None
                }
            }))
            // Unzip OPs and cache fragments.
            .unzip();
        // Merge cache updates.
        cache.extend(
            fragments
                .into_iter()
                .flatten()
                .filter_map(|(k, v)| k.map(|k| (k, v))),
        );
        // Get operation with highest strictly positive delta score, if any.
        ops_deltas
            .into_iter()
            .filter(|(_, delta)| delta > &0.)
            .max_by(|(_, delta), (_, delta_star)| delta.partial_cmp(&delta_star).unwrap())
    }

    /// Perform discovery given data set $\mathbf{D}$ and prior knowledge $\mathbf{K}$.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::{prelude::*, polars::prelude::*};
    ///
    /// // Load data set from CSV file.
    /// let data_set = CsvReader::from_path("./tests/assets/asia.csv").unwrap().finish().unwrap();
    /// let data_set: CategoricalDataMatrix = data_set.into();
    /// // Initialize empty prior knowledge.
    /// let prior_knowledge = FR::new(data_set.labels_iter(), [], []);
    ///
    /// // Initialize scoring criterion.
    /// let scoring_criterion = BIC::new(&data_set);
    ///
    /// // Perform discovery.
    /// let pred_graph: DGraph = HC::new(&scoring_criterion)
    ///     .call(&data_set, &prior_knowledge);
    /// ```
    ///
    pub fn call(&self, d: &D, k: &K) -> G {
        // Initialize graph from D and K.
        let ((mut add, mut del, mut rev), mut in_degree, mut g) = self.init(d, k);
        // Initialize delta scores cache.
        let mut cache = C::new(self.scoring_criterion);
        // Compute the initial score.
        let mut s_g: f64 = V!(g)
            // For each vertex.
            .map(|x| {
                // Get vertex parents.
                let z = Pa!(g, x).collect_vec();
                // Compute vertex score.
                let s = self.scoring_criterion.call(x, &z);
                // Insert into the cache.
                cache.extend([((x, z), s)]);

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

            // For each possible edge operation ...
            let op_delta = self.search((&add, &del, &rev), &mut cache, &in_degree, &g);

            // If best operation exists.
            if let Some(((x, y, a), delta)) = op_delta {
                // Apply operation to current solution.
                (g, s_g) = (Self::apply(&mut in_degree, g, x, y, a), s_g + delta);
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

/* Implement parallel Hill-Climbing for Decomposable Scoring Criteria */
impl<'a, D, K, G, S> HillClimbing<'a, D, K, G, S, Decomposable>
where
    D: DataSet + Sync,
    K: PriorKnowledge + Sync,
    G: DirectedGraph<Direction = Directed> + PathGraph + Sync,
    S: DecomposableScoringCriterion<D, G> + Sync,
{
    #[inline]
    fn par_search(
        &self,
        (add, del, rev): (&E, &E, &E),
        cache: &mut C<'a, D, G, S, Decomposable, (usize, Vec<usize>)>,
        in_degree: &[usize],
        g: &G,
    ) -> Option<(A, f64)> {
        // Compute operations deltas and cache fragments
        let (ops_deltas, fragments): (Vec<_>, Vec<_>) = add
            .par_iter()
            // Check if operation is valid, compute current operation delta score and cache fragments.
            .filter_map(|(x, y)| {
                if self.is_valid::<{ Op::ADD }>(in_degree, g, *x, *y) {
                    Some(self.eval::<{ Op::ADD }>(&cache, g, *x, *y))
                } else {
                    None
                }
            })
            .chain(del.par_iter().filter_map(|(x, y)| {
                if self.is_valid::<{ Op::DEL }>(in_degree, g, *x, *y) {
                    Some(self.eval::<{ Op::DEL }>(&cache, g, *x, *y))
                } else {
                    None
                }
            }))
            .chain(rev.par_iter().filter_map(|(x, y)| {
                if self.is_valid::<{ Op::REV }>(in_degree, g, *x, *y) {
                    Some(self.eval::<{ Op::REV }>(&cache, g, *x, *y))
                } else {
                    None
                }
            }))
            // Unzip OPs and cache fragments.
            .unzip();
        // Merge cache updates.
        cache.par_extend(
            fragments
                .into_par_iter()
                .flatten()
                .filter_map(|(k, v)| k.map(|k| (k, v))),
        );
        // Get operation with highest strictly positive delta score, if any.
        ops_deltas
            .into_par_iter()
            .filter(|(_, delta)| delta > &0.)
            .max_by(|(_, delta), (_, delta_star)| delta.partial_cmp(&delta_star).unwrap())
    }

    pub fn par_call(&self, d: &D, k: &K) -> G {
        // Initialize graph from D and K.
        let ((mut add, mut del, mut rev), mut in_degree, mut g) = self.init(d, k);
        // Initialize delta scores cache.
        let mut cache = C::new(self.scoring_criterion);
        // Compute the initial score.
        let mut s_g: f64 = {
            // Insert into the cache in parallel.
            cache.par_extend(
                (0..g.order())
                    .into_par_iter()
                    // For each vertex.
                    .map(|x| {
                        // Get vertex parents.
                        let z = Pa!(g, x).collect_vec();
                        // Compute vertex score.
                        let s = self.scoring_criterion.call(x, &z);

                        ((x, z), s)
                    }),
            );
            // Compute initial score.
            cache.par_values().sum()
        };

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

            // For each possible edge operation ...
            let op_delta = self.par_search((&add, &del, &rev), &mut cache, &in_degree, &g);

            // If best operation exists.
            if let Some(((x, y, a), delta)) = op_delta {
                // Apply operation to current solution.
                (g, s_g) = (Self::apply(&mut in_degree, g, x, y, a), s_g + delta);
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

pub type HC<'a, D, K, G, S, T> = HillClimbing<'a, D, K, G, S, T>;

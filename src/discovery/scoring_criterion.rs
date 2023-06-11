use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    graphs::{directions, DirectedGraph},
    types::FxIndexMap,
    Pa, V,
};

/// Score-types pseudo-enumerator for generics algorithms.
pub mod score_types {
    /// Decomposable score-type pseudo-enumerator for generics algorithms.
    #[derive(Clone, Debug)]
    pub struct Decomposable;
    /// Non-decomposable score-type pseudo-enumerator for generics algorithms.
    #[derive(Clone, Debug)]
    pub struct NonDecomposable;
}

/// Scoring criterion trait.
pub trait ScoringCriterion<D, G, T>: Clone + Debug + Sync {
    /// Computes the score value for the given data set $\mathbf{D}$ and graph $\mathcal{G}$.
    fn call(&self, g: &G) -> f64;

    /// Returns the maximum in-degree that can be reached while increasing the score.
    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        None
    }
}

/// Decomposable scoring criterion trait.
pub trait DecomposableScoringCriterion<D, G>: Clone + Debug + Sync {
    /// Computes the score value for the given data set $\mathbf{D}$, vertex $X$ and parents $\mathbf{Z}$.
    fn call(&self, x: usize, z: &[usize]) -> f64;

    /// Returns the maximum in-degree that can be reached while increasing the score.
    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        None
    }
}

/* Blanket implementation for Decomposable Scoring Criterion */
impl<D, G, S> ScoringCriterion<D, G, score_types::Decomposable> for S
where
    G: DirectedGraph<Direction = directions::Directed>,
    S: DecomposableScoringCriterion<D, G>,
{
    #[inline]
    fn call(&self, g: &G) -> f64 {
        V!(g)
            .map(|x| (x, Pa!(g, x).collect_vec()))
            .map(|(x, z)| self.call(x, &z))
            .sum()
    }

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        DecomposableScoringCriterion::max_in_degree_hint(self)
    }
}

#[derive(Clone, Debug)]
/// Scoring criterion cache wrapper.
pub struct ScoringCriterionCache<'a, D, G, S, T, K> {
    _d: PhantomData<D>,
    _g: PhantomData<G>,
    _t: PhantomData<T>,
    scoring_criterion: &'a S,
    cache: FxIndexMap<K, f64>,
}

impl<'a, D, G, S, T, K> ScoringCriterionCache<'a, D, G, S, T, K> {
    /// Construct a new scoring criterion cache wrapper given the scoring criterion $\mathcal{S}$.
    pub fn new(scoring_criterion: &'a S) -> Self {
        Self {
            _d: PhantomData,
            _g: PhantomData,
            _t: PhantomData,
            scoring_criterion,
            cache: Default::default(),
        }
    }
}

impl<'a, D, G, S, T, K> Extend<(K, f64)> for ScoringCriterionCache<'a, D, G, S, T, K>
where
    K: Eq + Hash,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, f64)>,
    {
        // Delegate call to inner member.
        self.cache.extend(iter)
    }
}

impl<'a, D, G, S, T, K> ParallelExtend<(K, f64)> for ScoringCriterionCache<'a, D, G, S, T, K>
where
    K: Eq + Hash + Send,
{
    fn par_extend<I>(&mut self, par_iter: I)
    where
        I: IntoParallelIterator<Item = (K, f64)>,
    {
        // Delegate call to inner member.
        self.cache.par_extend(par_iter)
    }
}

/* Implement ScoringCriterionCache for (NonDecomposable) ScoringCriterion. */
impl<'a, D, G, S> ScoringCriterion<D, G, score_types::NonDecomposable>
    for ScoringCriterionCache<'a, D, G, S, score_types::NonDecomposable, G>
where
    D: Clone + Debug + Sync,
    G: Clone + Debug + Eq + Hash + Sync,
    S: ScoringCriterion<D, G, score_types::NonDecomposable>,
{
    fn call(&self, g: &G) -> f64 {
        // Get value from cache ...
        self.cache
            .get(g)
            .copied()
            // ... or compute it if not in cache.
            .unwrap_or_else(|| self.scoring_criterion.call(g))
    }
}

impl<'a, D, G, S> ScoringCriterionCache<'a, D, G, S, score_types::NonDecomposable, G>
where
    D: Clone + Debug + Sync,
    G: Clone + Debug + Eq + Hash + Sync,
    S: ScoringCriterion<D, G, score_types::NonDecomposable>,
{
    /// Returns the score from cache or compute it if not present.
    ///
    /// Returns a `(Option<K>, f64)` pair from the scoring criterion cache.
    /// If the score value is in the cache then the key is `None`, otherwise
    /// it is `Some(K)`.
    ///
    /// The returned `(K, f64)` can be inserted into the cache by calling
    /// `extend` or `par_extend` over an iterator of such pairs.
    ///
    /// The idea here is to update the cache in batch after querying it.
    ///
    pub fn call(&self, g: &G) -> (Option<G>, f64) {
        // Get value from cache ...
        self.cache
            .get(g)
            .copied()
            // ... or compute it if not in cache.
            .map_or_else(
                // If not in cache, return (key, value) pair to be inserted ...
                || (Some(g.clone()), self.scoring_criterion.call(g)),
                // ... else return value.
                |v| (None, v),
            )
    }
}

/* Implement ScoringCriterionCache for DecomposableScoringCriterion. */
impl<'a, D, G, S> DecomposableScoringCriterion<D, G>
    for ScoringCriterionCache<'a, D, G, S, score_types::Decomposable, (usize, Vec<usize>)>
where
    D: Clone + Debug + Sync,
    G: DirectedGraph<Direction = directions::Directed>,
    S: DecomposableScoringCriterion<D, G>,
{
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute cache key.
        let k = (x, z.to_vec());

        // Get value from cache ...
        self.cache
            .get(&k)
            .copied()
            // ... or compute it if not in cache.
            .unwrap_or_else(|| self.scoring_criterion.call(x, z))
    }

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        // Delegate call to inner member.
        self.scoring_criterion.max_in_degree_hint()
    }
}

impl<'a, D, G, S> ScoringCriterionCache<'a, D, G, S, score_types::Decomposable, (usize, Vec<usize>)>
where
    D: Clone + Debug + Sync,
    G: DirectedGraph<Direction = directions::Directed>,
    S: DecomposableScoringCriterion<D, G>,
{
    /// Returns the score from cache or compute it if not present.
    ///
    /// Returns a `(Option<K>, f64)` pair from the scoring criterion cache.
    /// If the score value is in the cache then the key is `None`, otherwise
    /// it is `Some(K)`.
    ///
    /// The returned `(K, f64)` can be inserted into the cache by calling
    /// `extend` or `par_extend` over an iterator of such pairs.
    ///
    /// The idea here is to update the cache in batch after querying it.
    ///
    pub fn call(&self, x: usize, z: &[usize]) -> (Option<(usize, Vec<usize>)>, f64) {
        // Compute cache key.
        let k = (x, z.to_vec());

        // Get value from cache ...
        self.cache
            .get(&k)
            .copied()
            // ... or compute it if not in cache.
            .map_or_else(
                // If not in cache, return (key, value) pair to be inserted ...
                || (Some(k), self.scoring_criterion.call(x, z)),
                // ... else return value.
                |v| (None, v),
            )
    }
}

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use indexmap::map::{rayon::ParValues, Values};
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    graphs::{directions, DirectedGraph},
    types::FxIndexMap,
    Pa, V,
};

pub mod score_types {

    #[derive(Clone, Debug)]
    pub struct Decomposable;

    #[derive(Clone, Debug)]
    pub struct NonDecomposable;
}

pub trait ScoringCriterion<D, G, T>: Clone + Debug + Sync {
    fn call(&self, g: &G) -> f64;

    #[inline]
    fn max_in_degree_hint(&self) -> Option<usize> {
        None
    }
}

pub trait DecomposableScoringCriterion<D, G>: Clone + Debug + Sync {
    fn call(&self, x: usize, z: &[usize]) -> f64;

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

pub struct ScoringCriterionCache<'a, D, G, S, T, K> {
    _d: PhantomData<D>,
    _g: PhantomData<G>,
    _t: PhantomData<T>,
    scoring_criterion: &'a S,
    cache: FxIndexMap<K, f64>,
}

impl<'a, D, G, S, T, K> ScoringCriterionCache<'a, D, G, S, T, K>
where
    K: Sync,
{
    pub fn new(scoring_criterion: &'a S) -> Self {
        Self {
            _d: PhantomData,
            _g: PhantomData,
            _t: PhantomData,
            scoring_criterion,
            cache: Default::default(),
        }
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, f64> {
        self.cache.values()
    }

    #[inline]
    pub fn par_values(&self) -> ParValues<'_, K, f64> {
        self.cache.par_values()
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

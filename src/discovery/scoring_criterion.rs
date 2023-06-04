use std::fmt::Debug;

use itertools::Itertools;

use crate::{
    graphs::{directions, DirectedGraph},
    Pa, V,
};

/// Score-types pseudo-enumerator for generics algorithms.
pub mod score_types {
    /// Decomposable score-type pseudo-enumerator for generics algorithms.
    pub struct Decomposable;
    /// Non-decomposable score-type pseudo-enumerator for generics algorithms.
    pub struct NonDecomposable;
}

/// Scoring criterion trait.
pub trait ScoringCriterion<D, G>: Clone + Debug + Sync {
    /// Score-types pseudo-enumerator for generics algorithms.
    type ScoreType;

    /// Computes the score value for the given data set $\mathbf{D}$ and graph $\mathcal{G}$.
    fn call(&self, g: &G) -> f64;

    /// Returns the maximum number of parents that can be added to increase the score.
    #[inline]
    fn max_parents_hint(&self) -> Option<usize> {
        None
    }
}

/// Decomposable scoring criterion trait.
pub trait DecomposableScoringCriterion<D, G>: Clone + Debug + Sync {
    /// Computes the score value for the given data set $\mathbf{D}$, vertex $X$ and parents $\mathbf{Z}$.
    fn call(&self, x: usize, z: &[usize]) -> f64;
}

/* Blanket implementation for Decomposable Scoring Criterion */
impl<D, G, S> ScoringCriterion<D, G> for S
where
    G: DirectedGraph<Direction = directions::Directed>,
    S: DecomposableScoringCriterion<D, G>,
{
    type ScoreType = score_types::Decomposable;

    #[inline]
    fn call(&self, g: &G) -> f64 {
        V!(g)
            .map(|x| (x, Pa!(g, x).collect_vec()))
            .map(|(x, z)| self.call(x, &z))
            .sum()
    }
}

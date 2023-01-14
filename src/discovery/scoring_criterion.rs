use std::fmt::Debug;

/// Score-types pseudo-enumerator for generics algorithms.
pub mod score_types {
    /// Decomposable score-type pseudo-enumerator for generics algorithms.
    pub struct Decomposable;
    /// Non-decomposable score-type pseudo-enumerator for generics algorithms.
    pub struct NonDecomposable;
}

/// Scoring criterion trait.
pub trait ScoringCriterion<D, G>: Clone + Debug {
    /// Score-types pseudo-enumerator for generics algorithms.
    type ScoreType;

    /// Computes the score value for the given data set $\mathbf{D}$ and graph $\mathcal{G}$.
    fn call(&self, d: &D, g: &G) -> f64;
}

/// Decomposable scoring criterion trait.
pub trait DecomposableScoringCriterion<D, G>:
    ScoringCriterion<D, G, ScoreType = score_types::Decomposable>
{
    /// Computes the score value for the given data set $\mathbf{D}$, vertex $X$ and parents $\mathbf{Z}$.
    fn call(&self, d: &D, x: usize, z: Vec<usize>) -> f64;
}

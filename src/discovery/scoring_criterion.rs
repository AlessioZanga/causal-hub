pub mod score_types {
    pub struct Decomposable;
    pub struct NonDecomposable;
}

pub trait ScoringCriterion<D, G> {
    type ScoreType;

    fn call(&self, d: &D, g: &G) -> f64;
}

pub trait DecomposableScoringCriterion<D, G>: ScoringCriterion<D, G, ScoreType = score_types::Decomposable> {
    fn call(&self, d: &D, x: usize, z: Vec<usize>) -> f64;
}

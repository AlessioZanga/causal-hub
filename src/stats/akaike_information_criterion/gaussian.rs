use super::AkaikeInformationCriterion;
use crate::{
    data::ContinuousDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

impl<const RESCALED: bool, const PARALLEL: bool>
    AkaikeInformationCriterion<ContinuousDataMatrix, RESCALED, PARALLEL>
{
    /// Computes AIC given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn call(&self, d: &ContinuousDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Compute the number of parameters.
        // NOTE: Intercept, standard deviation and regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Initialize the log-likelihood functor.
        let ll = LogLikelihood::<ContinuousDataMatrix, PARALLEL>::new();
        // Compute the log-likelihood.
        let ll = ll.call(d, x, z);

        // Check if AIC must be scaled.
        match RESCALED {
            // Rescale AIC by -2, coherently with LL.
            true => ll - self.k * theta,
            // Otherwise, compute original AIC.
            false => 2. * self.k * theta - 2. * ll,
        }
    }
}

impl<G, const RESCALED: bool, const PARALLEL: bool>
    DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for AkaikeInformationCriterion<ContinuousDataMatrix, RESCALED, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, d: &ContinuousDataMatrix, x: usize, z: &[usize]) -> f64 {
        self.call(d, x, z)
    }
}

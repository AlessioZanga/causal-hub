use super::AkaikeInformationCriterion;
use crate::{
    data::ContinuousDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

impl<'a, G, const RESCALED: bool, const PARALLEL: bool>
    DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for AkaikeInformationCriterion<'a, ContinuousDataMatrix, RESCALED, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the number of parameters.
        // NOTE: Intercept, standard deviation and regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Initialize the log-likelihood functor.
        let s = LogLikelihood::<_, PARALLEL>::new(self.d);
        // Compute the log-likelihood.
        let s = DecomposableScoringCriterion::<ContinuousDataMatrix, G>::call(&s, x, z);

        // Check if AIC must be scaled.
        match RESCALED {
            // Rescale AIC by -2, coherently with LL.
            true => s - self.k * theta,
            // Otherwise, compute original AIC.
            false => 2. * self.k * theta - 2. * s,
        }
    }
}

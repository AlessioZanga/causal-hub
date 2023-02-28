use super::BayesianInformationCriterion;
use crate::{
    data::ContinuousDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    prelude::DataSet,
    stats::LogLikelihood,
};

impl<G, const RESCALED: bool, const PARALLEL: bool>
    DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for BayesianInformationCriterion<ContinuousDataMatrix, RESCALED, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, d: &ContinuousDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Get the sample size.
        let n = d.values().nrows() as f64;

        // Compute the number of parameters.
        // NOTE: Intercept, standard deviation and regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Initialize the log-likelihood functor.
        let s = LogLikelihood::<_, PARALLEL>::new();
        // Compute the log-likelihood.
        let s = DecomposableScoringCriterion::<ContinuousDataMatrix, G>::call(&s, d, x, z);

        // Check if BIC must be scaled.
        match RESCALED {
            // Rescale BIC by -2, coherently with LL.
            true => s - 0.5 * self.k * theta * f64::ln(n),
            // Otherwise, compute original BIC.
            false => self.k * theta * f64::ln(n) - 2. * s,
        }
    }
}

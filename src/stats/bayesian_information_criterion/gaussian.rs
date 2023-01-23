use super::BayesianInformationCriterion;
use crate::{
    data::ContinuousDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

impl<const RESCALED: bool, const PARALLEL: bool>
    BayesianInformationCriterion<ContinuousDataMatrix, RESCALED, PARALLEL>
{
    /// Computes BIC given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn call(&self, d: &ContinuousDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Get the sample size.
        let n = d.nrows() as f64;

        // Compute the number of parameters.
        // NOTE: Intercept, standard deviation and regression coefficient per parent.
        let theta = (2 + z.len()) as f64;

        // Initialize the log-likelihood functor.
        let ll = LogLikelihood::<ContinuousDataMatrix, PARALLEL>::new();
        // Compute the log-likelihood.
        let ll = ll.call(d, x, z);

        // Check if BIC must be scaled.
        match RESCALED {
            // Rescale BIC by -2, coherently with LL.
            true => ll - 0.5 * self.k * theta * f64::ln(n),
            // Otherwise, compute original BIC.
            false => self.k * theta * f64::ln(n) - 2. * ll,
        }
    }
}

impl<G, const RESCALED: bool, const PARALLEL: bool>
    DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for BayesianInformationCriterion<ContinuousDataMatrix, RESCALED, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, d: &ContinuousDataMatrix, x: usize, z: &[usize]) -> f64 {
        self.call(d, x, z)
    }
}

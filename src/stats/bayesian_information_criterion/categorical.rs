use super::BayesianInformationCriterion;
use crate::{
    data::CategoricalDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

impl<G, const RESCALED: bool, const PARALLEL: bool>
    DecomposableScoringCriterion<CategoricalDataMatrix, G>
    for BayesianInformationCriterion<CategoricalDataMatrix, RESCALED, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, d: &CategoricalDataMatrix, x: usize, z: &[usize]) -> f64 {
        // Get the sample size.
        let n = d.nrows() as f64;

        // Get the cardinality.
        let cards = d.cardinality();
        // Get the cardinality of vertices.
        // NOTE: If Z is empty, then the product of an empty vector is still one.
        let (card_x, card_z) = (cards[x], z.iter().map(|&z| cards[z]).product::<usize>());
        // Compute the number of parameters.
        let theta = ((card_x - 1) * card_z) as f64;

        // Initialize the log-likelihood functor.
        let s = LogLikelihood::<_, PARALLEL>::new();
        // Compute the log-likelihood.
        let s = DecomposableScoringCriterion::<CategoricalDataMatrix, G>::call(&s, d, x, z);

        // Check if BIC must be scaled.
        match RESCALED {
            // Rescale BIC by -2, coherently with LL.
            true => s - 0.5 * self.k * theta * f64::ln(n),
            // Otherwise, compute original BIC.
            false => self.k * theta * f64::ln(n) - 2. * s,
        }
    }
}

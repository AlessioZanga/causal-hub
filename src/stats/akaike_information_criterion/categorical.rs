use super::AkaikeInformationCriterion;
use crate::{
    data::CategoricalDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    stats::LogLikelihood,
};

impl<G, const RESCALED: bool, const PARALLEL: bool>
    DecomposableScoringCriterion<CategoricalDataMatrix, G>
    for AkaikeInformationCriterion<CategoricalDataMatrix, RESCALED, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, d: &CategoricalDataMatrix, x: usize, z: &[usize]) -> f64 {
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

        // Check if AIC must be scaled.
        match RESCALED {
            // Rescale AIC by -2, coherently with LL.
            true => s - self.k * theta,
            // Otherwise, compute original AIC.
            false => 2. * self.k * theta - 2. * s,
        }
    }
}

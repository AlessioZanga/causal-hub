use super::DiscreteBayesianNetwork;
use crate::{
    prelude::{BaseGraph, Factor, ProbabilisticGraphicalModel},
    utils::nan_to_zero,
    V,
};

/// Kullback-Leibler Divergence functor.
pub struct KullbackLeiblerDivergence<'a, P, Q> {
    p: &'a P,
    q: &'a Q,
}

impl<'a, P, Q> KullbackLeiblerDivergence<'a, P, Q> {
    /// Construct a new Kullback-Leibler Divergence functor.
    #[inline]
    pub const fn new(p: &'a P, q: &'a Q) -> Self {
        Self { p, q }
    }
}

impl<'a> KullbackLeiblerDivergence<'a, DiscreteBayesianNetwork, DiscreteBayesianNetwork> {
    /// Compute the Kullback-Leibler divergence given two discrete Bayesian networks.
    pub fn call(&self) -> f64 {
        // Assert underlying graphs are the same.
        assert_eq!(
            self.p.graph(),
            self.q.graph(),
            concat!(
                "P and Q must have the same underlying graphs, ",
                "consider distribution projection, where needed"
            )
        );
        // Assert models have same parameters.
        self.p
            .parameters()
            .values()
            .zip(self.q.parameters().values())
            .map(|(p, q)| (p.states(), q.states()))
            .for_each(|(p, q)| {
                assert_eq!(
                    p, q,
                    concat!(
                        "P and Q must have the same parameters states:\n",
                        "P: {:?}\n",
                        "Q: {:?}\n",
                    ),
                    p, q
                )
            });

        // Compute the KL divergence leveraging local decomposition.
        V!(self.p.graph())
            // Get X parameters w.r.t. P and Q.
            .map(|x| {
                (
                    self.p.parameters()[x].values(),
                    self.q.parameters()[x].values(),
                )
            })
            // Compute the KL(P, Q) = \sum P(X | Z) * log( P(X | Z) / Q(X | Z) ),
            // with 0 * log 0 = 0 and 0 / 0 = 0, i.e. mapping NaNs to zeros.
            .map(|(p, q)| (p * (p / q).mapv(f64::ln)).mapv(nan_to_zero).sum())
            // Aggregate the local KL divergences.
            .sum()
    }
}

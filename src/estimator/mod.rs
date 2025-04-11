mod bayesian;
mod maximum_likelihood;
pub use bayesian::*;
pub use maximum_likelihood::*;

use crate::{
    graph::{DiGraph, Graph},
    model::BayesianNetwork,
};

/// A trait for conditional probability distribution estimators.
pub trait ConditionalProbabilityDistributionEstimator<D, P> {
    /// Fits the estimator to the data and returns a CPD.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to fit the estimator to.
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated CDP.
    ///
    fn fit(&self, data: &D, x: usize, z: &[usize]) -> P;
}

/// A type alias for a conditional probability distribution estimator.
pub use ConditionalProbabilityDistributionEstimator as CPDEstimator;

/// A trait for Bayesian network estimators.
pub trait BayesianNetworkEstimator<D, BN> {
    /// Fits the estimator to the data and returns a Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to fit the estimator to.
    /// * `graph` - The graph to fit the estimator to.
    ///
    /// # Returns
    ///
    /// The estimated Bayesian network.
    ///
    fn fit(&self, data: &D, graph: DiGraph) -> BN;
}

/// A type alias for a Bayesian network estimator.
pub use BayesianNetworkEstimator as BNEstimator;

/// Blanket implement for all BN estimators with a corresponding CPD estimator.
impl<D, BN, E> BNEstimator<D, BN> for E
where
    BN: BayesianNetwork,
    E: CPDEstimator<D, BN::CPD>,
{
    fn fit(&self, data: &D, graph: DiGraph) -> BN {
        // Fit the parameters of the distribution using the estimator.
        let cpds: Vec<_> = graph
            .vertices()
            .map(|i| self.fit(data, i, &graph.parents(i)))
            .collect();
        // Construct the Bayesian network with the graph and the parameters.
        BN::new(graph, cpds)
    }
}

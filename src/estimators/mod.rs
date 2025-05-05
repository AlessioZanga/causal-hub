mod bayesian;
pub use bayesian::*;

mod maximum_likelihood;
pub use maximum_likelihood::*;

mod sufficient_statistics;
pub use sufficient_statistics::*;

use crate::{
    graphs::{DiGraph, Graph},
    models::{BayesianNetwork, ContinuousTimeBayesianNetwork},
};

/// A trait for sufficient statistics estimators.
pub trait ConditionalSufficientStatisticsEstimator<S> {
    /// Fits the estimator to the dataset and returns the conditional sufficient statistics.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The sufficient statistics.
    ///
    fn fit(&self, x: usize, z: &[usize]) -> S;
}

/// A type alias for a sufficient statistics estimator.
pub use ConditionalSufficientStatisticsEstimator as CSSEstimator;

/// A trait for conditional probability distribution estimators.
pub trait ConditionalProbabilityDistributionEstimator<P> {
    /// Fits the estimator to the dataset and returns a CPD.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated CDP.
    ///
    fn fit(&self, x: usize, z: &[usize]) -> P;
}

/// A type alias for a conditional probability distribution estimator.
pub use ConditionalProbabilityDistributionEstimator as CPDEstimator;

/// A trait for Bayesian network estimators.
pub trait BayesianNetworkEstimator<BN> {
    /// Fits the estimator to the dataset and returns a Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to fit the estimator to.
    ///
    /// # Returns
    ///
    /// The estimated Bayesian network.
    ///
    fn fit(&self, graph: DiGraph) -> BN;
}

/// A type alias for a Bayesian network estimator.
pub use BayesianNetworkEstimator as BNEstimator;

/// Blanket implement for all BN estimators with a corresponding CPD estimator.
impl<BN, E> BNEstimator<BN> for E
where
    BN: BayesianNetwork,
    E: CPDEstimator<BN::CPD>,
{
    fn fit(&self, graph: DiGraph) -> BN {
        // Fit the parameters of the distribution using the estimator.
        let cpds: Vec<_> = graph
            .vertices()
            .map(|i| self.fit(i, &graph.parents(i)))
            .collect();
        // Construct the BN with the graph and the parameters.
        BN::new(graph, cpds)
    }
}

/// A trait for CTBN estimators.
pub trait ContinuousTimeBayesianNetworkEstimator<CTBN> {
    /// Fits the estimator to the trajectory and returns a CTBN.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to fit the estimator to.
    ///
    /// # Returns
    ///
    /// The estimated CTBN.
    ///
    fn fit(&self, graph: DiGraph) -> CTBN;
}

/// A type alias for a CTBN estimator.
pub use ContinuousTimeBayesianNetworkEstimator as CTBNEstimator;

/// Blanket implement for all CTBN estimators with a corresponding CPD estimator.
impl<CTBN, E> CTBNEstimator<CTBN> for E
where
    CTBN: ContinuousTimeBayesianNetwork,
    E: CPDEstimator<CTBN::CIM>,
{
    fn fit(&self, graph: DiGraph) -> CTBN {
        // Fit the parameters of the distribution using the estimator.
        let cims: Vec<_> = graph
            .vertices()
            .map(|i| self.fit(i, &graph.parents(i)))
            .collect();
        // Construct the CTBN with the graph and the parameters.
        CTBN::new(graph, cims)
    }
}

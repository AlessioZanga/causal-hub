mod bayesian;
pub use bayesian::*;

mod expectation_maximization;
pub use expectation_maximization::*;

mod maximum_likelihood;
pub use maximum_likelihood::*;

mod sufficient_statistics;
pub use sufficient_statistics::*;

mod raw;
pub use raw::*;
use rayon::prelude::*;

use crate::{
    distributions::CPD,
    graphs::{DiGraph, Graph},
    models::{BN, CTBN},
};

/// A trait for sufficient statistics estimators.
pub trait ConditionalSufficientStatisticsEstimator {
    /// The type of sufficient statistics.
    type Output;

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
    fn fit(&self, x: usize, z: &[usize]) -> Self::Output;
}

/// A type alias for a sufficient statistics estimator.
pub use ConditionalSufficientStatisticsEstimator as CSSEstimator;

/// A trait for sufficient statistics estimators in parallel.
pub trait ParallelConditionalSufficientStatisticsEstimator {
    /// The type of sufficient statistics.
    type Output;

    /// Fits the estimator to the dataset and returns the conditional sufficient statistics in parallel.
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
    fn par_fit(&self, x: usize, z: &[usize]) -> Self::Output;
}

/// A type alias for a parallel sufficient statistics estimator.
pub use ParallelConditionalSufficientStatisticsEstimator as ParCSSEstimator;

/// A trait for conditional probability distribution estimators.
pub trait ConditionalProbabilityDistributionEstimator<P>
where
    P: CPD,
{
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
    #[inline]
    fn fit(&self, x: usize, z: &[usize]) -> P {
        // Return only the CPD.
        self.fit_transform(x, z).1
    }

    /// Fits the estimator to the dataset and returns the sufficient statistics and the CPD.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated sufficient statistics and CPD.
    ///
    fn fit_transform(&self, x: usize, z: &[usize]) -> (P::SS, P);
}

/// A type alias for a conditional probability distribution estimator.
pub use ConditionalProbabilityDistributionEstimator as CPDEstimator;

/// A trait for conditional probability distribution estimators in parallel.
pub trait ParallelConditionalProbabilityDistributionEstimator<P> {
    /// The type of sufficient statistics.
    type SS;

    /// Fits the estimator to the dataset and returns a CPD in parallel.
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
    #[inline]
    fn par_fit(&self, x: usize, z: &[usize]) -> P {
        // Return only the CPD.
        self.par_fit_transform(x, z).1
    }

    /// Fits the estimator to the dataset and returns the sufficient statistics and the CPD is parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated sufficient statistics and CPD.
    ///
    fn par_fit_transform(&self, x: usize, z: &[usize]) -> (Self::SS, P);
}

/// A type alias for a parallel conditional probability distribution estimator.
pub use ParallelConditionalProbabilityDistributionEstimator as ParCPDEstimator;

/// A trait for Bayesian network estimators.
pub trait BayesianNetworkEstimator<T> {
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
    fn fit(&self, graph: DiGraph) -> T;
}

/// A type alias for a Bayesian network estimator.
pub use BayesianNetworkEstimator as BNEstimator;

/// Blanket implement for all BN estimators with a corresponding CPD estimator.
impl<T, E> BNEstimator<T> for E
where
    T: BN,
    T::CPD: CPD,
    E: CPDEstimator<T::CPD>,
{
    fn fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cpds: Vec<_> = graph
            .vertices()
            .map(|i| self.fit(i, &graph.parents(i)))
            .collect();
        // Construct the BN with the graph and the parameters.
        T::new(graph, cpds)
    }
}

/// A trait for parallel Bayesian network estimators.
pub trait ParallelBayesianNetworkEstimator<T> {
    /// Fits the estimator to the dataset and returns a Bayesian network in parallel.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to fit the estimator to.
    ///
    /// # Returns
    ///
    /// The estimated Bayesian network.
    ///
    fn par_fit(&self, graph: DiGraph) -> T;
}

/// A type alias for a parallel Bayesian network estimator.
pub use ParallelBayesianNetworkEstimator as ParBNEstimator;

/// Blanket implement for all BN estimators with a corresponding CPD estimator.
impl<T, E> ParBNEstimator<T> for E
where
    T: BN,
    T::CPD: Send,
    E: ParCPDEstimator<T::CPD> + Sync,
{
    fn par_fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cpds: Vec<_> = graph
            .vertices()
            .into_par_iter()
            .map(|i| self.par_fit(i, &graph.parents(i)))
            .collect();
        // Construct the BN with the graph and the parameters.
        T::new(graph, cpds)
    }
}

/// A trait for CTBN estimators.
pub trait ContinuousTimeBayesianNetworkEstimator<T> {
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
    fn fit(&self, graph: DiGraph) -> T;
}

/// A type alias for a CTBN estimator.
pub use ContinuousTimeBayesianNetworkEstimator as CTBNEstimator;

/// Blanket implement for all CTBN estimators with a corresponding CPD estimator.
impl<T, E> CTBNEstimator<T> for E
where
    T: CTBN,
    T::CIM: CPD,
    E: CPDEstimator<T::CIM>,
{
    fn fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cims: Vec<_> = graph
            .vertices()
            .map(|i| self.fit(i, &graph.parents(i)))
            .collect();
        // Construct the CTBN with the graph and the parameters.
        T::new(graph, cims)
    }
}

/// A trait for parallel CTBN estimators.
pub trait ParallelContinuousTimeBayesianNetworkEstimator<T> {
    /// Fits the estimator to the trajectory and returns a CTBN in parallel.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to fit the estimator to.
    ///
    /// # Returns
    ///
    /// The estimated CTBN.
    ///
    fn par_fit(&self, graph: DiGraph) -> T;
}

/// A type alias for a parallel CTBN estimator.
pub use ParallelContinuousTimeBayesianNetworkEstimator as ParCTBNEstimator;

/// Blanket implement for all CTBN estimators with a corresponding CPD estimator.
impl<T, E> ParCTBNEstimator<T> for E
where
    T: CTBN,
    T::CIM: Send,
    E: ParCPDEstimator<T::CIM> + Sync,
{
    fn par_fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cims: Vec<_> = graph
            .vertices()
            .into_par_iter()
            .map(|i| self.par_fit(i, &graph.parents(i)))
            .collect();
        // Construct the CTBN with the graph and the parameters.
        T::new(graph, cims)
    }
}

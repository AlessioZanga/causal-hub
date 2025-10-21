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
    models::{BN, CIM, CPD, CTBN, DiGraph, Graph},
    set,
    types::Set,
};

/// A trait for sufficient statistics estimators.
pub trait CSSEstimator<T> {
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
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for sufficient statistics estimators in parallel.
pub trait ParCSSEstimator<T> {
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
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for conditional probability distribution estimators.
pub trait CPDEstimator<T>
where
    T: CPD,
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
    /// The estimated CPD.
    ///
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for conditional probability distribution estimators in parallel.
pub trait ParCPDEstimator<T>
where
    T: CPD,
{
    /// Fits the estimator to the dataset and returns a CPD in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated CPD.
    ///
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for Bayesian network estimators.
pub trait BNEstimator<T> {
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
            .into_iter()
            .map(|i| {
                let i = set![i];
                self.fit(&i, &graph.parents(&i))
            })
            .collect();
        // Construct the BN with the graph and the parameters.
        T::new(graph, cpds)
    }
}

/// A trait for parallel Bayesian network estimators.
pub trait ParBNEstimator<T> {
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

/// Blanket implement for all BN estimators with a corresponding CPD estimator.
impl<T, E> ParBNEstimator<T> for E
where
    T: BN,
    T::CPD: CPD + Send,
    E: ParCPDEstimator<T::CPD> + Sync,
{
    fn par_fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cpds: Vec<_> = graph
            .vertices()
            .into_par_iter()
            .map(|i| {
                let i = set![i];
                self.par_fit(&i, &graph.parents(&i))
            })
            .collect();
        // Construct the BN with the graph and the parameters.
        T::new(graph, cpds)
    }
}

/// A trait for conditional intensity matrix estimators.
pub trait CIMEstimator<T>
where
    T: CIM,
{
    /// Fits the estimator to the dataset and returns a CIM.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated CIM.
    ///
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for conditional intensity matrix estimators in parallel.
pub trait ParCIMEstimator<T>
where
    T: CIM,
{
    /// Fits the estimator to the dataset and returns a CIM in parallel.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to fit the estimator to.
    /// * `z` - The variables to condition on.
    ///
    /// # Returns
    ///
    /// The estimated CIM.
    ///
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> T;
}

/// A trait for CTBN estimators.
pub trait CTBNEstimator<T> {
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

/// Blanket implement for all CTBN estimators with a corresponding CIM estimator.
impl<T, E> CTBNEstimator<T> for E
where
    T: CTBN,
    T::CIM: CIM,
    E: CIMEstimator<T::CIM>,
{
    fn fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cims: Vec<_> = graph
            .vertices()
            .into_iter()
            .map(|i| {
                let i = set![i];
                self.fit(&i, &graph.parents(&i))
            })
            .collect();
        // Construct the CTBN with the graph and the parameters.
        T::new(graph, cims)
    }
}

/// A trait for parallel CTBN estimators.
pub trait ParCTBNEstimator<T> {
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

/// Blanket implement for all CTBN estimators with a corresponding CIM estimator.
impl<T, E> ParCTBNEstimator<T> for E
where
    T: CTBN,
    T::CIM: CIM + Send,
    E: ParCIMEstimator<T::CIM> + Sync,
{
    fn par_fit(&self, graph: DiGraph) -> T {
        // Fit the parameters of the distribution using the estimator.
        let cims: Vec<_> = graph
            .vertices()
            .into_par_iter()
            .map(|i| {
                let i = set![i];
                self.par_fit(&i, &graph.parents(&i))
            })
            .collect();
        // Construct the CTBN with the graph and the parameters.
        T::new(graph, cims)
    }
}

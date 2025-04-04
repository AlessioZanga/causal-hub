mod categorical;
pub use categorical::*;

use crate::estimator::Estimator;

/// A trait for Bayesian networks.
pub trait BayesianNetwork {
    /// The type of the labels.
    type Labels;
    /// The type of the graph.
    type Graph;
    /// The type of the distribution.
    type Distribution;
    /// The type of the parameters.
    type Parameters;

    /// Returns the labels of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the labels.
    ///
    fn labels(&self) -> &Self::Labels;

    /// Returns the underlying graph.
    ///
    /// # Returns
    ///
    /// A reference to the graph.
    ///
    fn graph(&self) -> &Self::Graph;

    /// Returns the parameters.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    fn parameters(&self) -> &Self::Parameters;

    /// Returns the parameters size.
    ///
    /// # Returns
    ///
    /// The parameters size.
    ///
    fn parameters_size(&self) -> usize;

    /// Fits the Bayesian network to the data given the estimator and the graph.
    /// 
    /// # Arguments
    /// 
    /// * `estimator` - The estimator used to fit.
    /// * `graph` - The graph to fit the estimator to.
    /// 
    /// # Returns
    /// 
    /// The fitted Bayesian network.
    ///
    fn from_estimator<E>(estimator: &E, graph: Self::Graph) -> Self
    where
        E: Estimator<Distribution = Self::Distribution>;
}

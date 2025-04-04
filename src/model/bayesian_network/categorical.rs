use fxhash::FxHashMap;

use crate::{
    distribution::{CategoricalDistribution, Distribution},
    estimator::Estimator,
    graph::{DiGraph, Graph},
};

use super::BayesianNetwork;

/// A categorical Bayesian network.
///
#[derive(Clone, Debug)]
pub struct CategoricalBayesianNetwork {
    /// The underlying graph.
    graph: DiGraph,
    /// The parameters of the distribution.
    parameters: FxHashMap<String, CategoricalDistribution>,
}

/// A type alias for the categorical Bayesian network.
pub type CategoricalBN = CategoricalBayesianNetwork;

impl BayesianNetwork for CategoricalBayesianNetwork {
    type Labels = <Self::Graph as Graph>::Labels;
    type Graph = DiGraph;
    type Distribution = CategoricalDistribution;
    type Parameters = FxHashMap<String, Self::Distribution>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        self.graph.labels()
    }

    #[inline]
    fn graph(&self) -> &Self::Graph {
        &self.graph
    }

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    fn parameters_size(&self) -> usize {
        self.parameters
            .iter()
            .map(|(_, d)| d.parameters_size())
            .sum()
    }

    fn from_estimator<E>(estimator: &E, graph: Self::Graph) -> Self
    where
        E: Estimator<Distribution = Self::Distribution>,
    {
        // Fit the parameters of the distribution using the estimator.
        let parameters = graph
            .labels()
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                (
                    // The label of the variable.
                    x.into(),
                    // The fitted distribution.
                    estimator.fit(i, &graph.parents(i)),
                )
            })
            .collect();

        Self { graph, parameters }
    }
}

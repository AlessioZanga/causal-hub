use crate::{
    distribution::{CategoricalDistribution, Distribution},
    estimator::Estimator,
    graph::{DiGraph, Graph},
    types::{FxIndexMap, FxIndexSet},
};

use super::BayesianNetwork;

/// A categorical Bayesian network.
///
#[derive(Clone, Debug)]
pub struct CategoricalBayesianNetwork {
    /// The underlying graph.
    graph: DiGraph,
    /// The parameters of the distribution.
    parameters: FxIndexMap<String, CategoricalDistribution>,
}

/// A type alias for the categorical Bayesian network.
pub type CategoricalBN = CategoricalBayesianNetwork;

impl BayesianNetwork for CategoricalBayesianNetwork {
    type Labels = <Self::Graph as Graph>::Labels;
    type Graph = DiGraph;
    type Distribution = CategoricalDistribution;
    type Parameters = FxIndexMap<String, Self::Distribution>;

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

impl CategoricalBayesianNetwork {
    /// Creates a new categorical Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `parameters` - The parameters of the distribution.
    ///
    /// # Returns
    ///
    /// A new `CategoricalBayesianNetwork` instance.
    ///
    pub fn new(graph: DiGraph, parameters: Vec<CategoricalDistribution>) -> Self {
        // Assert same number of labels and parameters.
        assert_eq!(
            graph.labels().len(),
            parameters.len(),
            "Number of labels and distributions must be equal."
        );
        // Create map of parameters.
        let mut parameters: FxIndexMap<_, _> = parameters
            .into_iter()
            .map(|x| (x.labels()[0].clone(), x))
            .collect();
        // Assert each vertex has a parameter.
        assert!(
            graph.labels().iter().all(|x| parameters.contains_key(x)),
            "Each vertex must have a distribution."
        );
        // Reorder the parameters to match the order of the graph labels.
        parameters.sort_by(|a, _, b, _| {
            let a = graph.labels().get_index_of(a).unwrap();
            let b = graph.labels().get_index_of(b).unwrap();
            a.cmp(&b)
        });
        // Assert the labels of the parameters are the same as the graph parents.
        assert!(
            // Check if all vertices have the same labels as their parents.
            graph.vertices().into_iter().all(|i| {
                // Get the parents of the vertex.
                let parents: FxIndexSet<_> = graph.parents(i).into_iter().collect();
                // Check if the labels of the parameters are in the parents.
                parameters[i].labels().iter().skip(1).all(|j| {
                    // Check if the label is in the parents.
                    parents.contains(&graph.labels().get_index_of(j).unwrap())
                })
            }),
            "Distributions labels must be the same as the graph parents."
        );

        Self { graph, parameters }
    }
}

use core::panic;

use super::BayesianNetwork;
use crate::{
    distribution::{CategoricalCPD, Distribution},
    graph::{DiGraph, Graph},
    types::{FxIndexMap, FxIndexSet},
};

/// A categorical Bayesian network.
///
#[derive(Clone, Debug)]
pub struct CategoricalBayesianNetwork {
    /// The underlying graph.
    graph: DiGraph,
    /// The conditional probability distributions.
    cpds: FxIndexMap<String, CategoricalCPD>,
}

/// A type alias for the categorical Bayesian network.
pub type CategoricalBN = CategoricalBayesianNetwork;

impl CategoricalBayesianNetwork {
    /// Creates a new categorical Bayesian network.
    ///
    /// # Arguments
    ///
    /// * `graph` - The underlying graph.
    /// * `cpds` - The parameters of the distribution.
    ///
    /// # Returns
    ///
    /// A new `CategoricalBayesianNetwork` instance.
    ///
    pub fn new(graph: DiGraph, cpds: Vec<CategoricalCPD>) -> Self {
        // Create map of parameters.
        let cpds = cpds
            .into_iter()
            .map(|x| (x.labels()[0].clone(), x))
            .collect();

        Self::with_graph_cpds(graph, cpds)
    }
}

impl BayesianNetwork for CategoricalBayesianNetwork {
    type Labels = <DiGraph as Graph>::Labels;
    type CPD = CategoricalCPD;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        self.graph.labels()
    }

    #[inline]
    fn graph(&self) -> &DiGraph {
        &self.graph
    }

    #[inline]
    fn cpds(&self) -> &FxIndexMap<String, Self::CPD> {
        &self.cpds
    }

    fn parameters_size(&self) -> usize {
        self.cpds.iter().map(|(_, d)| d.parameters_size()).sum()
    }

    fn with_graph_cpds(graph: DiGraph, mut cpds: FxIndexMap<String, Self::CPD>) -> Self {
        // Assert same number of graph labels and CPDs.
        assert_eq!(
            graph.labels().len(),
            cpds.len(),
            "Failed to map graph labels to distributions labels."
        );
        // Reorder the CPDs to match the order of the graph labels.
        cpds.sort_by(|a, _, b, _| {
            let a = graph
                .labels()
                .get_index_of(a)
                .unwrap_or_else(|| panic!("Failed to get index of label '{}'.", a));
            let b = graph
                .labels()
                .get_index_of(b)
                .unwrap_or_else(|| panic!("Failed to get index of label '{}'.", b));
            a.cmp(&b)
        });
        // Assert the labels of the parameters are the same as the graph parents.
        assert!(
            // Check if all vertices have the same labels as their parents.
            graph.vertices().all(|i| {
                // Get the parents of the vertex.
                let parents: FxIndexSet<_> = graph.parents(i).into_iter().collect();
                // Check if the labels of the parameters are in the parents.
                cpds[i].labels().iter().skip(1).all(|j| {
                    // Check if the label is in the parents.
                    parents.contains(&graph.labels().get_index_of(j).unwrap())
                })
            }),
            "Failed to align graph parents and conditioning labels."
        );

        Self { graph, cpds }
    }
}

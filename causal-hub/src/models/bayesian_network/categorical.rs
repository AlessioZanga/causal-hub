use ndarray::Array1;
use serde::{Deserialize, Serialize};

use super::BN;
use crate::{
    datasets::CategoricalDataset,
    distributions::{CPD, CategoricalCPD},
    graphs::{DiGraph, Graph, TopologicalOrder},
    types::{FxIndexMap, FxIndexSet},
};

/// A categorical Bayesian network (BN).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalBayesianNetwork {
    /// The states of the variables.
    states: FxIndexMap<String, FxIndexSet<String>>,
    /// The underlying graph.
    graph: DiGraph,
    /// The conditional probability distributions.
    cpds: FxIndexMap<String, CategoricalCPD>,
    /// The topological order of the graph.
    topological_order: Vec<usize>,
}

/// A type alias for the categorical Bayesian network.
pub type CategoricalBN = CategoricalBayesianNetwork;

impl CategoricalBN {
    /// Returns the states of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the states of the variables.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }
}

impl BN for CategoricalBN {
    type Labels = <DiGraph as Graph>::Labels;
    type CPD = CategoricalCPD;
    type Sample = Array1<u8>;
    type Dataset = CategoricalDataset;

    fn new<I>(graph: DiGraph, cpds: I) -> Self
    where
        I: IntoIterator<Item = Self::CPD>,
    {
        // Collect the CPDs into a map.
        let mut cpds: FxIndexMap<_, _> = cpds
            .into_iter()
            .map(|x| (x.label().to_owned(), x))
            .collect();
        // Sort the CPDs by their labels.
        cpds.sort_keys();

        // Assert same number of graph labels and CPDs.
        assert!(
            graph.labels().iter().eq(cpds.keys()),
            "Graph labels and distributions labels must be the same."
        );

        // Allocate the states of the variables.
        let mut states: FxIndexMap<String, FxIndexSet<String>> = FxIndexMap::default();
        // Insert the states of the variables into the map to check if they are the same.
        for cpd in cpds.values() {
            std::iter::once((cpd.label(), cpd.states()))
                .chain(cpd.conditioning_states())
                .for_each(|(l, s)| {
                    // Check if the states are already in the map.
                    if let Some(existing_states) = states.get(l) {
                        // Check if the states are the same.
                        assert_eq!(
                            existing_states, s,
                            "States of `{l}` must be the same across CPDs.",
                        );
                    } else {
                        // Insert the states into the map.
                        states.insert(l.to_owned(), s.clone());
                    }
                });
        }
        // Sort the states of the variables.
        states.sort_keys();

        // Assert the labels of the parameters are the same as the graph parents.
        assert!(
            // Check if all vertices have the same labels as their parents.
            graph.vertices().all(|i| {
                // Check if the labels of the parameters are in the parents.
                graph
                    .parents(i)
                    .into_iter()
                    .eq(cpds[i].conditioning_labels().iter().map(|j| {
                        // Get the index of the label in the graph.
                        graph.labels().get_index_of(j).unwrap()
                    }))
            }),
            "Graph parents labels and conditioning labels must be the same."
        );

        // Assert the graph is acyclic.
        let topological_order = graph.topological_order().expect("Graph must be acyclic.");

        Self {
            states,
            graph,
            cpds,
            topological_order,
        }
    }

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

    #[inline]
    fn topological_order(&self) -> &[usize] {
        &self.topological_order
    }
}

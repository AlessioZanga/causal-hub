use approx::{AbsDiffEq, RelativeEq};
use ndarray::Array1;
use serde::{Deserialize, Serialize};

use super::BN;
use crate::{
    datasets::CatData,
    distributions::{CPD, CatCPD},
    graphs::{DiGraph, Graph, TopologicalOrder},
    types::{Labels, Map, States},
};

/// A categorical Bayesian network (BN).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalBayesianNetwork {
    /// The states of the variables.
    states: States,
    /// The underlying graph.
    graph: DiGraph,
    /// The conditional probability distributions.
    cpds: Map<String, CatCPD>,
    /// The topological order of the graph.
    topological_order: Vec<usize>,
}

/// A type alias for the categorical Bayesian network.
pub type CatBN = CategoricalBayesianNetwork;

impl CatBN {
    /// Returns the states of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the states of the variables.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        &self.states
    }
}

impl AbsDiffEq for CatBN {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.states.eq(&other.states)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self
                .cpds
                .iter()
                .zip(&other.cpds)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.abs_diff_eq(other_cpd, epsilon)
                })
    }
}

impl RelativeEq for CatBN {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.states.eq(&other.states)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self
                .cpds
                .iter()
                .zip(&other.cpds)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.relative_eq(other_cpd, epsilon, max_relative)
                })
    }
}

impl BN for CatBN {
    type CPD = CatCPD;
    type Sample = Array1<u8>;
    type Samples = CatData;

    fn new<I>(graph: DiGraph, cpds: I) -> Self
    where
        I: IntoIterator<Item = Self::CPD>,
    {
        // Collect the CPDs into a map.
        let mut cpds: Map<_, _> = cpds
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
        let mut states: States = Default::default();
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
            graph.vertices().into_iter().all(|i| {
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
    fn labels(&self) -> &Labels {
        self.graph.labels()
    }

    #[inline]
    fn graph(&self) -> &DiGraph {
        &self.graph
    }

    #[inline]
    fn cpds(&self) -> &Map<String, Self::CPD> {
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

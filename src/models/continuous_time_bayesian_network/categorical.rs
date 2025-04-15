use ndarray::prelude::*;

use super::ContinuousTimeBayesianNetwork;
use crate::{
    distributions::{CPD, CategoricalCIM, CategoricalCPD},
    graphs::{DiGraph, Graph},
    models::{BayesianNetwork, CategoricalBN},
    types::FxIndexMap,
};

/// A categorical continuous time Bayesian network (CTBN).
#[derive(Clone, Debug)]
pub struct CategoricalContinuousTimeBayesianNetwork {
    /// The initial distribution.
    initial_distribution: CategoricalBN,
    /// The underlying graph.
    graph: DiGraph,
    /// The conditional intensity matrices.
    cims: FxIndexMap<String, CategoricalCIM>,
}

/// A type alias for the categorical CTBN.
pub type CategoricalCTBN = CategoricalContinuousTimeBayesianNetwork;

impl ContinuousTimeBayesianNetwork for CategoricalCTBN {
    type Labels = <DiGraph as Graph>::Labels;
    type CIM = CategoricalCIM;
    type InitialDistribution = CategoricalBN;

    fn new<I>(graph: DiGraph, cims: I) -> Self
    where
        I: IntoIterator<Item = Self::CIM>,
    {
        // Collect the CPDs into a map.
        let mut cims: FxIndexMap<_, _> = cims
            .into_iter()
            .map(|x| (x.label().to_owned(), x))
            .collect();
        // Sort the CPDs by their labels.
        cims.sort_keys();

        // Assert same number of graph labels and CPDs.
        assert!(
            graph.labels().iter().eq(cims.keys()),
            "Graph labels and distributions labels must be the same."
        );

        // Assert the labels of the parameters are the same as the graph parents.
        assert!(
            // Check if all vertices have the same labels as their parents.
            graph.vertices().all(|i| {
                // Check if the labels of the parameters are in the parents.
                graph
                    .parents(i)
                    .into_iter()
                    .eq(cims[i].conditioning_labels().iter().map(|j| {
                        // Get the index of the label in the graph.
                        graph.labels().get_index_of(j).unwrap()
                    }))
            }),
            "Graph parents labels and conditioning labels must be the same."
        );

        // Initialize an empty graph for the uniform initial distribution.
        let initial_graph = DiGraph::empty(graph.labels());
        // Initialize the CPDs as uniform distributions.
        let initial_cpds = cims.values().map(|cim| {
            // Get label and states of the CIM.
            let state = (cim.label(), cim.states());
            // Set empty conditioning states.
            let conditioning_states: [(&str, Vec<&str>); 0] = [];
            // Set uniform parameters.
            let alpha = state.0.len();
            let parameters = Array::from_vec(vec![1. / alpha as f64; alpha]);
            let parameters = parameters.insert_axis(Axis(0));
            // Construct the CPD.
            CategoricalCPD::new(state, conditioning_states, parameters)
        });
        // Initialize a uniform initial distribution.
        let initial_distribution = CategoricalBN::new(initial_graph, initial_cpds);

        Self {
            initial_distribution,
            graph,
            cims,
        }
    }

    fn labels(&self) -> &Self::Labels {
        self.graph.labels()
    }

    fn graph(&self) -> &DiGraph {
        &self.graph
    }

    fn cims(&self) -> &FxIndexMap<String, Self::CIM> {
        &self.cims
    }

    fn parameters_size(&self) -> usize {
        // Parameters size of the initial distribution.
        self.initial_distribution.parameters_size()
            // Parameters size of the CIMs.
            + self
                .cims
                .values()
                .map(|x| x.parameters_size())
                .sum::<usize>()
    }

    fn initial_distribution(&self) -> &Self::InitialDistribution {
        &self.initial_distribution
    }

    fn with_initial_distribution<I>(
        initial_distribution: Self::InitialDistribution,
        graph: DiGraph,
        cims: I,
    ) -> Self
    where
        I: IntoIterator<Item = Self::CIM>,
    {
        // Construct the CTBN.
        let mut ctbn = Self::new(graph, cims);

        // Assert the initial distribution has same labels.
        assert!(
            initial_distribution.labels().eq(ctbn.labels()),
            "Initial distribution labels must be the same as the CIMs labels."
        );
        // Assert the initial distribution has same states.
        assert!(
            initial_distribution
                .cpds()
                .into_iter()
                .zip(ctbn.cims())
                .all(|((_, cpd), (_, cim))| cpd.states().eq(cim.states())),
            "Initial distribution states must be the same as the CIMs states."
        );

        // Set the initial distribution.
        ctbn.initial_distribution = initial_distribution;

        ctbn
    }
}

use super::ContinuousTimeBayesianNetwork;
use crate::{
    distributions::{CPD, CategoricalCIM},
    graphs::{DiGraph, Graph},
    types::FxIndexMap,
};

/// A categorical continuous time Bayesian network (CTBN).
#[derive(Clone, Debug)]
pub struct CategoricalContinuousTimeBayesianNetwork {
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

        Self { graph, cims }
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
        self.cims.values().map(|x| x.parameters_size()).sum()
    }
}

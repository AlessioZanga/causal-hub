use approx::{AbsDiffEq, RelativeEq};
use ndarray::prelude::*;
use serde::{Deserialize, Serialize};

use super::CTBN;
use crate::{
    datasets::{CatTrj, CatTrjs},
    distributions::{CPD, CatCIM, CatCPD},
    graphs::{DiGraph, Graph},
    models::{BN, CatBN},
    types::{Labels, Map, States},
};

/// A categorical continuous time Bayesian network (CTBN).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoricalContinuousTimeBayesianNetwork {
    /// The initial distribution.
    initial_distribution: CatBN,
    /// The underlying graph.
    graph: DiGraph,
    /// The conditional intensity matrices.
    cims: Map<String, CatCIM>,
}

/// A type alias for the categorical CTBN.
pub type CatCTBN = CategoricalContinuousTimeBayesianNetwork;

impl CatCTBN {
    /// Returns the states of the variables.
    ///
    /// # Returns
    ///
    /// A reference to the states of the variables.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        self.initial_distribution.states()
    }
}

impl AbsDiffEq for CatCTBN {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.initial_distribution.eq(&other.initial_distribution)
            && self.graph.eq(&other.graph)
            && self
                .cims
                .iter()
                .zip(&other.cims)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.abs_diff_eq(other_cpd, epsilon)
                })
    }
}

impl RelativeEq for CatCTBN {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.initial_distribution.eq(&other.initial_distribution)
            && self.graph.eq(&other.graph)
            && self
                .cims
                .iter()
                .zip(&other.cims)
                .all(|((label, cpd), (other_label, other_cpd))| {
                    label.eq(other_label) && cpd.relative_eq(other_cpd, epsilon, max_relative)
                })
    }
}

impl CTBN for CatCTBN {
    type CIM = CatCIM;
    type InitialDistribution = CatBN;
    type Event = (f64, Array1<u8>);
    type Trajectory = CatTrj;
    type Trajectories = CatTrjs;

    fn new<I>(graph: DiGraph, cims: I) -> Self
    where
        I: IntoIterator<Item = Self::CIM>,
    {
        // Collect the CPDs into a map.
        let mut cims: Map<_, _> = cims
            .into_iter()
            .map(|x| (x.labels()[0].to_owned(), x)) // FIXME: This assumes `x` has a single element.
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
            graph.vertices().into_iter().all(|i| {
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
            let states = cim.states().clone();
            // Set empty conditioning states.
            let conditioning_states = States::default();
            // Set uniform parameters.
            let alpha = cim.cardinality().product();
            let parameters = Array::from_vec(vec![1. / alpha as f64; alpha]);
            let parameters = parameters.insert_axis(Axis(0));
            // Construct the CPD.
            CatCPD::new(states, conditioning_states, parameters)
        });
        // Initialize a uniform initial distribution.
        let initial_distribution = CatBN::new(initial_graph, initial_cpds);

        Self {
            initial_distribution,
            graph,
            cims,
        }
    }

    fn labels(&self) -> &Labels {
        self.graph.labels()
    }

    fn graph(&self) -> &DiGraph {
        &self.graph
    }

    fn cims(&self) -> &Map<String, Self::CIM> {
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

use approx::{AbsDiffEq, RelativeEq};

use crate::{
    datasets::{GaussSample, GaussTable},
    inference::TopologicalOrder,
    models::{BN, CPD, DiGraph, GaussCPD, Graph, Labelled},
    set,
    types::{Labels, Map},
};

/// A Gaussian Bayesian network.
#[derive(Clone, Debug)]
pub struct GaussBN {
    /// The name of the model.
    name: Option<String>,
    /// The description of the model.
    description: Option<String>,
    /// The labels of the variables.
    labels: Labels,
    /// The graph of the model.
    graph: DiGraph,
    /// The parameters of the model.
    cpds: Map<String, GaussCPD>,
    /// The topological order of the graph.
    topological_order: Vec<usize>,
}

impl PartialEq for GaussBN {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.graph.eq(&other.graph)
            && self.topological_order.eq(&other.topological_order)
            && self.cpds.eq(&other.cpds)
    }
}

impl AbsDiffEq for GaussBN {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
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

impl RelativeEq for GaussBN {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.labels.eq(&other.labels)
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

impl Labelled for GaussBN {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl BN for GaussBN {
    type CPD = GaussCPD;
    type Sample = GaussSample;
    type Samples = GaussTable;

    fn new<I>(graph: DiGraph, cpds: I) -> Self
    where
        I: IntoIterator<Item = Self::CPD>,
    {
        // Collect the CPDs into a map.
        let mut cpds: Map<_, _> = cpds
            .into_iter()
            // Assert CPD contains exactly one label.
            // TODO: Refactor code and remove this assumption.
            .inspect(|x| {
                assert_eq!(x.labels().len(), 1, "CPD must contain exactly one label.");
            })
            .map(|x| (x.labels()[0].to_owned(), x))
            .collect();
        // Sort the CPDs by their labels.
        cpds.sort_keys();

        // Assert same number of graph labels and CPDs.
        assert!(
            graph.labels().iter().eq(cpds.keys()),
            "Graph labels and distributions labels must be the same."
        );

        // Get the labels of the variables.
        let labels: Labels = graph.labels().clone();

        // Assert the labels of the parameters are the same as the graph parents.
        assert!(
            // Check if all vertices have the same labels as their parents.
            graph.vertices().into_iter().all(|i| {
                // Check if the labels of the parameters are in the parents.
                graph
                    .parents(&set![i])
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
            name: None,
            description: None,
            labels,
            graph,
            cpds,
            topological_order,
        }
    }

    #[inline]
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    #[inline]
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
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
        self.cpds.iter().map(|(_, x)| x.parameters_size()).sum()
    }

    #[inline]
    fn topological_order(&self) -> &[usize] {
        &self.topological_order
    }

    fn with_optionals<I>(
        name: Option<String>,
        description: Option<String>,
        graph: DiGraph,
        cpds: I,
    ) -> Self
    where
        I: IntoIterator<Item = Self::CPD>,
    {
        // Assert name is not empty string.
        if let Some(name) = &name {
            assert!(!name.is_empty(), "Name cannot be an empty string.");
        }
        // Assert description is not empty string.
        if let Some(description) = &description {
            assert!(
                !description.is_empty(),
                "Description cannot be an empty string."
            );
        }

        // Construct the BN.
        let mut bn = Self::new(graph, cpds);

        // Set the optional fields.
        bn.name = name;
        bn.description = description;

        bn
    }
}

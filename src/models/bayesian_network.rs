use std::fmt::{Debug, Display, Formatter};

use is_sorted::IsSorted;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{
    ConditionalProbabilityDistribution, DiscreteCPD, DiscreteJPD, Factor,
    JointProbabilityDistribution,
};
use crate::{
    graphs::{directions, structs::DirectedDenseAdjacencyMatrixGraph, DirectedGraph},
    io::BIF,
    prelude::{BaseGraph, PathGraph},
    types::FxIndexMap,
    Pa, L, V,
};

/// Probabilistic Graphical Model (PGM) trait.
pub trait ProbabilisticGraphicalModel:
    Clone
    + Debug
    + Display
    + Serialize
    + for<'a> Deserialize<'a>
    + Into<(Self::Graph, FxIndexMap<String, Self::Parameter>)>
{
    /// Underlying directed graph associated type. TODO: Generalize this bound.
    type Graph: DirectedGraph<Direction = directions::Directed>;
    /// Parameter associated type.
    type Parameter: Factor;

    /// Joint distribution associated type.
    type JPD: JointProbabilityDistribution<Phi = <Self::Parameter as Factor>::Phi>;
    /// Conditional distribution associated type.
    type CPD: ConditionalProbabilityDistribution<Phi = <Self::Parameter as Factor>::Phi>;

    /// Reference to the underlying graph.
    fn graph(&self) -> &Self::Graph;

    /// Reference to the parameters.
    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter>;
}

/// Bayesian Network $\mathcal{B}$ trait.
pub trait BayesianNetwork: ProbabilisticGraphicalModel + PartialEq + Eq {
    /// Constructor of $\mathcal{B} = (\mathcal{G}, \Theta)$.
    fn new<I>(graph: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>;

    /// Construct $\mathcal{B}$ and the associated graph $\mathcal{G}$ given the parameters $\Theta$.
    fn with_parameters<I>(theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>;
}

/// Discrete Bayesian Network $\mathcal{B}$.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteBayesianNetwork {
    graph: DirectedDenseAdjacencyMatrixGraph,
    theta: FxIndexMap<String, DiscreteCPD>,
}

impl Display for DiscreteBayesianNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Iterate over the CPDs.
        for t in self.theta.values() {
            // Print CPD.
            writeln!(f, "{t}")?;
        }

        Ok(())
    }
}

impl From<DiscreteBayesianNetwork>
    for (
        DirectedDenseAdjacencyMatrixGraph,
        FxIndexMap<String, DiscreteCPD>,
    )
{
    fn from(b: DiscreteBayesianNetwork) -> Self {
        (b.graph, b.theta)
    }
}

impl ProbabilisticGraphicalModel for DiscreteBayesianNetwork {
    type Graph = DirectedDenseAdjacencyMatrixGraph;

    type Parameter = DiscreteCPD;

    type JPD = DiscreteJPD;

    type CPD = DiscreteCPD;

    #[inline]
    fn graph(&self) -> &Self::Graph {
        &self.graph
    }

    #[inline]
    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter> {
        // Assert parameters are sorted according to keys.
        debug_assert!(self.theta.keys().is_sorted());

        &self.theta
    }
}

impl PartialEq for DiscreteBayesianNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.graph == other.graph && self.theta == other.theta
    }
}

impl Eq for DiscreteBayesianNetwork {}

impl BayesianNetwork for DiscreteBayesianNetwork {
    fn new<I>(graph: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>,
    {
        // Get parameters target.
        let theta: FxIndexMap<_, _> = theta
            .into_iter()
            .map(|theta| (theta.target().to_owned(), theta))
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .collect();

        // Assert graph and parameters must contain the same variables.
        assert!(
            L!(graph).eq(theta.keys()),
            "Graph and parameters must contain the same variables"
        );
        // Assert graph and parameters must induce the same structure.
        assert!(
            V!(graph)
                .zip(L!(graph))
                .zip(theta.values())
                .all(|((i, x), t)| {
                    Pa!(graph, i)
                        .map(|y| graph.get_vertex_by_index(y))
                        .eq(t.scope().filter(|&z| z != x))
                }),
            "Graph and parameters must induce the same structure"
        );
        // Assert graph is acyclic.
        assert!(graph.is_acyclic(), "Graph must be acyclic");

        Self { graph, theta }
    }

    fn with_parameters<I>(theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>,
    {
        // Get parameters target.
        let theta: FxIndexMap<_, _> = theta
            .into_iter()
            .map(|theta| (theta.target().to_owned(), theta))
            .sorted_by(|(x, _), (y, _)| x.cmp(y))
            .collect();
        // Get vertices.
        let vertices = theta.keys().map(|x| x.as_str());
        // Get edges.
        let edges = theta.values().flat_map(|phi| {
            phi.states()
                .keys()
                .filter(|&z| z != phi.target())
                .map(|z| z.as_str())
                .zip(std::iter::repeat(phi.target()))
        });
        // Construct graph.
        let graph = Self::Graph::new(vertices, edges);

        Self { graph, theta }
    }
}

impl From<BIF> for DiscreteBayesianNetwork {
    fn from(bif: BIF) -> Self {
        Self::with_parameters(bif.theta)
    }
}

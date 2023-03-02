use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{DiscreteCPD, Factor};
use crate::{
    graphs::{directions, DiGraph, DirectedGraph},
    prelude::BaseGraph,
    types::FxIndexMap,
};

/// Bayesian Network trait.
pub trait BayesianNetwork: Clone + Debug + Display + Serialize + for<'a> Deserialize<'a> {
    /// Underlying directed graph associated type.
    type Graph: DirectedGraph<Direction = directions::Directed>;
    /// Parameter associated type.
    type Parameter: Factor;

    /// Constructor of $\mathcal{B} = (\mathcal{G}, \Theta)$.
    fn new<I>(graph: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>;

    /// Reference to the underlying graph.
    fn graph(&self) -> &Self::Graph;

    /// Reference to the parameters.
    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter>;
}

/// Discrete Bayesian Network implementation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteBayesianNetwork {
    graph: DiGraph,
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

impl BayesianNetwork for DiscreteBayesianNetwork {
    type Graph = DiGraph;

    type Parameter = DiscreteCPD;

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
            graph.labels().eq(theta.keys()),
            "Graph and parameters must contain the same variables"
        );

        Self { graph, theta }
    }

    #[inline]
    fn graph(&self) -> &Self::Graph {
        &self.graph
    }

    #[inline]
    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter> {
        &self.theta
    }
}

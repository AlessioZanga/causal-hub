use std::fmt::{Debug, Display};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use super::{ConditionalProbabilityDistribution, Factor, JointProbabilityDistribution};
use crate::{graphs::BaseGraph, prelude::DataSet, types::FxIndexMap};

/// Probabilistic Graphical Model (PGM) trait.
pub trait ProbabilisticGraphicalModel:
    Clone
    + Debug
    + Display
    + PartialEq
    + Eq
    + Serialize
    + for<'a> Deserialize<'a>
    + Into<(Self::Graph, FxIndexMap<String, Self::Parameter>)>
{
    /// Associated data set type.
    type Data: DataSet;
    /// Underlying directed graph associated type.
    type Graph: BaseGraph;
    /// Parameter associated type.
    type Parameter: Factor;

    /// Joint distribution associated type.
    type JPD: JointProbabilityDistribution<Phi = <Self::Parameter as Factor>::Phi>;
    /// Conditional distribution associated type.
    type CPD: ConditionalProbabilityDistribution<Phi = <Self::Parameter as Factor>::Phi>;

    /// Constructor of $\mathcal{B} = (\mathcal{G}, \Theta)$.
    fn new<I, V>(graph: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = (V, Self::Parameter)>,
        V: Into<String>;

    /// Reference to the underlying graph.
    fn graph(&self) -> &Self::Graph;

    /// Reference to the parameters.
    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter>;

    /// Draw `n` samples.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, n: usize) -> Self::Data;
}

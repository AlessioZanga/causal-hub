use std::fmt::{Debug, Display, Formatter};

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{DiscreteCPD, Factor};
use crate::{
    data::{DataSet, DiscreteDataMatrix},
    graphs::{directions, DiGraph, DirectedGraph},
    types::FxIndexMap,
};

pub trait BayesianNetwork: Clone + Debug + Display + Serialize + for<'a> Deserialize<'a> {
    type Data: DataSet;
    type Graph: DirectedGraph<Direction = directions::Directed>;
    type Parameter: Factor;

    fn new<I>(g: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>;

    fn graph(&self) -> &Self::Graph;

    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter>;

    fn sample<R>(&self, rng: &mut R, size: usize) -> Self::Data
    where
        R: Rng + ?Sized;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscreteBayesianNetwork {}

impl Display for DiscreteBayesianNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!() // FIXME:
    }
}

impl BayesianNetwork for DiscreteBayesianNetwork {
    type Data = DiscreteDataMatrix;

    type Graph = DiGraph;

    type Parameter = DiscreteCPD;

    fn new<I>(g: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>,
    {
        todo!() // FIXME:
    }

    fn graph(&self) -> &Self::Graph {
        todo!() // FIXME:
    }

    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter> {
        todo!() // FIXME:
    }

    fn sample<R>(&self, rng: &mut R, size: usize) -> Self::Data
    where
        R: Rng + ?Sized,
    {
        todo!() // FIXME:
    }
}

use std::fmt::{Debug, Display, Formatter};

use is_sorted::IsSorted;
use itertools::Itertools;
use ndarray::{prelude::*, SliceInfoElem as SIE};
use rand::{distributions::WeightedIndex, prelude::*};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    CategoricalCPD, CategoricalFactor, CategoricalJPD, ConditionalProbabilityDistribution, Factor,
    JointProbabilityDistribution,
};
use crate::{
    graphs::{directions, structs::DirectedDenseAdjacencyMatrix, DirectedGraph},
    io::BIF,
    prelude::{
        algorithms::traversal::TopologicalSort, CategoricalDataMatrix, DataSet, Graph, PathGraph,
    },
    types::FxIndexMap,
    Pa, L, V,
};

pub trait ProbabilisticGraphicalModel:
    Clone
    + Debug
    + Display
    + Serialize
    + for<'a> Deserialize<'a>
    + Into<(Self::Graph, FxIndexMap<String, Self::Parameter>)>
{
    type Data: DataSet;

    type Graph: DirectedGraph<Direction = directions::Directed>;
    /// Parameter associated type.
    type Parameter: Factor<Phi = Self::Phi>; // TODO: This patch is needed to avoid compiler recursion limit.
    /// Underlying factor type.
    type Phi: Factor;

    type JPD: JointProbabilityDistribution<Phi = <Self::Parameter as Factor>::Phi>;

    type CPD: ConditionalProbabilityDistribution<Phi = <Self::Parameter as Factor>::Phi>;

    fn graph(&self) -> &Self::Graph;

    fn parameters(&self) -> &FxIndexMap<String, Self::Parameter>;

    /// Draw `n` samples.
    fn sample<R: Rng>(&self, rng: &mut R, n: usize) -> Self::Data;

    /// Draw `n` samples in parallel.
    fn par_sample<R: Rng + SeedableRng + Send>(&self, rng: &mut R, n: usize) -> Self::Data;
}

pub trait BayesianNetwork: ProbabilisticGraphicalModel + PartialEq + Eq {
    fn new<I>(graph: Self::Graph, theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>;

    fn with_parameters<I>(theta: I) -> Self
    where
        I: IntoIterator<Item = Self::Parameter>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoricalBayesianNetwork {
    graph: DirectedDenseAdjacencyMatrix,
    theta: FxIndexMap<String, CategoricalCPD>,
}

impl Display for CategoricalBayesianNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Iterate over the CPDs.
        for t in self.theta.values() {
            // Print CPD.
            writeln!(f, "{t}")?;
        }

        Ok(())
    }
}

impl From<CategoricalBayesianNetwork>
    for (
        DirectedDenseAdjacencyMatrix,
        FxIndexMap<String, CategoricalCPD>,
    )
{
    fn from(b: CategoricalBayesianNetwork) -> Self {
        (b.graph, b.theta)
    }
}

impl ProbabilisticGraphicalModel for CategoricalBayesianNetwork {
    type Data = CategoricalDataMatrix;

    type Graph = DirectedDenseAdjacencyMatrix;

    type Parameter = CategoricalCPD;

    type Phi = CategoricalFactor;

    type JPD = CategoricalJPD;

    type CPD = CategoricalCPD;

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

    fn sample<R: Rng>(&self, rng: &mut R, n: usize) -> Self::Data {
        // Allocate the new data set values.
        let mut data = Array2::<u8>::zeros((n, self.graph.order()));
        // Get topological sort of the underlying graph.
        let order = TopologicalSort::new(&self.graph);

        // For each vertex in the graph ...
        for x in order {
            // Get Pa(X).
            let pa_x = Pa!(self.graph, x).collect_vec();
            // Compute insertion index to align X in Pa(X) vector.
            let in_x = pa_x.binary_search(&x).unwrap_err();
            // Get the factor Phi(X).
            let phi_x = &self.theta[x];

            // For each sample ...
            data.rows_mut().into_iter().for_each(|mut row| {
                // Allocate P(X | Pa(X)) indices.
                let mut indices = Vec::with_capacity(self.graph.order());
                // Set P(X | Pa(X)) indices.
                indices.extend(pa_x.iter().map(|&z| SIE::Index(row[z] as isize)));
                indices.insert(in_x, (..).into());
                // Get P(X | Pa(X)) values.
                let weights = phi_x.values().slice(indices.as_slice());
                // Sample from P(X | Pa(X)).
                let sample = WeightedIndex::new(&weights).unwrap().sample(rng);
                // Assign sampled values.
                row[x] = sample.try_into().unwrap();
            });
        }

        // Get the states.
        let states = self
            .theta
            .iter()
            .map(|(k, v)| (k.into(), v.states()[k].clone()))
            .collect();

        // Return sampled data set.
        Self::Data::with_data_labels(data, states)
    }

    fn par_sample<R: Rng + SeedableRng + Send>(&self, rng: &mut R, n: usize) -> Self::Data {
        // Allocate the new data set values.
        let mut data = Array2::<u8>::zeros((n, self.graph.order()));
        // Get topological sort of the underlying graph.
        let order = TopologicalSort::new(&self.graph);

        // Initialize seeds for parallel rngs.
        let seeds = (0..n).map(|_| rng.next_u64()).collect_vec();
        // Initialize parallel rngs.
        let mut rngs = Vec::with_capacity(n);
        seeds
            .into_par_iter()
            .map(|seed| R::seed_from_u64(seed))
            .collect_into_vec(&mut rngs);

        // For each vertex in the graph ...
        for x in order {
            // Get Pa(X).
            let pa_x = Pa!(self.graph, x).collect_vec();
            // Compute insertion index to align X in Pa(X) vector.
            let in_x = pa_x.binary_search(&x).unwrap_err();
            // Get the factor Phi(X).
            let phi_x = &self.theta[x];

            // For each sample ...
            rngs.par_iter_mut()
                .zip(data.axis_iter_mut(Axis(0)))
                .for_each(|(rng, mut row)| {
                    // Allocate P(X | Pa(X)) indices.
                    let mut indices = Vec::with_capacity(self.graph.order());
                    // Set P(X | Pa(X)) indices.
                    indices.extend(pa_x.iter().map(|&z| SIE::Index(row[z] as isize)));
                    indices.insert(in_x, (..).into());
                    // Get P(X | Pa(X)) values.
                    let weights = phi_x.values().slice(indices.as_slice());
                    // Sample from P(X | Pa(X)).
                    let sample = WeightedIndex::new(&weights).unwrap().sample(rng);
                    // Assign sampled values.
                    row[x] = sample.try_into().unwrap();
                });
        }

        // Get the states.
        let states = self
            .theta
            .iter()
            .map(|(k, v)| (k.into(), v.states()[k].clone()))
            .collect();

        // Return sampled data set.
        Self::Data::with_data_labels(data, states)
    }
}

impl PartialEq for CategoricalBayesianNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.graph == other.graph && self.theta == other.theta
    }
}

impl Eq for CategoricalBayesianNetwork {}

impl BayesianNetwork for CategoricalBayesianNetwork {
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
                        .map(|y| &graph[y])
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

impl From<BIF> for CategoricalBayesianNetwork {
    fn from(bif: BIF) -> Self {
        Self::with_parameters(bif.theta)
    }
}

use ndarray::prelude::*;
use rand::{
    Rng,
    distr::{Distribution as _Distribution, weighted::WeightedIndex},
};

use super::BNSampler;
use crate::{
    datasets::CategoricalDataset,
    distributions::CPD,
    graphs::{Graph, TopologicalOrder},
    models::{BayesianNetwork, CategoricalBN},
    utils::RMI,
};

/// A forward sampler.
#[derive(Clone, Copy, Debug, Default)]
pub struct ForwardSampler;

impl ForwardSampler {
    /// Construct a new forward sampler.
    ///
    /// # Returns
    ///
    /// Return a new `ForwardSampler` instance.
    ///
    #[inline]
    pub const fn new() -> Self {
        Self
    }
}

impl BNSampler<CategoricalBN, CategoricalDataset> for ForwardSampler {
    fn sample<R>(&self, rng: &mut R, bn: &CategoricalBN, n: usize) -> CategoricalDataset
    where
        R: Rng,
    {
        // Get the graph.
        let graph = bn.graph();
        // Allocate the dataset.
        let mut dataset: Array2<u8> = Array::zeros((n, bn.labels().len()));
        // Compute the topological order of the graph.
        let order = graph
            .topological_order()
            .expect("Failed to compute topological order.");

        // Cache the parents.
        let parents: Vec<_> = graph.vertices().map(|x| graph.parents(x)).collect();
        // Cache the RMIs.
        let idx: Vec<_> = bn
            .cpds()
            .values()
            .map(|cpd| RMI::new(cpd.conditioning_cardinality().iter().copied()))
            .collect();

        // For each vertex in the topological order ...
        order.into_iter().for_each(|x| {
            // Get the parents of the vertex.
            let pa_x = &parents[x];
            // Get the RMI of the vertex.
            let rmi_x = &idx[x];
            // Get the CPD of the vertex.
            let cpd_x = bn.cpds()[x].parameters();
            // For each sample ...
            dataset.rows_mut().into_iter().for_each(|mut row| {
                // Compute the index on the parents to condition on.
                // NOTE: We can to this because the labels and states are sorted (i.e. aligned).
                let pa_i = rmi_x.ravel(pa_x.iter().map(|&z| row[z] as usize));
                // Get the distribution of the vertex.
                let p_x = cpd_x.row(pa_i);
                // Construct the sampler.
                let s_x = WeightedIndex::new(p_x).expect("Failed to construct sampler.");
                // Sample from the distribution.
                row[x] = s_x.sample(rng) as u8;
            });
        });

        // Get the states.
        let states = bn.cpds().iter().map(|(label, cpd)| (label, cpd.states()));

        // Construct the dataset.
        CategoricalDataset::new(states, dataset)
    }
}

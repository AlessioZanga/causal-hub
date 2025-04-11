use ndarray::prelude::*;
use rand::{
    Rng,
    distr::{Distribution as _Distribution, weighted::WeightedIndex},
};

use super::BNSampler;
use crate::{
    data::CategoricalData,
    distribution::CPD,
    graph::{Graph, TopologicalOrder},
    model::{BayesianNetwork, CategoricalBN},
    utils::RMI,
};

/// A forward sampler.
pub struct ForwardSampler;

impl BNSampler<CategoricalBN, CategoricalData> for ForwardSampler {
    fn sample<R>(&self, rng: &mut R, bn: &CategoricalBN, n: usize) -> CategoricalData
    where
        R: Rng,
    {
        // Get the graph.
        let graph = bn.graph();
        // Allocate the data.
        let mut data: Array2<u8> = Array::zeros((n, bn.labels().len()));
        // Compute the topological order of the graph.
        let order = graph
            .topological_order()
            .expect("Failed to compute topological order.");

        // Cache the parents.
        let parents: Vec<_> = graph.vertices().map(|x| graph.parents(x)).collect();
        // Cache the RMIs.
        let rmi: Vec<_> = bn
            .cpds()
            .values()
            .map(|cpd| RMI::new(cpd.conditioning_cardinality().iter().copied()))
            .collect();

        // For each vertex in the topological order ...
        order.into_iter().for_each(|x| {
            // Get the parents of the vertex.
            let pa_x = &parents[x];
            // Get the RMI of the vertex.
            let rmi_x = &rmi[x];
            // Get the CPD of the vertex.
            let cpd_x = bn.cpds()[x].parameters();
            // For each sample ...
            data.rows_mut().into_iter().for_each(|mut row| {
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

        // Construct the data.
        CategoricalData::new(states, data)
    }
}

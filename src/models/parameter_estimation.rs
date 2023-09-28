use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use super::CategoricalBayesianNetwork;
use crate::{
    data::{CategoricalDataMatrix, DataSet},
    graphs::{structs::DirectedDenseAdjacencyMatrix, DirectedGraph, Graph},
    prelude::{BayesianNetwork, CategoricalCPD, ConditionalCountMatrix, MarginalCountMatrix},
    Pa, L, V,
};

pub trait ParameterEstimation<D, G, M>
where
    D: DataSet,
    G: Graph,
{
    fn call(d: &D, g: &G) -> M;
}

pub struct MaximumLikelihoodEstimation<const PARALLEL: bool> {}

impl<const PARALLEL: bool>
    ParameterEstimation<
        CategoricalDataMatrix,
        DirectedDenseAdjacencyMatrix,
        CategoricalBayesianNetwork,
    > for MaximumLikelihoodEstimation<PARALLEL>
{
    fn call(
        d: &CategoricalDataMatrix,
        g: &DirectedDenseAdjacencyMatrix,
    ) -> CategoricalBayesianNetwork {
        // Assert dataset and graph have same labels.
        assert!(L!(g).eq(d.labels_iter()));

        // Estimate parameters of a given variable.
        let estimate = |x: usize| {
            // Compute the parents set.
            let z = Pa!(g, x).collect_vec();
            // Compute the absolute frequencies.
            let n = match z.is_empty() {
                true => Array1::from(MarginalCountMatrix::new(d, x)).insert_axis(Axis(0)),
                false => ConditionalCountMatrix::new(d, x, &z).into(),
            };
            // Cast to float.
            let n = n.mapv(|n| n as f64);
            // Compute marginal sums.
            let n_i = n.sum_axis(Axis(1)).insert_axis(Axis(1));
            // Check that at least one configuration for each parent set is observed.
            assert!(
                n_i.iter().all(|&n_i| n_i > 0.),
                "At least one configuration for each parent set must be observed"
            );
            // Get target label and states.
            let (x, y) = (&g[x], d.states()[x].clone());
            // Get conditioning variables labels and states.
            let z = z
                .into_iter()
                .map(|z| (g.vertex_to_label(z), d.states()[z].clone()));
            // Construct CPD from states and values.
            CategoricalCPD::new((x, y), z, n / n_i)
        };

        // Preallocate memory for parameters.
        let mut theta = Vec::with_capacity(g.order());

        // Perform parameters estimation.
        match PARALLEL {
            true => (0..g.order())
                .into_par_iter()
                .map(estimate)
                .collect_into_vec(&mut theta),
            false => theta.extend(V!(g).map(estimate)),
        };

        CategoricalBayesianNetwork::new(g.clone(), theta)
    }
}

pub struct BayesianEstimation<const PARALLEL: bool> {}

impl<const PARALLEL: bool>
    ParameterEstimation<
        CategoricalDataMatrix,
        DirectedDenseAdjacencyMatrix,
        CategoricalBayesianNetwork,
    > for BayesianEstimation<PARALLEL>
{
    fn call(
        d: &CategoricalDataMatrix,
        g: &DirectedDenseAdjacencyMatrix,
    ) -> CategoricalBayesianNetwork {
        // Assert dataset and graph have same labels.
        assert!(L!(g).eq(d.labels_iter()));

        // Estimate parameters of a given variable.
        let estimate = |x: usize| {
            // Compute the parents set.
            let z = Pa!(g, x).collect_vec();
            // Compute the absolute frequencies.
            let n = match z.is_empty() {
                true => Array1::from(MarginalCountMatrix::new(d, x)).insert_axis(Axis(0)),
                false => ConditionalCountMatrix::new(d, x, &z).into(),
            };
            // Add pseudo counts. // TODO: Generalize to non-uniform distributions.
            let n = n + 1;
            // Cast to float.
            let n = n.mapv(|n| n as f64);
            // Compute marginal sums.
            let n_i = n.sum_axis(Axis(1)).insert_axis(Axis(1));
            // Check that at least one configuration for each parent set is observed.
            assert!(
                n_i.iter().all(|&n_i| n_i > 0.),
                "At least one configuration for each parent set must be observed"
            );
            // Get target label and states.
            let (x, y) = (&g[x], d.states()[x].clone());
            // Get conditioning variables labels and states.
            let z = z
                .into_iter()
                .map(|z| (g.vertex_to_label(z), d.states()[z].clone()));
            // Construct CPD from states and values.
            CategoricalCPD::new((x, y), z, n / n_i)
        };

        // Preallocate memory for parameters.
        let mut theta = Vec::with_capacity(g.order());

        // Perform parameters estimation.
        match PARALLEL {
            true => (0..g.order())
                .into_par_iter()
                .map(estimate)
                .collect_into_vec(&mut theta),
            false => theta.extend(V!(g).map(estimate)),
        };

        CategoricalBayesianNetwork::new(g.clone(), theta)
    }
}

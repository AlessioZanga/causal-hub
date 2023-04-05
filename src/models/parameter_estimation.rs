use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use super::DiscreteBayesianNetwork;
use crate::{
    data::{DataSet, DiscreteDataMatrix},
    graphs::{structs::DirectedDenseAdjacencyMatrixGraph, BaseGraph, DirectedGraph},
    prelude::{BayesianNetwork, ConditionalCountMatrix, DiscreteCPD, MarginalCountMatrix},
    Pa, L, V,
};

/// Parameter estimation trait for model $\mathcal{M}$ given data $\mathcal{D}$ and graph $\mathcal{G}$.
pub trait ParameterEstimation<D, G, M>
where
    D: DataSet,
    G: BaseGraph,
{
    /// Construct the model $\mathcal{M}$ given data $\mathcal{D}$ and graph $\mathcal{G}$.
    fn call(d: &D, g: &G) -> M;
}

/// Maximum Likelihood Estimation (MLE) functor.
pub struct MaximumLikelihoodEstimation<const PARALLEL: bool> {}

impl<const PARALLEL: bool>
    ParameterEstimation<
        DiscreteDataMatrix,
        DirectedDenseAdjacencyMatrixGraph,
        DiscreteBayesianNetwork,
    > for MaximumLikelihoodEstimation<PARALLEL>
{
    fn call(
        d: &DiscreteDataMatrix,
        g: &DirectedDenseAdjacencyMatrixGraph,
    ) -> DiscreteBayesianNetwork {
        // Assert dataset and graph have same labels.
        assert!(L!(g).eq(d.labels()));

        // Estimate parameters of a given variable.
        let estimate = |x: usize| {
            // Compute the parents set.
            let z = Pa!(g, x).collect_vec();
            // Compute the absolute frequencies.
            let n = match z.is_empty() {
                true => Array1::from(MarginalCountMatrix::new(d, x)).insert_axis(Axis(0)),
                false => ConditionalCountMatrix::<false>::new(d, x, &z).into(),
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
            let (x, y) = (g.get_vertex_by_index(x), d.states()[x].clone());
            // Get conditioning variables labels and states.
            let z = z
                .into_iter()
                .map(|z| (g.get_vertex_by_index(z), d.states()[z].clone()));
            // Construct CPD from states and values.
            DiscreteCPD::new((x, y), z, n / n_i)
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

        DiscreteBayesianNetwork::new(g.clone(), theta)
    }
}

/// Bayesian Estimation (BE) functor.
pub struct BayesianEstimation<const PARALLEL: bool> {}

impl<const PARALLEL: bool>
    ParameterEstimation<
        DiscreteDataMatrix,
        DirectedDenseAdjacencyMatrixGraph,
        DiscreteBayesianNetwork,
    > for BayesianEstimation<PARALLEL>
{
    fn call(
        d: &DiscreteDataMatrix,
        g: &DirectedDenseAdjacencyMatrixGraph,
    ) -> DiscreteBayesianNetwork {
        // Assert dataset and graph have same labels.
        assert!(L!(g).eq(d.labels()));

        // Estimate parameters of a given variable.
        let estimate = |x: usize| {
            // Compute the parents set.
            let z = Pa!(g, x).collect_vec();
            // Compute the absolute frequencies.
            let n = match z.is_empty() {
                true => Array1::from(MarginalCountMatrix::new(d, x)).insert_axis(Axis(0)),
                false => ConditionalCountMatrix::<false>::new(d, x, &z).into(),
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
            let (x, y) = (g.get_vertex_by_index(x), d.states()[x].clone());
            // Get conditioning variables labels and states.
            let z = z
                .into_iter()
                .map(|z| (g.get_vertex_by_index(z), d.states()[z].clone()));
            // Construct CPD from states and values.
            DiscreteCPD::new((x, y), z, n / n_i)
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

        DiscreteBayesianNetwork::new(g.clone(), theta)
    }
}

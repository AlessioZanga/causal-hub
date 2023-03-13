use itertools::Itertools;
use ndarray::prelude::*;

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

/// Maximum Likelihood Estimation (MLE).
pub struct MaximumLikelihoodEstimation {}

impl
    ParameterEstimation<
        DiscreteDataMatrix,
        DirectedDenseAdjacencyMatrixGraph,
        DiscreteBayesianNetwork,
    > for MaximumLikelihoodEstimation
{
    fn call(
        d: &DiscreteDataMatrix,
        g: &DirectedDenseAdjacencyMatrixGraph,
    ) -> DiscreteBayesianNetwork {
        // Assert dataset and graph have same labels.
        assert!(L!(g).eq(L!(d)));

        // For each vertex ... TODO: Parallelize over vertices.
        let theta = V!(g)
            // ... compute its parents set ...
            .map(|x| (x, Pa!(g, x).collect_vec()))
            // ... compute the absolute frequencies ...
            .map(|(x, z)| {
                let n = match z.is_empty() {
                    true => Array1::from(MarginalCountMatrix::new(d, x)).insert_axis(Axis(0)),
                    false => ConditionalCountMatrix::<false>::new(d, x, &z).into(),
                };

                (x, z, n)
            })
            // ... cast to float ...
            .map(|(x, z, n)| (x, z, n.mapv(|n| n as f64)))
            // ... compute relative frequencies ...
            .map(|(x, z, n)| {
                // Compute marginal sums.
                let n_i = n.sum_axis(Axis(1)).insert_axis(Axis(1));
                // Check that at least one configuration for each parent set is observed.
                assert!(
                    n_i.iter().all(|&n_i| n_i > 0.),
                    "At least one configuration for each parent set must be observed"
                );

                (x, z, n / n_i)
            })
            // ... construct parameters.
            .map(|(x, z, n)| {
                // Get target label and states.
                let (x, y) = (g.label(x), d.states()[x].clone());
                // Get conditioning variables labels and states.
                let z = z.into_iter().map(|z| (g.label(z), d.states()[z].clone()));
                // Construct CPD from states and values.
                DiscreteCPD::new((x, y), z, n)
            });

        DiscreteBayesianNetwork::new(g.clone(), theta)
    }
}

/// Bayesian Estimation (BE) with given Prior Distribution.
pub struct BayesianEstimation {}

impl
    ParameterEstimation<
        DiscreteDataMatrix,
        DirectedDenseAdjacencyMatrixGraph,
        DiscreteBayesianNetwork,
    > for BayesianEstimation
{
    fn call(
        d: &DiscreteDataMatrix,
        g: &DirectedDenseAdjacencyMatrixGraph,
    ) -> DiscreteBayesianNetwork {
        // Assert dataset and graph have same labels.
        assert!(L!(g).eq(L!(d)));

        // For each vertex ... TODO: Parallelize over vertices.
        let theta = V!(g)
            // ... compute its parents set ...
            .map(|x| (x, Pa!(g, x).collect_vec()))
            // ... compute the absolute frequencies ...
            .map(|(x, z)| {
                let n = match z.is_empty() {
                    true => Array1::from(MarginalCountMatrix::new(d, x)).insert_axis(Axis(0)),
                    false => ConditionalCountMatrix::<false>::new(d, x, &z).into(),
                };

                (x, z, n)
            })
            // ... cast to float ...
            .map(|(x, z, n)| (x, z, n.mapv(|n| n as f64)))
            // ... add pseudo counts ... TODO: Generalize to non-uniform distributions.
            .map(|(x, z, n)| (x, z, n + 1.))
            // ... compute relative frequencies ...
            .map(|(x, z, n)| {
                // Compute marginal sums.
                let n_i = n.sum_axis(Axis(1)).insert_axis(Axis(1));

                (x, z, n / n_i)
            })
            // ... construct parameters.
            .map(|(x, z, n)| {
                // Get target label and states.
                let (x, y) = (g.label(x), d.states()[x].clone());
                // Get conditioning variables labels and states.
                let z = z.into_iter().map(|z| (g.label(z), d.states()[z].clone()));
                // Construct CPD from states and values.
                DiscreteCPD::new((x, y), z, n)
            });

        DiscreteBayesianNetwork::new(g.clone(), theta)
    }
}

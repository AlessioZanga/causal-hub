use itertools::Itertools;
use ndarray::prelude::*;

use super::DiscreteBayesianNetwork;
use crate::{
    data::{DataSet, DiscreteDataMatrix},
    graphs::{structs::DirectedDenseAdjacencyMatrixGraph, BaseGraph, DirectedGraph},
    prelude::{BayesianNetwork, ConditionalCountMatrix, DiscreteCPD},
    Pa, L, V,
};

/// Parameter estimator trait for model $\mathcal{M}$ given data $\mathcal{D}$ and graph $\mathcal{G}$.
pub trait ParameterEstimator<D, G, M>
where
    D: DataSet,
    G: BaseGraph,
{
    /// Construct the model $\mathcal{M}$ given data $\mathcal{D}$ and graph $\mathcal{G}$.
    fn call(d: &D, g: &G) -> M;
}

/// Maximum Likelihood Estimator (MLE).
pub struct MaximumLikelihoodEstimator {}

impl
    ParameterEstimator<
        DiscreteDataMatrix,
        DirectedDenseAdjacencyMatrixGraph,
        DiscreteBayesianNetwork,
    > for MaximumLikelihoodEstimator
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
            // ... compute the relative frequencies ...
            .map(|(x, z)| {
                // Compute the absolute frequencies.
                let n = ConditionalCountMatrix::<false>::new(d, x, &z);
                // Get the underlying counts and cast to float.
                let n: Array2<usize> = n.into();
                let n = n.mapv(|n| n as f64);
                // Compute marginal sums.
                let n_i = n.sum_axis(Axis(1)).insert_axis(Axis(1));
                // Check that at least one configuration for each parent set is observed.
                assert!(
                    n_i.iter().all(|&n_i| n_i > 0.),
                    "At least one configuration for each parent set must be observed"
                );
                // Compute relative frequencies.
                (x, z, n / n_i)
            })
            // ... construct parameters.
            .map(|(x, z, values)| {
                // Get target label and states.
                let (x, y) = (g.label(x), d.states()[x].clone());
                // Get conditioning variables labels and states.
                let z = z.into_iter().map(|z| (g.label(z), d.states()[z].clone()));
                // Construct CPD from states and values.
                DiscreteCPD::new((x, y), z, values)
            });

        DiscreteBayesianNetwork::new(g.clone(), theta)
    }
}

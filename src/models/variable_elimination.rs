use std::{collections::BTreeSet, ops::Mul};

use itertools::Itertools;
use rayon::prelude::*;
use split_iter::Splittable;

use super::{
    BayesianNetwork, DistributionEstimation, DistributionProjection, ProbabilisticGraphicalModel,
};
use crate::{
    graphs::BaseGraph,
    models::{ConditionalProbabilityDistribution, Factor, JointProbabilityDistribution},
    prelude::{DirectedGraph, FxIndexMap},
    types::FxIndexSet,
    Adj, Pa, L, V,
};

/// Variable Elimination (VE) functor.
#[derive(Clone, Debug)]
pub struct VariableElimination<'a, M, const PARALLEL: bool> {
    model: &'a M,
}

impl<'a, M, const PARALLEL: bool> VariableElimination<'a, M, PARALLEL> {
    /// Construct a new variable elimination functor.
    pub const fn new(model: &'a M) -> Self {
        Self { model }
    }

    /// Compute the sum-product of $\pmb{\Phi}$ w.r.t. given elimination order $\mathbf{Z}$.
    ///
    /// # Panics
    ///
    /// Panics if $\pmb{\Phi}$ is empty, or when $\mathbf{Z}$ is not a subset of the scope of $\pmb{\Phi}$.
    ///
    fn sum_product<'b, I, P, Z>(phi: I, z: Z) -> P
    where
        I: IntoIterator<Item = P> + FromIterator<P> + IntoParallelIterator<Item = P>,
        P: Factor,
        Z: IntoIterator<Item = &'b str>,
        Self: 'a,
    {
        // Apply variable elimination to the given variables.
        let phi = z.into_iter().fold(phi, Self::variable_elimination);
        // Compute the factor product. TODO: Change `reduce` and `reduce_with` to `fold`.
        match PARALLEL {
            true => phi.into_par_iter().reduce_with(Mul::mul).unwrap(),
            false => phi.into_iter().reduce(Mul::mul).unwrap(),
        }
    }

    /// Perform variable elimination w.r.t. the given variable $Z$.
    ///
    /// # Panics
    ///
    /// Panics if $\mathbf{Z}$ is not a subset of the scope of $\pmb{\Phi}$.
    ///
    fn variable_elimination<P>(phi: P, z: &str) -> P
    where
        P: IntoIterator + FromIterator<P::Item>,
        P::Item: Factor,
    {
        // Split factors when the given variable is in their scope.
        let (phi_prime, phi_dprime) = phi.into_iter().split(|phi| !phi.in_scope(z));
        // Compute the factor product. TODO: Change `reduce` and `reduce_with` to `fold`.
        let psi = match PARALLEL {
            true => phi_prime
                .collect_vec()
                .into_par_iter()
                .reduce_with(Mul::mul)
                .unwrap(),
            false => phi_prime.reduce(Mul::mul).unwrap(),
        };
        // Eliminate variable by marginalization.
        let tau = psi.marginalize([z]);
        // Return new sum-product factor.
        phi_dprime.chain([tau]).collect()
    }
}

impl<'a, M, const PARALLEL: bool> VariableElimination<'a, M, PARALLEL>
where
    M: ProbabilisticGraphicalModel,
{
    /// Compute the elimination order w.r.t. the given variables $\mathbf{Z}$.
    fn elimination_order<'b, Z>(&self, z: Z) -> Vec<&'b str>
    where
        Z: IntoIterator<Item = &'b str>,
    {
        // Get associated graph.
        let g = self.model.graph();
        // Initialize an empty elimination order.
        let mut order = Vec::with_capacity(g.order());
        // Initialize the set of variables to be ordered.
        let mut queue: FxIndexSet<_> = z.into_iter().collect();
        // Clone the associated adjacencies.
        let mut g: FxIndexMap<_, FxIndexSet<_>> = V!(g)
            .map(|x| {
                (
                    g.get_vertex_by_index(x),
                    Adj!(g, x).map(|x| g.get_vertex_by_index(x)).collect(),
                )
            })
            .collect();
        // While there are still variables to be ordered.
        while !queue.is_empty() {
            // Compute the "cost" of each variable.
            let z = *queue
                .iter()
                // Select the variable with minimum cost.
                // NOTE: This uses the `MinFill` cost function, kinda.
                .min_by_key(|&z| g[z].len())
                .unwrap();
            // Add it to the elimination order.
            order.push(z);
            // Remove it from the to-be-ordered set.
            queue.remove(&z);
            // Remove it from the associated adjacencies.
            g.remove(&z);
            g.values_mut().for_each(|x| {
                x.remove(&z);
            });
        }

        order
    }

    /// Perform variable elimination w.r.t. the given variables $X$.
    ///
    /// # Panics
    ///
    /// Panics if $\mathbf{X}$ is not a subset of the scope of $\pmb{\Phi}$.
    ///
    pub fn call<'b, X>(&self, x: X) -> <M::Parameter as Factor>::Phi
    where
        X: IntoIterator<Item = &'b str>,
    {
        // Sort and deduplicate query variables.
        let x: BTreeSet<_> = x.into_iter().collect();
        // Get variables labels.
        let z = L!(self.model.graph());
        // Get the variables that needs to be eliminated.
        let z = iter_set::difference(z, x);
        // Compute the elimination order.
        let z = self.elimination_order(z);
        // Get the parameters.
        let phi = self
            .model
            .parameters()
            .values()
            .cloned()
            .map(|phi| phi.into())
            .collect_vec();
        // Execute variable elimination.
        Self::sum_product(phi, z)
    }
}

impl<'a, M, const PARALLEL: bool> DistributionEstimation for VariableElimination<'a, M, PARALLEL>
where
    M: ProbabilisticGraphicalModel,
{
    type JPD = M::JPD;

    type CPD = M::CPD;

    fn marginal(&self, x: &str) -> Self::JPD {
        Self::JPD::from_factor(self.call([x]))
    }

    fn joint<'b, X>(&self, x: X) -> Self::JPD
    where
        X: IntoIterator<Item = &'b str>,
    {
        Self::JPD::from_factor(self.call(x))
    }

    fn conditional<'b, Z>(&self, x: &'b str, z: Z) -> Self::CPD
    where
        Z: IntoIterator<Item = &'b str>,
    {
        Self::CPD::from_factor(x, self.call([x].into_iter().chain(z)))
    }
}

impl<'a, M, const PARALLEL: bool> DistributionProjection for VariableElimination<'a, M, PARALLEL>
where
    M: BayesianNetwork<Parameter = <Self as DistributionEstimation>::CPD>,
    M::Graph: DirectedGraph,
{
    type Projection = M;

    fn project_onto(&self, q: &Self::Projection) -> Self::Projection {
        // Get underlying graphs of Q.
        let g_q = q.graph();
        // Assert P and Q labels are the same.
        assert!(L!(self.model.graph()).eq(L!(g_q)));
        // Project P parameters onto Q structure.
        let theta = V!(g_q)
            // Get the parents of each vertex.
            .map(|x| {
                (
                    g_q.get_vertex_by_index(x),
                    Pa!(g_q, x).map(|z| g_q.get_vertex_by_index(z)),
                )
            })
            // Project P parameters onto Q structure.
            .map(|(x, z)| self.conditional(x, z));
        // Construct projection of P given projected parameters.
        Self::Projection::with_parameters(theta)
    }
}

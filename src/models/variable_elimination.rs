use std::{collections::BTreeSet, ops::Mul};

use split_iter::Splittable;

use crate::{
    graphs::BaseGraph,
    models::{BayesianNetwork, Factor},
    prelude::FxIndexMap,
    types::FxIndexSet,
    Adj, L, V,
};

/// Variable Elimination functor.
#[derive(Clone, Debug)]
pub struct VariableElimination<'a, M> {
    model: &'a M,
}

impl<'a, M> VariableElimination<'a, M> {
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
    pub fn sum_product<'b, P, Z>(phi: P, z: Z) -> P::Item
    where
        P: IntoIterator + FromIterator<P::Item>,
        P::Item: Factor,
        Z: IntoIterator<Item = &'b str>,
        Self: 'a,
    {
        // Apply variable elimination to the given variables.
        let phi = z.into_iter().fold(phi, Self::variable_elimination);
        // Compute the factor product.
        phi.into_iter().reduce(Mul::mul).unwrap()
    }

    /// Perform variable elimination w.r.t. the given variable $Z$.
    ///
    /// # Panics
    ///
    /// Panics if $\mathbf{Z}$ is not a subset of the scope of $\pmb{\Phi}$.
    ///
    pub fn variable_elimination<P>(phi: P, z: &str) -> P
    where
        P: IntoIterator + FromIterator<P::Item>,
        P::Item: Factor,
    {
        // Split factors when the given variable is in their scope.
        let (phi_prime, phi_dprime) = phi.into_iter().split(|phi| !phi.in_scope(z));
        // Compute the factor product.
        let psi = phi_prime.reduce(Mul::mul).unwrap();
        // Eliminate variable by marginalization.
        let tau = psi.marginalize([z]);
        // Return new sum-product factor.
        phi_dprime.chain([tau]).collect()
    }
}

impl<'a, M> VariableElimination<'a, M>
where
    M: BayesianNetwork,
{
    /// Compute the elimination order w.r.t. the given variables $\mathbf{Z}$.
    pub fn elimination_order<'b, Z>(&self, z: Z) -> Vec<&'b str>
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
            .map(|x| (g.label(x), Adj!(g, x).map(|x| g.label(x)).collect()))
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

    /// Execute the query for $\phi(\mathbf{X})$.
    pub fn query<'b, X, Y>(&self, x: X) -> Y
    where
        X: IntoIterator<Item = &'b str>,
        Y: Factor + From<M::Parameter>,
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
        let phi: Vec<_> = self
            .model
            .parameters()
            .values()
            .cloned()
            .map(|phi| phi.into())
            .collect();
        // Execute variable elimination.
        Self::sum_product(phi, z)
    }
}

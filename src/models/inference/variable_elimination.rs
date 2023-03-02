use std::ops::Mul;

use split_iter::Splittable;

use crate::models::{BayesianNetwork, Factor};

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

    /// Compute the sum-product with given elimination order.
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

    /// Perform variable elimination w.r.t. the given variable.
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
    /// Execute the query for $\phi(\mathbf{X})$.
    pub fn query<'b, X, Y>(&self, x: X) -> Y
    where
        M::Parameter: Into<Y>,
        X: IntoIterator<Item = &'b str>,
        Y: Factor,
    {
        // FIXME: Compute the variables that needs to be eliminated.
        let z = x.into_iter();
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

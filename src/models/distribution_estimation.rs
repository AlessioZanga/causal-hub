/// Trait for marginal $\mathcal{P}(X)$, joint $\mathcal{P}(\mathbf{X})$ and conditional $\mathcal{P}(X \mid \mathbf{Z})$ distribution estimation.
pub trait DistributionEstimation {
    /// Marginal distribution associated type.
    type Marginal;
    /// Joint distribution associated type.
    type Joint;
    /// Conditional distribution associated type.
    type Conditional;

    /// Compute the marginal distribution $\mathcal{P}(X)$.
    fn marginal(&self, x: &str) -> Self::Marginal;

    /// Compute the joint distribution $\mathcal{P}(\mathbf{X})$.
    fn joint<'a, X>(&self, x: X) -> Self::Joint
    where
        X: IntoIterator<Item = &'a str>;

    /// Compute the conditional distribution $\mathcal{P}(X \mid \mathbf{Z})$.
    fn conditional<'a, Z>(&self, x: &'a str, z: Z) -> Self::Conditional
    where
        Z: IntoIterator<Item = &'a str>;
}

/// Trait for marginal $\mathcal{P}(X)$, joint $\mathcal{P}(\mathbf{X})$ and conditional $\mathcal{P}(X \mid \mathbf{Z})$ query estimation.
pub trait QueryEstimation {
    /// Marginal query associated type.
    type Marginal;
    /// Joint query associated type.
    type Joint;
    /// Conditional query associated type.
    type Conditional;

    /// Compute the marginal query $\mathcal{P}(X)$.
    fn marginal(&self, x: &str) -> Self::Marginal;

    /// Compute the joint query $\mathcal{P}(\mathbf{X})$.
    fn joint<'a, X>(&self, x: X) -> Self::Joint
    where
        X: IntoIterator<Item = &'a str>;

    /// Compute the conditional query $\mathcal{P}(X \mid \mathbf{Z})$.
    fn conditional<'a, Z>(&self, x: &'a str, z: Z) -> Self::Conditional
    where
        Z: IntoIterator<Item = &'a str>;
}

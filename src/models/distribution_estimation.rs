use super::{ConditionalProbabilityDistribution, JointProbabilityDistribution};

/// Marginal $\mathcal{P}(X)$, joint $\mathcal{P}(\mathbf{X})$ and
/// conditional $\mathcal{P}(X \mid \mathbf{Z})$ distribution estimation trait.
pub trait DistributionEstimation {
    /// Joint distribution associated type.
    type JPD: JointProbabilityDistribution;
    /// Conditional distribution associated type.
    type CPD: ConditionalProbabilityDistribution;

    /// Compute the marginal distribution $\mathcal{P}(X)$.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!() // FIXME:
    /// ```
    ///
    fn marginal(&self, x: &str) -> Self::JPD;

    /// Compute the joint distribution $\mathcal{P}(\mathbf{X})$.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!() // FIXME:
    /// ```
    ///
    fn joint<'a, X>(&self, x: X) -> Self::JPD
    where
        X: IntoIterator<Item = &'a str>;

    /// Compute the conditional distribution $\mathcal{P}(X \mid \mathbf{Z})$.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!() // FIXME:
    /// ```
    ///
    fn conditional<'a, Z>(&self, x: &'a str, z: Z) -> Self::CPD
    where
        Z: IntoIterator<Item = &'a str>;
}

/// Distribution projection trait.
pub trait DistributionProjection {
    /// Projection associated type.
    type Projection;

    /// Projects $\mathcal{P}$ onto $\mathcal{Q}$.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!() // FIXME:
    /// ```
    ///
    fn project_onto(&self, q: &Self::Projection) -> Self::Projection;
}

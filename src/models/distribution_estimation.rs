use super::{ConditionalProbabilityDistribution, JointProbabilityDistribution};

pub trait DistributionEstimation {
    type JPD: JointProbabilityDistribution;

    type CPD: ConditionalProbabilityDistribution;

    fn marginal(&self, x: &str) -> Self::JPD;

    fn joint<'a, X>(&self, x: X) -> Self::JPD
    where
        X: IntoIterator<Item = &'a str>;

    fn conditional<'a, Z>(&self, x: &'a str, z: Z) -> Self::CPD
    where
        Z: IntoIterator<Item = &'a str>;
}

pub trait ParallelDistributionEstimation {
    type JPD: JointProbabilityDistribution;

    type CPD: ConditionalProbabilityDistribution;

    fn par_marginal(&self, x: &str) -> Self::JPD;

    fn par_joint<'a, X>(&self, x: X) -> Self::JPD
    where
        X: IntoIterator<Item = &'a str>;

    fn par_conditional<'a, Z>(&self, x: &'a str, z: Z) -> Self::CPD
    where
        Z: IntoIterator<Item = &'a str>;
}

pub trait DistributionProjection {
    type Projection;

    fn project_onto(&self, q: &Self::Projection) -> Self::Projection;
}

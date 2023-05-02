use std::{collections::BTreeSet, f64::consts::FRAC_1_SQRT_2};

use libm::erfc;

use crate::{
    data::ContinuousDataMatrix,
    discovery::ConditionalIndependenceTest,
    prelude::DataSet,
    stats::{CovarianceMatrix, PartialCorrelation},
};

/// Fisher's Z conditional independence test.
#[derive(Clone, Debug)]
pub struct FisherZ<'a> {
    rho: PartialCorrelation,
    alpha: f64,
    n: usize,
    labels: &'a BTreeSet<String>,
}

impl<'a, 'b: 'a> FisherZ<'a> {
    /// Construct Fisher's Z conditional independence test with $\alpha = 0.05$ .
    #[inline]
    pub fn new(d: &'b ContinuousDataMatrix) -> Self {
        // Compute covariance matrix.
        let sigma = CovarianceMatrix::from(d);
        // Initialize partial correlation functor.
        let rho = PartialCorrelation::from(sigma);

        Self {
            rho,
            alpha: 0.05,
            n: d.values().nrows(),
            labels: d.labels(),
        }
    }
}

impl<'a, 'b: 'a> From<&'b ContinuousDataMatrix> for FisherZ<'a> {
    #[inline]
    fn from(d: &'b ContinuousDataMatrix) -> Self {
        Self::new(d)
    }
}

impl<'a: 'b, 'b> ConditionalIndependenceTest<'b> for FisherZ<'a> {
    #[inline]
    fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64) {
        // Compute degree of freedom.
        let dof = self.n - z.len() - 3;

        // Compute partial correlation.
        let stat = self.rho.call(x, y, z);
        // Compute test statistic from partial correlation as:
        //      sqrt(n - |z| - 3) * (1/2 * ln((1 + rho) / (1 - rho)))
        //      sqrt(n - |z| - 3) * atanh(rho)
        let stat = f64::sqrt(dof as f64) * f64::atanh(stat);

        // Compute p-value as:
        //      |x| > \Phi^-1(1 - \alpha / 2)
        //      \Phi(|x|) > 1 - \alpha / 2
        //      2 * (1 - \Phi(|x|)) < \alpha
        //      2 * (1 - (1 / 2  * (1 + erf(|x| / sqrt(2))))) < \alpha
        //      2 - (1 + erf(|x| / sqrt(2))) < \alpha
        //      1 - erf(|x| / sqrt(2)) < \alpha
        //      erfc(|x| * 1 / sqrt(2)) < \alpha
        let pval = erfc(f64::abs(stat) * FRAC_1_SQRT_2);

        (dof, stat, pval)
    }
    #[inline]

    fn labels(&self) -> &'b BTreeSet<String> {
        self.labels
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute p-value.
        let (_, _, pval) = self.eval(x, y, z);

        pval > self.alpha
    }

    #[inline]
    fn with_significance_level(mut self, alpha: f64) -> Self {
        // Assert alpha in (0, 1).
        assert!((0. ..1.).contains(&alpha));
        // Set significance level.
        self.alpha = alpha;

        self
    }
}

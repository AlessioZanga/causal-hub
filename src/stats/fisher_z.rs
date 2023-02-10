use std::f64::consts::FRAC_1_SQRT_2;

use libm::erfc;

use crate::{
    data::ContinuousDataMatrix,
    discovery::ConditionalIndependenceTest,
    stats::{CovarianceMatrix, PartialCorrelation},
};

/// Fisher's Z conditional independence test.
#[derive(Clone, Debug)]
pub struct FisherZ {
    rho: PartialCorrelation,
    alpha: f64,
    n: usize,
}

impl FisherZ {
    /// Construct Fisher's Z conditional independence test with $\alpha = 0.05$ .
    #[inline]
    pub fn new(d: &ContinuousDataMatrix) -> Self {
        // Compute covariance matrix.
        let sigma = CovarianceMatrix::from(d);
        // Initialize partial correlation functor.
        let rho = PartialCorrelation::from(sigma);

        Self {
            rho,
            alpha: 0.05,
            n: d.nrows(),
        }
    }
}

impl ConditionalIndependenceTest for FisherZ {
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
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute p-value.
        let (_, _, pval) = self.eval(x, y, z);

        pval < self.alpha
    }

    fn with_significance_level(mut self, alpha: f64) -> Self {
        // Set significance level.
        self.alpha = alpha;

        self
    }
}
use std::{
    collections::{btree_set, BTreeSet},
    f64::consts::FRAC_1_SQRT_2,
    iter::Map,
};

use libm::erfc;

use crate::{
    data::GaussianDataMatrix,
    discovery::ConditionalIndependenceTest,
    prelude::DataSet,
    stats::{CovarianceMatrix, PartialCorrelation},
};

/// Fisher's Z conditional independence test.
#[derive(Clone, Debug)]
pub struct FisherZ {
    rho: PartialCorrelation,
    alpha: f64,
    n: usize,
    labels: BTreeSet<String>,
}

impl<'a> FisherZ {
    /// Construct Fisher's Z conditional independence test with $\alpha = 0.05$ .
    #[inline]
    pub fn new(d: &'a GaussianDataMatrix) -> Self {
        // Compute covariance matrix.
        let sigma = CovarianceMatrix::from(d);
        // Initialize partial correlation functor.
        let rho = PartialCorrelation::from(sigma);

        Self {
            rho,
            alpha: 0.05,
            n: d.sample_size(),
            labels: d.labels_iter().map(|x| x.into()).collect(),
        }
    }
}

impl<'a> From<&'a GaussianDataMatrix> for FisherZ {
    #[inline]
    fn from(d: &'a GaussianDataMatrix) -> Self {
        Self::new(d)
    }
}

impl<'a> ConditionalIndependenceTest<'a> for FisherZ {
    type LabelsIter<'b> = Map<btree_set::Iter<'b, String>, fn(&'b String) -> &'b str>;

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

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }
}

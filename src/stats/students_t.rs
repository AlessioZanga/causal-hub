use std::collections::BTreeSet;

use statrs::function::beta::beta_reg;

use crate::{
    data::ContinuousDataMatrix,
    discovery::ConditionalIndependenceTest,
    prelude::DataSet,
    stats::{CovarianceMatrix, PartialCorrelation},
};

/// Students' T conditional independence test.
#[derive(Clone, Debug)]
pub struct StudentsT<'a> {
    rho: PartialCorrelation,
    alpha: f64,
    n: usize,
    labels: &'a BTreeSet<String>,
}

impl<'a, 'b: 'a> StudentsT<'a> {
    /// Construct Students' T conditional independence test with $\alpha = 0.05$ .
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

impl<'a, 'b: 'a> From<&'b ContinuousDataMatrix> for StudentsT<'a> {
    #[inline]
    fn from(d: &'b ContinuousDataMatrix) -> Self {
        Self::new(d)
    }
}

impl<'a: 'b, 'b> ConditionalIndependenceTest<'b> for StudentsT<'a> {
    #[inline]
    fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64) {
        // Compute degree of freedom.
        let dof = self.n - z.len() - 2;

        // Compute partial correlation.
        let stat = self.rho.call(x, y, z);

        // Compute test statistic from partial correlation as:
        //      |sqrt((n - |z| - 2) / (1 - rho^2)) * rho|
        let v = dof as f64;
        let t = f64::abs(f64::sqrt(v / (1. - f64::powi(stat, 2))) * stat);
        // Compute p-value as:
        //      |t| > \Phi^-1(1 - \alpha / 2)
        //      \Phi(|t|) > 1 - \alpha / 2
        //      2 * (1 - \Phi(|t|)) < \alpha
        //      2 * (1 - (1 - 1 / 2 * I_x(a, b))) < \alpha
        //      2 * (1 / 2 * I_x(a, b)) < \alpha
        //      I_x(a, b) < \alpha
        // where:
        //      a = v / 2,
        //      b = 1 / 2,
        //      x = v / (t^2 + v).
        let (a, b, x) = (0.5 * v, 0.5, v / (f64::powi(t, 2) + v));
        let pval = beta_reg(a, b, x);

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

    fn labels(&self) -> &'b BTreeSet<String> {
        self.labels
    }
}

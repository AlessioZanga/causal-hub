use std::{
    collections::{btree_set, BTreeSet},
    iter::Map,
};

use statrs::function::beta::beta_reg;

use super::ConditionalIndependenceTest;
use crate::{
    data::GaussianDataMatrix,
    prelude::DataSet,
    stats::{CovarianceMatrix, PartialCorrelation},
};

#[derive(Clone, Debug)]
pub struct StudentsT {
    rho: PartialCorrelation,
    alpha: f64,
    n: usize,
    labels: BTreeSet<String>,
}

impl StudentsT {
    #[inline]
    pub fn new(d: &GaussianDataMatrix, alpha: f64) -> Self {
        // Compute covariance matrix.
        let sigma = CovarianceMatrix::from(d);
        // Initialize partial correlation functor.
        let rho = PartialCorrelation::from(sigma);

        Self {
            rho,
            alpha,
            n: d.sample_size(),
            labels: d.labels_iter().map(|x| x.into()).collect(),
        }
    }

    #[inline]
    pub fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64) {
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
}

impl ConditionalIndependenceTest for StudentsT {
    type LabelsIter<'a> = Map<btree_set::Iter<'a, String>, fn(&'a String) -> &'a str>;

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.labels.iter().map(|x| x.as_str())
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute p-value.
        let (_, _, pval) = self.eval(x, y, z);

        pval > self.alpha
    }
}

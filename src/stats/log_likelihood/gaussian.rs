use std::f64::consts::PI;

use ndarray::prelude::*;
use ndarray_linalg::least_squares::*;

use super::LogLikelihood;
use crate::{
    data::ContinuousDataMatrix,
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    prelude::DataSet,
};

impl<'a, const PARALLEL: bool> LogLikelihood<'a, ContinuousDataMatrix, PARALLEL> {
    #[inline]
    pub(crate) fn marginal_eval(x: ArrayView1<f64>, n: usize) -> (Array1<f64>, f64) {
        // Compute the mean.
        let mean = x.sum() / n as f64;
        // Compute residuals.
        let residuals = &x - mean;
        // Compute the standard deviation.
        let std = residuals.std(1.);

        (residuals, std)
    }

    /// Computes marginal log-likelihood given data set $\mathbf{D}$ and vertex $X$.
    #[inline]
    pub fn marginal(&self, x: usize) -> f64 {
        // Get the variable and sample size.
        let (x, n) = (self.d.values().column(x), self.d.values().nrows());

        // Compute residuals and standard deviation. TODO: Parallelize over mean and variance.
        let (residuals, std) = Self::marginal_eval(x, n);

        // Compute the (marginal) log-likelihood. TODO: Parallelize over log-likelihood.
        (residuals / std)
            // Compute log(norm(mean, std).pdf(x)).
            .mapv(|x| -(f64::ln(f64::sqrt(2. * PI)) + 0.5 * x * x + f64::ln(std)))
            // Sum each term.
            .sum()
    }

    #[inline]
    pub(crate) fn conditional_eval(
        x: ArrayView1<f64>,
        z: ArrayView2<f64>,
        n: usize,
        p: usize,
    ) -> (Array1<f64>, f64) {
        // Get OLS result.
        let ols = z
            // Perform OLS.
            .least_squares(&x)
            // Check OLS status.
            .expect("Failed to perform OLS");

        // Get fitted parameters and residuals sum of squared.
        let (beta, rss) = (
            ols.solution,
            ols.residual_sum_of_squares
                .expect("Failed to compute the residuals sum of squares")
                .sum(),
        );

        // Compute fitted values.
        let residuals = &x - (&z * &beta).sum_axis(Axis(1));
        // Compute standard deviation.
        let std = f64::sqrt(rss / (n - p) as f64);

        (residuals, std)
    }

    /// Computes conditional log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn conditional(&self, x: usize, z: &[usize]) -> f64 {
        // Get reference to underling values.
        let d = self.d.values();
        // Get sample size and number of conditioning variables.
        let (n, m) = (d.nrows(), z.len());
        // Get a copy of the variable.
        let x = d.column(x);
        // Allocate a new contiguous matrix. TODO: Avoid initialization (?).
        let mut z_ = Array2::ones((n, m + 1));
        // Fill with observed variables, skipping the intercept.
        for (i, &z) in z.iter().enumerate() {
            // Copy data from column to column.
            d.column(z).assign_to(z_.column_mut(i + 1));
        }

        // Compute residuals and standard deviation.
        let (residuals, std) = Self::conditional_eval(x, z_.view(), n, m + 1);

        // Compute the (conditional) log-likelihood. TODO: Parallelize over log-likelihood.
        (residuals / std)
            // Compute log(norm(mean, std).pdf(x)).
            .mapv(|x| -(f64::ln(f64::sqrt(2. * PI)) + 0.5 * x * x + f64::ln(std)))
            // Sum each term.
            .sum()
    }
}

impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for LogLikelihood<'a, ContinuousDataMatrix, PARALLEL>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        match z.is_empty() {
            true => self.marginal(x),
            false => self.conditional(x, z),
        }
    }
}

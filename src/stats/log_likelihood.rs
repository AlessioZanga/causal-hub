use std::f64::consts::PI;

use ndarray::prelude::*;
use ndarray_linalg::least_squares::*;
use rayon::prelude::*;

use crate::{
    data::{ConditionalCountMatrix, ContinuousDataMatrix, DiscreteDataMatrix, MarginalCountMatrix},
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
    prelude::DataSet,
    utils::{axis_chunks_size, nan_to_zero},
};

/// Marginal Log-Likelihood functor.
#[derive(Clone, Debug)]
pub struct MarginalLogLikelihood<'a, D> {
    pub(crate) data_set: &'a D,
}

impl<'a, D> MarginalLogLikelihood<'a, D> {
    /// Constructor for LL functor.
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

/* Discrete LL */

impl<'a> MarginalLogLikelihood<'a, DiscreteDataMatrix> {
    /// Computes marginal log-likelihood given data_set set $\mathbf{D}$ and vertex $X$.
    #[inline]
    pub fn call(&self, x: usize) -> f64 {
        // Compute marginal contingency table.
        let n_i = MarginalCountMatrix::new(self.data_set, x);

        // Get the underlying view.
        let n_i = n_i.values();

        // Sum over states and cast to floating point.
        let n = n_i.sum() as f64;
        let n_i = n_i.mapv(|i| i as f64);

        // Compute log-likelihood as n_i * ln(n_i  / n).
        (&n_i * (&n_i / n).mapv(f64::ln))
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            // Sum each term.
            .sum()
    }
}

/* Gaussian LL */

impl<'a> MarginalLogLikelihood<'a, ContinuousDataMatrix> {
    /// Computes marginal log-likelihood given data_set set $\mathbf{D}$ and vertex $X$.
    #[inline]
    pub fn call(&self, x: usize) -> f64 {
        // Get the variable and sample size.
        let (x, n) = (
            self.data_set.values().column(x),
            self.data_set.sample_size(),
        );

        // Compute the mean.
        let mean = x.sum() / n as f64;
        // Compute residuals.
        let residuals = &x - mean;
        // Compute the standard deviation.
        let std = residuals.std(1.);

        // Compute the (marginal) log-likelihood. TODO: Parallelize over log-likelihood.
        (residuals / std)
            // Compute log(norm(mean, std).pdf(x)).
            .mapv(|x| -(f64::ln(f64::sqrt(2. * PI)) + 0.5 * x * x + f64::ln(std)))
            // Sum each term.
            .sum()
    }
}

/// Conditional Log-Likelihood functor.
#[derive(Clone, Debug)]
pub struct ConditionalLogLikelihood<'a, D> {
    pub(crate) data_set: &'a D,
}

impl<'a, D> ConditionalLogLikelihood<'a, D> {
    /// Constructor for LL functor.
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

/* Discrete LL */

impl<'a> ConditionalLogLikelihood<'a, DiscreteDataMatrix> {
    /// Computes conditional log-likelihood given data_set set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute marginal contingency table.
        let n_ij = ConditionalCountMatrix::new(self.data_set, x, z);

        // Get the underlying view.
        let n_ij = n_ij.values();

        // Sum over states and cast to floating point.
        let n_j = n_ij
            .sum_axis(Axis(1))
            .insert_axis(Axis(1))
            .mapv(|j| j as f64);
        let n_ij = n_ij.mapv(|i| i as f64);

        // Compute log-likelihood as n_ij * ln(n_ij  / n_i).
        (&n_ij * (&n_ij / n_j).mapv(f64::ln))
            // Map NaNs to zero.
            .mapv(nan_to_zero)
            // Sum each term.
            .sum()
    }

    /// Computes conditional log-likelihood given data_set set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$ in parallel.
    #[inline]
    pub fn par_call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute marginal contingency table.
        let n_ij = ConditionalCountMatrix::new(self.data_set, x, z);

        // Get the underlying view.
        let n_ij = n_ij.values();

        // Iterate over chunks.
        n_ij.axis_chunks_iter(Axis(0), axis_chunks_size(n_ij))
            // Map each chunk and sum over in parallel.
            .into_par_iter()
            .map(|n_ij| {
                // Sum over states and cast to floating point.
                let n_j = n_ij
                    .sum_axis(Axis(1))
                    .insert_axis(Axis(1))
                    .mapv(|j| j as f64);
                let n_ij = n_ij.mapv(|i| i as f64);

                // Compute log-likelihood as n_ij * ln(n_ij  / n_i).
                (&n_ij * (&n_ij / n_j).mapv(f64::ln))
                    // Map NaNs to zero.
                    .mapv(nan_to_zero)
                    // Sum each term.
                    .sum()
            })
            .sum()
    }
}

/* Gaussian LL */

impl<'a> ConditionalLogLikelihood<'a, ContinuousDataMatrix> {
    /// Computes conditional log-likelihood given data_set set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Get reference to underling values.
        let d = self.data_set.values();
        // Get sample size and number of conditioning variables.
        let (n, m) = (d.nrows(), z.len());
        // Get a copy of the variable.
        let x = d.column(x);
        // Allocate a new contiguous matrix. TODO: Avoid initialization (?).
        let mut z_ = Array2::ones((n, m + 1));
        // Fill with observed variables, skipping the intercept.
        for (i, &z) in z.iter().enumerate() {
            // Copy data_set from column to column.
            d.column(z).assign_to(z_.column_mut(i + 1));
        }

        // Get OLS result.
        let ols = z_
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
        let residuals = &x - (&z_ * &beta).sum_axis(Axis(1));
        // Compute standard deviation.
        let std = f64::sqrt(rss / (n - (m + 1)) as f64);

        // Compute the (conditional) log-likelihood. TODO: Parallelize over log-likelihood.
        (residuals / std)
            // Compute log(norm(mean, std).pdf(x)).
            .mapv(|x| -(f64::ln(f64::sqrt(2. * PI)) + 0.5 * x * x + f64::ln(std)))
            // Sum each term.
            .sum()
    }
}

/// Log-Likelihood (LL) functor.
#[derive(Clone, Debug)]
pub struct LogLikelihood<'a, D> {
    pub(crate) data_set: &'a D,
}

impl<'a, D> LogLikelihood<'a, D> {
    /// Constructor for LL functor.
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

impl<'a, G> DecomposableScoringCriterion<ContinuousDataMatrix, G>
    for LogLikelihood<'a, ContinuousDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        match z.is_empty() {
            true => MarginalLogLikelihood::new(self.data_set).call(x),
            false => ConditionalLogLikelihood::new(self.data_set).call(x, z),
        }
    }
}

impl<'a, G> DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for LogLikelihood<'a, DiscreteDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        match z.is_empty() {
            true => MarginalLogLikelihood::new(self.data_set).call(x),
            false => ConditionalLogLikelihood::new(self.data_set).call(x, z),
        }
    }
}

/// Alias for the LogLikelihood functor.
pub type LL<'a, D> = LogLikelihood<'a, D>;

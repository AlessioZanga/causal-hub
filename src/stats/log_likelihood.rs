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

/// Log-Likelihood (LL) functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct LogLikelihood<'a, D, const PARALLEL: bool> {
    pub(crate) data: &'a D,
}

impl<'a, D, const PARALLEL: bool> LogLikelihood<'a, D, PARALLEL> {
    /// Constructor for LL functor.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!() // FIXME:
    /// ```
    ///
    #[inline]
    pub const fn new(data: &'a D) -> Self {
        Self { data }
    }
}

/* Implement Discrete LL */

impl<'a, const PARALLEL: bool> LogLikelihood<'a, DiscreteDataMatrix, PARALLEL> {
    /// Computes marginal log-likelihood given data set $\mathbf{D}$ and vertex $X$.
    #[inline]
    pub fn marginal(&self, x: usize) -> f64 {
        // Compute marginal contingency table.
        let n_i = MarginalCountMatrix::new(self.data, x);

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

    #[inline]
    pub(crate) fn conditional_eval(n_ij: ArrayView2<usize>) -> f64 {
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

    /// Computes conditional log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn conditional(&self, x: usize, z: &[usize]) -> f64 {
        // Compute marginal contingency table.
        let n_ij = ConditionalCountMatrix::<PARALLEL>::new(self.data, x, z);

        // Iterate over chunks.
        let n_ij = n_ij
            .values()
            .axis_chunks_iter(Axis(0), axis_chunks_size(n_ij.values()));

        // Check if parallelization is enabled.
        match PARALLEL {
            // Map each chunk and sum over in parallel.
            true => n_ij.into_par_iter().map(Self::conditional_eval).sum(),
            // Map each chunk and sum over.
            false => n_ij.map(Self::conditional_eval).sum(),
        }
    }
}

/* Implement Gaussian LL */

impl<'a, const PARALLEL: bool> LogLikelihood<'a, ContinuousDataMatrix, PARALLEL> {
    /// Computes marginal log-likelihood given data set $\mathbf{D}$ and vertex $X$.
    #[inline]
    pub fn marginal(&self, x: usize) -> f64 {
        // Get the variable and sample size.
        let (x, n) = (self.data.values().column(x), self.data.sample_size());

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

    /// Computes conditional log-likelihood given data set $\mathbf{D}$ and vertex $X$ and parents $\mathbf{Z}$.
    #[inline]
    pub fn conditional(&self, x: usize, z: &[usize]) -> f64 {
        // Get reference to underling values.
        let d = self.data.values();
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

impl<'a, G, const PARALLEL: bool> DecomposableScoringCriterion<DiscreteDataMatrix, G>
    for LogLikelihood<'a, DiscreteDataMatrix, PARALLEL>
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

/// Alias for single-thread LL functor.
pub type LL<'a, D> = LogLikelihood<'a, D, false>;
/// Alias for multi-thread LL functor.
pub type ParallelLL<'a, D> = LogLikelihood<'a, D, true>;

use std::f64::consts::PI;

use argmin::{
    core::{CostFunction, Error, Executor, Gradient},
    solver::{
        linesearch::{condition::ArmijoCondition, BacktrackingLineSearch},
        quasinewton::BFGS,
    },
};
use itertools::izip;
use ndarray::prelude::*;
use ndarray_linalg::least_squares::*;
use rayon::prelude::*;
use statrs::function::gamma::{digamma, ln_gamma as lgamma};

use crate::{
    data::{
        CategoricalDataMatrix, ConditionalCountMatrix, DataSet, GaussianDataMatrix,
        MarginalCountMatrix, ZINBDataMatrix,
    },
    discovery::DecomposableScoringCriterion,
    graphs::{directions, DirectedGraph},
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

/* Categorical LL */

impl<'a> MarginalLogLikelihood<'a, CategoricalDataMatrix> {
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

impl<'a> MarginalLogLikelihood<'a, GaussianDataMatrix> {
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

/* Categorical LL */

impl<'a> ConditionalLogLikelihood<'a, CategoricalDataMatrix> {
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

impl<'a> ConditionalLogLikelihood<'a, GaussianDataMatrix> {
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

impl<'a, G> DecomposableScoringCriterion<CategoricalDataMatrix, G>
    for LogLikelihood<'a, CategoricalDataMatrix>
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

impl<'a, G> DecomposableScoringCriterion<GaussianDataMatrix, G>
    for LogLikelihood<'a, GaussianDataMatrix>
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

/* Implement LogLikelihood for multivariate ZINB distribution. */

/// Define the multivariate ZINB objective function.
struct ZINBObjective<'a> {
    /// The response vector.
    x: ArrayView1<'a, f64>,
    /// The design matrix.
    z: ArrayView2<'a, f64>,
}

/// Implement the `CostFunction` trait for `ZINBObjective`.
impl<'a> CostFunction for ZINBObjective<'a> {
    type Param = Array1<f64>;
    type Output = f64;

    fn cost(&self, theta: &Self::Param) -> Result<Self::Output, Error> {
        // [Z; (n, z)]
        let (n, z) = self.z.dim();
        // [\theta; 2z + 3] = [[alpha; z], [delta; 1], [beta; z], [gamma; 1], [lambda; 1]]
        let (alpha_delta, beta_gamma, lambda) = (
            theta.slice(s![..(z + 1)]),
            theta.slice(s![(z + 1)..(2 * z + 2)]),
            theta[2 * z + 2],
        );

        // Z1 = [Z; (n, 1)]
        let mut z1 = Array2::ones((n, z + 1));
        z1.slice_mut(s![.., ..z]).assign(&self.z);

        // logit(p) = Z * alpha + delta
        let logit_p = z1.dot(&alpha_delta);
        // log(p / (1 - p)) = logit(p)
        let exp_logit_p = logit_p.mapv(f64::exp);
        // p = exp(logit(p)) / (1 + exp(logit(p)))
        let p = &exp_logit_p / (1.0 + &exp_logit_p);
        // Fill the infinite values with 1.0.
        let p = p.mapv(|x| if x.is_finite() { x } else { 1.0 });

        // logit(q) = Z * beta + gamma
        let logit_q = z1.dot(&beta_gamma);
        // log(q / (1 - q)) = logit(q)
        let exp_logit_q = logit_q.mapv(f64::exp);
        // q = exp(logit(q)) / (1 + exp(logit(q)))
        let q = &exp_logit_q / (1.0 + &exp_logit_q);
        // Fill the infinite values with 1.0.
        let q = q.mapv(|x| if x.is_finite() { x } else { 1.0 });

        // k = exp(lambda)
        let k = f64::exp(lambda);

        // Compute the log-likelihood.
        // \sum_{i \in x0} log(pi_i + (1 - pi_i) * (1 - q_i)^k)
        // \sum_{i \in x1} log(1 - pi_i) + log_ascfacto(k, x_i) - lgamma(x_i + 1) + x_i * log(q_i) + k * log(1 - q_i)
        let log_likelihood: f64 = izip!(p, q, &self.x)
            .map(|(p_i, q_i, &x_i)| {
                if x_i < f32::EPSILON as f64 {
                    f64::ln(p_i + (1.0 - p_i) * f64::powf(1.0 - q_i, k))
                } else {
                    f64::ln_1p(-p_i)
                        + lgamma(k + x_i) - lgamma(k) // Logarithm of the ascending factorial.
                        - lgamma(x_i + 1.0)
                        + x_i * f64::ln(q_i)
                        + k * f64::ln_1p(-q_i)
                }
            })
            .sum();

        // Clamp the log-likelihood to prevent overflow.
        let log_likelihood = f64::clamp(log_likelihood, f64::MIN, f64::MAX);
        // Negate the log-likelihood since we are minimizing.
        let log_likelihood = -log_likelihood;

        Ok(log_likelihood)
    }
}

/// Implement the `Gradient` trait for `ZINBObjective`.
impl<'a> Gradient for ZINBObjective<'a> {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;

    fn gradient(&self, theta: &Self::Param) -> Result<Self::Gradient, Error> {
        // [Z; (n, z)]
        let (n, z) = self.z.dim();

        // [\theta; 2z + 3] = [[alpha; z], [delta; 1], [beta; z], [gamma; 1], [lambda; 1]]
        let (alpha_delta, beta_gamma, lambda) = (
            theta.slice(s![..(z + 1)]),
            theta.slice(s![(z + 1)..(2 * z + 2)]),
            theta[2 * z + 2],
        );

        // Z1 = [Z; (n, 1)]
        let mut z1 = Array2::ones((n, z + 1));
        z1.slice_mut(s![.., ..z]).assign(&self.z);

        // logit(p) = Z * alpha + delta
        let logit_p = z1.dot(&alpha_delta);
        // log(p / (1 - p)) = logit(p)
        let exp_logit_p = logit_p.mapv(f64::exp);
        // p = exp(logit(p)) / (1 + exp(logit(p)))
        let p = &exp_logit_p / (1.0 + &exp_logit_p);
        // Fill the infinite values with 1.0.
        let p = p.mapv(|x| if x.is_finite() { x } else { 1.0 });

        // logit(q) = Z * beta + gamma
        let logit_q = z1.dot(&beta_gamma);
        // log(q / (1 - q)) = logit(q)
        let exp_logit_q = logit_q.mapv(f64::exp);
        // q = exp(logit(q)) / (1 + exp(logit(q)))
        let q = &exp_logit_q / (1.0 + &exp_logit_q);
        // Fill the infinite values with 1.0.
        let q = q.mapv(|x| if x.is_finite() { x } else { 1.0 });

        // k = exp(lambda)
        let k = f64::exp(lambda);

        // Initialize the gradient.
        let mut gradient = Array1::<f64>::zeros(2 * z + 3);

        // Compute the gradient.
        let (alpha_delta, beta_gamma, lambda) = izip!(p, q, &self.x, z1.rows())
            .map(|(p_i, q_i, &x_i, z1_i)| {
                if x_i < f32::EPSILON as f64 {
                    // d_i = p_i + (1 - p_i) * pow(1 - q_i, k)
                    let d_i = p_i + (1.0 - p_i) * f64::powf(1.0 - q_i, k);

                    (
                        // gradient[0..(z + 1)] = Z1i * ((1 - p_i) * (p_i - p_i * pow(1 - q_i, k)) / d_i)
                        &z1_i * ((1.0 - p_i) * (p_i - p_i * f64::powf(1.0 - q_i, k)) / d_i),
                        // gradient[(p + 1)..(2 * p + 2)] = -Z1i * ((1 - p_i) * (k * pow(1 - q_i, k - 1)) * q_i * (1 - q_i) / d_i)
                        -&z1_i
                            * ((1.0 - p_i)
                                * (k * f64::powf(1.0 - q_i, k - 1.0))
                                * q_i
                                * (1.0 - q_i)
                                / d_i),
                        // gradient[2 * p + 2] = (1 - p_i) * pow(1 - q_i, k) * log(1 - q_i) / d_i) * k
                        ((1.0 - p_i) * f64::powf(1.0 - q_i, k) * f64::ln_1p(-q_i) / d_i) * k,
                    )
                } else {
                    (
                        // gradient[0..(z + 1)] = -Z1i * p_i
                        -&z1_i * p_i,
                        // gradient[(p + 1)..(2 * p + 2)] = -Z1i * ((k + y_i) * q_i - y_i)
                        -&z1_i * ((k + x_i) * q_i - x_i),
                        // gradient[2 * p + 2] = digamma(k + y_i) - digamma(k) + log(1 - q_i) * k
                        (digamma(k + x_i) - digamma(k) + f64::ln_1p(-q_i)) * k,
                    )
                }
            })
            .fold(
                (Array1::<f64>::zeros(z + 1), Array1::<f64>::zeros(z + 1), 0.),
                |(alpha_delta, beta_gamma, lambda), (alpha_delta_i, beta_gamma_i, lambda_i)| {
                    (
                        alpha_delta + alpha_delta_i,
                        beta_gamma + beta_gamma_i,
                        lambda + lambda_i,
                    )
                },
            );

        // Fill the gradient.
        gradient.slice_mut(s![..(z + 1)]).assign(&alpha_delta);
        gradient
            .slice_mut(s![(z + 1)..(2 * z + 2)])
            .assign(&beta_gamma);
        gradient[2 * z + 2] = lambda;

        // Negate the gradient since we are minimizing.
        let gradient = -gradient;

        Ok(gradient)
    }
}

impl<'a, G> DecomposableScoringCriterion<ZINBDataMatrix, G> for LogLikelihood<'a, ZINBDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Get reference to underling values.
        let d = self.data_set.values();
        // Get sample size and number of conditioning variables.
        let (n, m) = (d.nrows(), z.len());
        // Get a copy of the variable.
        let x = d.column(x);
        // Allocate a new contiguous matrix.
        let mut z_ = Array2::<f64>::zeros((n, m));
        // Fill with observed variables.
        for (i, &z) in z.iter().enumerate() {
            // Copy data_set from column to column.
            d.column(z).assign_to(z_.column_mut(i));
        }
        // Get a view of the design matrix.
        let z = z_.view();

        // Initialize the starting parameters.
        let theta_0 = Array1::zeros(2 * z.len() + 3);
        // Initialize the inverse Hessian matrix.
        let inv_hessian = f32::EPSILON as f64 * Array2::eye(theta_0.len());

        // Initialize the objective function.
        let objective = ZINBObjective { x, z };

        // Initialize the solver.
        let step = ArmijoCondition::new(f32::EPSILON as f64).unwrap();
        let search = BacktrackingLineSearch::new(step);
        let solver = BFGS::new(search);

        // Run the solver.
        let results = Executor::new(objective, solver)
            .configure(|s| s.inv_hessian(inv_hessian).param(theta_0).max_iters(500))
            .run()
            .expect("Failed to run the solver");

        // Get the negated log-likelihood.
        -results.state.best_cost
    }
}

/// Alias for the LogLikelihood functor.
pub type LL<'a, D> = LogLikelihood<'a, D>;

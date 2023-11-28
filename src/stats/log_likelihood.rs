use std::f64::consts::PI;

use argmin::{
    core::{CostFunction, Error, Executor, Gradient},
    solver::{
        linesearch::{condition::ArmijoCondition, BacktrackingLineSearch},
        quasinewton::BFGS,
    },
};
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

const E: f64 = f32::EPSILON as f64;

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
        let (x, n) = (self.data_set.data().column(x), self.data_set.sample_size());

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
        let d = self.data_set.data();
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
#[derive(Clone, Debug)]
struct ZINBObjective {
    /// The response vector.
    x1: Array2<f64>,
    // lgamma(x1 + 1).
    x1_1_lgamma: Array2<f64>,
    /// The design matrix.
    z10: Array2<f64>,
    z11: Array2<f64>,
}

impl ZINBObjective {
    /// Constructor for ZINBObjective.
    #[inline]
    fn new(d: &Array2<f64>, x: usize, z: &[usize]) -> Self {
        // Get sample size and number of conditioning variables.
        let (n, m) = (d.nrows(), z.len());

        // Get a copy of the variable.
        let x = d.column(x);
        // Partition the indices vector.
        let (i0, i1): (Vec<_>, Vec<_>) = (0..n).partition(|&i| x[i] < E);
        // Partition the response vector.
        let mut x1 = Array1::zeros(i1.len());
        i1.iter().enumerate().for_each(|(i, &j)| x1[i] = x[j]);
        let x1 = x1.insert_axis(Axis(1));

        // Pre-compute lgamma(x1 + 1).
        let x1_1_lgamma = (&x1 + 1.).mapv(lgamma);

        // Allocate a new contiguous matrix.
        let mut z1 = Array2::<f64>::ones((n, m + 1));
        // Fill with observed variables.
        z.iter().enumerate().for_each(|(i, &j)| {
            z1.column_mut(i).assign(&d.column(j));
        });

        // Partition the design matrix.
        let (mut z10, mut z11) = (
            Array2::zeros((i0.len(), z1.ncols())),
            Array2::zeros((i1.len(), z1.ncols())),
        );
        i0.iter()
            .enumerate()
            .for_each(|(i, &j)| z10.row_mut(i).assign(&z1.row(j)));
        i1.iter()
            .enumerate()
            .for_each(|(i, &j)| z11.row_mut(i).assign(&z1.row(j)));

        Self {
            x1,
            x1_1_lgamma,
            z10,
            z11,
        }
    }

    fn eval(z: &Array2<f64>, theta: ArrayView1<f64>) -> Array2<f64> {
        // logit(p) = Z * alpha + delta
        let logit = z.dot(&theta).insert_axis(Axis(1));
        // log(p / (1 - p)) = logit(p)
        let exp_logit = logit.mapv(f64::exp);
        // p = exp(logit(p)) / (1 + exp(logit(p)))
        let p = &exp_logit / (1. + &exp_logit);
        // Fill the infinite values with 1..
        let p = p.mapv(|x| if x.is_finite() { x } else { 1. });
        // Avoid NaNs by mapping to [E, 1 - E].
        (p - E).mapv(f64::abs)
    }
}

/// Implement the `CostFunction` trait for `ZINBObjective`.
impl CostFunction for ZINBObjective {
    type Param = Array1<f64>;
    type Output = f64;

    fn cost(&self, theta: &Self::Param) -> Result<Self::Output, Error> {
        // [Z1; (n, z + 1)]
        let z1 = self.z11.ncols();

        // Assert parameters are valid.
        assert!(
            theta.iter().all(|&i| i.is_finite()),
            "Invalid parameters: {theta}",
        );

        // [\theta; 2(z + 1) + 1] = [[alpha; z], [delta; 1], [beta; z], [gamma; 1], [lambda; 1]]
        let (alpha_delta, beta_gamma, lambda) = (
            theta.slice(s![..z1]),
            theta.slice(s![z1..(2 * z1)]),
            theta[2 * z1],
        );

        // logit(p) = Z * alpha + delta
        let p0 = Self::eval(&self.z10, alpha_delta);
        let p1 = Self::eval(&self.z11, alpha_delta);
        // logit(q) = Z * beta + gamma
        let q0 = Self::eval(&self.z10, beta_gamma);
        let q1 = Self::eval(&self.z11, beta_gamma);
        // k = exp(lambda), clamped to avoid overflow.
        let k = f64::exp(f64::min(lambda, 1e2));

        // Logarithm of the ascending factorial function.
        let log_ascfacto = |k: f64, x: &Array2<f64>| -> Array2<f64> {
            x.mapv(|i| (0..(i as usize)).map(|j| f64::ln(k + j as f64)).sum())
        };

        // Compute the log-likelihood.
        let log_likelihood =
            // \sum_{i \in x0} log(pi_i + (1 - pi_i) * (1 - q_i)^k)
            (&p0 + (1. - &p0) * (1. - &q0).mapv(|i| f64::powf(i, k)) + E)
                .mapv(f64::ln)
                .sum()
            // \sum_{i \in x1} log(1 - pi_i) + log_ascfacto(k, x_i) - lgamma(x_i + 1) + x_i * log(q_i) + k * log(1 - q_i)
            + ((1. - &p1).mapv(f64::ln)
                + log_ascfacto(k, &self.x1)
                - &self.x1_1_lgamma
                + &self.x1 * &q1.mapv(f64::ln)
                + k * (1. - &q1).mapv(f64::ln)
            ).sum();

        // Negate the log-likelihood since we are minimizing.
        let log_likelihood = -log_likelihood;

        // Clip the log-likelihood to avoid overflow.
        let log_likelihood = f64::clamp(log_likelihood, E * f64::MIN, E * f64::MAX);
        // Assert log-likelihood is valid.
        assert!(
            log_likelihood.is_finite(),
            "Invalid log-likelihood: {log_likelihood}, with parameters: {theta}",
        );

        Ok(log_likelihood)
    }
}

/// Implement the `Gradient` trait for `ZINBObjective`.
impl Gradient for ZINBObjective {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;

    fn gradient(&self, theta: &Self::Param) -> Result<Self::Gradient, Error> {
        // [Z; (n, z + 1)]
        let z1 = self.z11.ncols();

        // Assert parameters are valid.
        assert!(
            theta.iter().all(|&i| i.is_finite()),
            "Invalid parameters: {theta}",
        );

        // [\theta; 2(z + 1) + 1] = [[alpha; z], [delta; 1], [beta; z], [gamma; 1], [lambda; 1]]
        let (alpha_delta, beta_gamma, lambda) = (
            theta.slice(s![..z1]),
            theta.slice(s![z1..(2 * z1)]),
            theta[2 * z1],
        );

        // logit(p) = Z * alpha + delta
        let p0 = Self::eval(&self.z10, alpha_delta);
        let p1 = Self::eval(&self.z11, alpha_delta);
        // logit(q) = Z * beta + gamma
        let q0 = Self::eval(&self.z10, beta_gamma);
        let q1 = Self::eval(&self.z11, beta_gamma);
        // k = exp(lambda), clamped to avoid overflow.
        let k = f64::exp(f64::min(lambda, 1e2));

        // Initialize the gradient.
        let mut gradient = Array1::<f64>::zeros(2 * z1 + 1);

        // Pre-compute the following terms.
        let _q0 = -&q0; // -q0
        let _q1 = -&q1; // -q1
        let _k_1 = k - 1.; // k - 1
        let _1_p0 = 1. - &p0; // (1 - p0)
        let _1_q0 = 1. - &q0; // (1 - q0)
        let _1_q1 = 1. - &q1; // (1 - q1)
        let _1_q0_k = _1_q0.mapv(|i| f64::powf(i, k)); // (1 - q0)^k
        let d0 = &p0 + &_1_p0 * &_1_q0_k; // p0 + (1 - p0) * pow(1 - q0, k)
        let _1_p0_d0 = &_1_p0 / &d0; // (1 - p0) / d0

        // alpha_delta
        gradient.slice_mut(s![..z1]).assign(&{
            // Z10 * ((1 - p0) * p0 * (1 - pow(1 - q0, k)) / d0)
            (&self.z10 * &_1_p0_d0 * &p0 * (1. - &_1_q0_k)).sum_axis(Axis(0))
            // -Z11 * p1
            -(&self.z11 * &p1).sum_axis(Axis(0))
        });

        // beta_gamma
        gradient.slice_mut(s![z1..(2 * z1)]).assign(&{
            // -Z10 * ((1 - p0) * (k * pow(1 - q0, k - 1)) * q0 * (1 - q0) / d0) -> Z10 * ((1 - p0) * (k * pow(1 - q0, k - 1)) * (-q0) * (1 - q0) / d0)
            (&self.z10 * (&_1_p0_d0 * k * &_1_q0.mapv(|i| f64::powf(i, _k_1)) * &_q0 * &_1_q0)).sum_axis(Axis(0))
            // -Z11 * ((k + x1) * q1 - x1) -> Z11 * ((k + x1) * (-q1) + x1)
            + (&self.z11 * ((k + &self.x1) * &_q1 + &self.x1)).sum_axis(Axis(0))
        });

        // lambda
        gradient[2 * z1] = (
            // (1 - p0) * pow(1 - q0, k) * log(1 - q0) / d0)
            (&_1_p0_d0 * &_1_q0_k * &_1_q0.mapv(f64::ln)).sum()
            // digamma(k + x1) - digamma(k) + log(1 - q1)
            + ((&self.x1 + k).mapv(digamma) - digamma(k) + &_1_q1.mapv(f64::ln)).sum()
            // * k
        ) * k;

        // Negate the gradient since we are minimizing.
        let gradient = -gradient;

        // Clip the gradient to avoid overflow.
        let gradient = gradient.mapv_into(|i| f64::clamp(i, E * f64::MIN, E * f64::MAX));
        // Assert gradient is valid.
        assert!(
            gradient.iter().all(|&i| i.is_finite()),
            "Invalid gradient: {gradient}, with parameters: {theta}",
        );

        Ok(gradient)
    }
}

impl<'a, G> DecomposableScoringCriterion<ZINBDataMatrix, G> for LogLikelihood<'a, ZINBDataMatrix>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Initialize the objective function.
        let f = ZINBObjective::new(self.data_set.data(), x, z);

        // Initialize the starting parameters.
        let theta_0 = Array1::zeros(2 * (z.len() + 1) + 1);
        // Initialize the inverse Hessian using the initial gradient as in:
        // "Numerical Optimization, p. 142. Second Edition. Nocedal & Wright."
        let l_0 = E * f.gradient(&theta_0).unwrap().mapv(f64::abs).sum().recip();
        let h_0 = l_0 * Array2::eye(theta_0.len());

        // Initialize the solver.
        let step = ArmijoCondition::new(f64::sqrt(E)).expect("Failed to initialize the step");
        let search = BacktrackingLineSearch::new(step);
        let solver = BFGS::new(search)
            .with_tolerance_cost(1e-10)
            .expect("Failed to initialize the solver");
        // Run the solver.
        let results = Executor::new(f, solver)
            .configure(|s| s.param(theta_0).inv_hessian(h_0).max_iters(500))
            .timer(false)
            .run()
            .expect("Failed to run the solver");

        // Get the negated log-likelihood.
        -results.state.best_cost
    }
}

/// Alias for the LogLikelihood functor.
pub type LL<'a, D> = LogLikelihood<'a, D>;

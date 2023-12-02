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
    graphs::{Directed, DirectedGraph},
    utils::{axis_chunks_size, nan_to_zero},
};

const E: f64 = f32::EPSILON as f64;

#[derive(Clone, Debug)]
pub struct MarginalLogLikelihood<'a, D> {
    pub(crate) data_set: &'a D,
}

impl<'a, D> MarginalLogLikelihood<'a, D> {
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

/* Categorical LL */

impl<'a> MarginalLogLikelihood<'a, CategoricalDataMatrix> {
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

#[derive(Clone, Debug)]
pub struct ConditionalLogLikelihood<'a, D> {
    pub(crate) data_set: &'a D,
}

impl<'a, D> ConditionalLogLikelihood<'a, D> {
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

/* Categorical LL */

impl<'a> ConditionalLogLikelihood<'a, CategoricalDataMatrix> {
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

#[derive(Clone, Debug)]
pub struct LogLikelihood<'a, D> {
    pub(crate) data_set: &'a D,
}

impl<'a, D> LogLikelihood<'a, D> {
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

impl<'a, G> DecomposableScoringCriterion<CategoricalDataMatrix, G>
    for LogLikelihood<'a, CategoricalDataMatrix>
where
    G: DirectedGraph<Direction = Directed>,
{
    type LabelsIter<'b> = <CategoricalDataMatrix as DataSet>::LabelsIter<'b> where Self: 'b;

    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        if z.is_empty() {
            MarginalLogLikelihood::new(self.data_set).call(x)
        } else {
            ConditionalLogLikelihood::new(self.data_set).call(x, z)
        }
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.data_set.labels_iter()
    }
}

impl<'a, G> DecomposableScoringCriterion<GaussianDataMatrix, G>
    for LogLikelihood<'a, GaussianDataMatrix>
where
    G: DirectedGraph<Direction = Directed>,
{
    type LabelsIter<'b> = <GaussianDataMatrix as DataSet>::LabelsIter<'b> where Self: 'b;

    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        if z.is_empty() {
            MarginalLogLikelihood::new(self.data_set).call(x)
        } else {
            ConditionalLogLikelihood::new(self.data_set).call(x, z)
        }
    }

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.data_set.labels_iter()
    }
}

/* Implement LogLikelihood for multivariate ZINB distribution. */

#[derive(Clone, Debug)]
struct ZINBObjective {
    x1: Array2<f64>,
    // lgamma(x1 + 1).
    x1_1_lgamma: Array2<f64>,

    z10: Array2<f64>,
    z11: Array2<f64>,
}

impl ZINBObjective {
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
        let pi_0 = Self::eval(&self.z10, alpha_delta);
        let pi_1 = Self::eval(&self.z11, alpha_delta);
        // logit(q) = Z * beta + gamma
        let p_0 = Self::eval(&self.z10, beta_gamma);
        let p_1 = Self::eval(&self.z11, beta_gamma);
        // r = exp(lambda), clamped to avoid overflow.
        let r = f64::exp(f64::min(lambda, 1e2));

        // Logarithm of the ascending factorial function.
        let log_ascfacto = |r: f64, x: &Array2<f64>| -> Array2<f64> {
            x.mapv(|i| (0..(i as usize)).map(|j| f64::ln(r + j as f64)).sum())
        };

        // Compute the log-likelihood.
        let log_likelihood =
            // \sum_{i \in x0} log(pi_i + (1 - pi_i) * (1 - q_i)^r)
            (&pi_0 + (1. - &pi_0) * (1. - &p_0).mapv(|i| f64::powf(i, r)) + E)
                .mapv(f64::ln)
                .sum()
            // \sum_{i \in x1} log(1 - pi_i) + log_ascfacto(r, x_i) - lgamma(x_i + 1) + x_i * log(q_i) + r * log(1 - q_i)
            + ((1. - &pi_1).mapv(f64::ln)
                + log_ascfacto(r, &self.x1)
                - &self.x1_1_lgamma
                + &self.x1 * &p_1.mapv(f64::ln)
                + r * (1. - &p_1).mapv(f64::ln)
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
        let pi_0 = Self::eval(&self.z10, alpha_delta);
        let pi_1 = Self::eval(&self.z11, alpha_delta);
        // logit(q) = Z * beta + gamma
        let p_0 = Self::eval(&self.z10, beta_gamma);
        let p_1 = Self::eval(&self.z11, beta_gamma);
        // r = exp(lambda), clamped to avoid overflow.
        let r = f64::exp(f64::min(lambda, 1e2));

        // Initialize the gradient.
        let mut gradient = Array1::<f64>::zeros(2 * z1 + 1);

        // Pre-compute the following terms.
        let _p0 = -&p_0; // -p_0
        let _p1 = -&p_1; // -p_1
        let _1_pi0 = 1. - &pi_0; // (1 - pi_0)
        let _1_p0 = 1. - &p_0; // (1 - p_0)
        let _1_p1 = 1. - &p_1; // (1 - p_1)
        let _1_p0_k = _1_p0.mapv(|i| f64::powf(i, r)); // (1 - p_0)^r
        let d0 = &pi_0 + &_1_pi0 * &_1_p0_k; // pi_0 + (1 - pi_0) * pow(1 - p_0, r)
        let _1_pi0_d0 = &_1_pi0 / &d0; // (1 - pi_0) / d0

        // alpha_delta
        gradient.slice_mut(s![..z1]).assign(&{
            // Z10 * ((1 - pi_0) * pi_0 * (1 - pow(1 - p_0, r)) / d0)
            (&self.z10 * &_1_pi0_d0 * &pi_0 * (1. - &_1_p0_k)).sum_axis(Axis(0))
            // -Z11 * pi_1
            -(&self.z11 * &pi_1).sum_axis(Axis(0))
        });

        // beta_gamma
        gradient.slice_mut(s![z1..(2 * z1)]).assign(&{
            // -Z10 * ((1 - pi_0) * (r * pow(1 - p_0, r - 1)) * p_0 * (1 - p_0) / d0) -> Z10 * ((1 - pi_0) * (r * pow(1 - p_0, r - 1)) * (-p_0) * (1 - p_0) / d0)
            (&self.z10 * (&_1_pi0_d0 * r * &_1_p0_k * &_p0)).sum_axis(Axis(0))
            // -Z11 * ((r + x1) * p_1 - x1) -> Z11 * ((r + x1) * (-p_1) + x1)
            + (&self.z11 * ((r + &self.x1) * &_p1 + &self.x1)).sum_axis(Axis(0))
        });

        // lambda
        gradient[2 * z1] = (
            // (1 - pi_0) * pow(1 - p_0, r) * log(1 - p_0) / d0)
            (&_1_pi0_d0 * &_1_p0_k * &_1_p0.mapv(f64::ln)).sum()
            // digamma(r + x1) - digamma(r) + log(1 - p_1)
            + ((&self.x1 + r).mapv(digamma) - digamma(r) + &_1_p1.mapv(f64::ln)).sum()
            // * r
        ) * r;

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
    G: DirectedGraph<Direction = Directed>,
{
    type LabelsIter<'b> = <ZINBDataMatrix as DataSet>::LabelsIter<'b> where Self: 'b;

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

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        self.data_set.labels_iter()
    }
}

pub type LL<'a, D> = LogLikelihood<'a, D>;

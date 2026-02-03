use ndarray::prelude::*;
use ndarray_linalg::{Cholesky, SVD, UPLO};
use ndarray_rand::RandomExt;
use rand::Rng;
use rand_distr::Normal;

use crate::{
    models::{GaussCPD, GaussCPDP},
    random::Random,
    types::{Error, Labels, Result},
};

/// A struct for random Gaussian CPD parameters generation.
struct RngGaussCPDP<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    x: usize,
    z: usize,
    s_a: f64,
    s_b: f64,
    e: f64,
}

impl<'a, R> RngGaussCPDP<'a, R>
where
    R: Rng,
{
    /// Creates a new `RngGaussCPDP` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `x` - The number of target variables.
    /// * `z` - The number of conditioning variables.
    /// * `s_a` - The standard deviation of the regression coefficients.
    /// * `s_b` - The standard deviation of the intercept.
    /// * `e` - A small positive constant for covariance regularization.
    ///
    /// # Errors
    ///
    /// * If `s_a` is not positive.
    /// * If `s_b` is not positive.
    /// * If `e` is not positive.
    ///
    /// # Returns
    ///
    /// A new `RngGaussCPDP` instance.
    ///
    fn new(rng: &'a mut R, x: usize, z: usize, s_a: f64, s_b: f64, e: f64) -> Result<Self> {
        // Check parameters.
        if s_a <= 0.0 {
            return Err(Error::InvalidParameter("s_a", "must be positive"));
        }
        if s_b <= 0.0 {
            return Err(Error::InvalidParameter("s_b", "must be positive"));
        }
        if e <= 0.0 {
            return Err(Error::InvalidParameter("e", "must be positive"));
        }

        Ok(Self {
            rng,
            x,
            z,
            s_a,
            s_b,
            e,
        })
    }
}

impl<R> Random for RngGaussCPDP<'_, R>
where
    R: Rng,
{
    type Output = Result<GaussCPDP>;

    fn random(&mut self) -> Self::Output {
        // 1. Generate coefficient matrix A (x x z)
        let mut a = if self.x > 0 && self.z > 0 {
            let dist_a = Normal::new(0.0, self.s_a)
                .map_err(|e| Error::InvalidParameter("s_a", &e.to_string()))?;
            Array2::random_using((self.x, self.z), dist_a, self.rng)
        } else {
            Array2::zeros((self.x, self.z))
        };

        // 2. Control regression strength: spectral norm ||A||
        if self.x > 0 && self.z > 0 {
            let (_, s, _) = a
                .svd(false, false)
                .map_err(|e| Error::Linalg(&format!("Failed to compute SVD: {e}")))?;
            let spectral_norm = s[0];
            if spectral_norm > 1.0 {
                a /= spectral_norm;
            }
        }

        // 3. Generate intercept vector b (x)
        let b = if self.x > 0 {
            let dist_b = Normal::new(0.0, self.s_b)
                .map_err(|e| Error::InvalidParameter("s_b", &e.to_string()))?;
            Array1::random_using(self.x, dist_b, self.rng)
        } else {
            Array1::zeros(self.x)
        };

        // 4. Generate covariance matrix Sigma (x x x)
        let s = if self.x > 0 {
            let mut s;
            // Create the Normal distribution.
            let dist_m = Normal::new(0.0, 1.0)
                .map_err(|e| Error::InvalidParameter("sigma", &e.to_string()))?;
            loop {
                // Sample random matrix M (x x x)
                let m = Array2::random_using((self.x, self.x), dist_m, self.rng);

                // Compute Sigma = M * M^T
                s = m.dot(&m.t());

                // Regularize: Sigma = Sigma + e * I
                for i in 0..self.x {
                    s[[i, i]] += self.e;
                }

                // 5. Validation step: positive definite
                if s.cholesky(UPLO::Lower).is_ok() {
                    break;
                }
            }
            s
        } else {
            Array2::zeros((self.x, self.x))
        };

        // 6. Construct GaussCPDP
        GaussCPDP::new(a, b, s)
    }
}

/// A struct for random Gaussian CPD generation.
pub struct RngGaussCPD<'a, R>
where
    R: Rng,
{
    rng: &'a mut R,
    labels: &'a Labels,
    conditioning_labels: &'a Labels,
    s_a: f64,
    s_b: f64,
    e: f64,
}

impl<'a, R> RngGaussCPD<'a, R>
where
    R: Rng,
{
    /// Creates a new `RngGaussCPD` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `labels` - The labels of the target variables.
    /// * `conditioning_labels` - The labels of the conditioning variables.
    /// * `s_a` - The standard deviation of the regression coefficients.
    /// * `s_b` - The standard deviation of the intercept.
    /// * `e` - A small positive constant for covariance regularization.
    ///
    /// # Returns
    ///
    /// A new `RngGaussCPD` instance.
    ///
    pub fn new(
        rng: &'a mut R,
        labels: &'a Labels,
        conditioning_labels: &'a Labels,
        s_a: f64,
        s_b: f64,
        e: f64,
    ) -> Result<Self> {
        Ok(Self {
            rng,
            labels,
            conditioning_labels,
            s_a,
            s_b,
            e,
        })
    }
}

impl<R> Random for RngGaussCPD<'_, R>
where
    R: Rng,
{
    type Output = Result<GaussCPD>;

    fn random(&mut self) -> Self::Output {
        // Generate parameters
        let mut rng_params = RngGaussCPDP::new(
            self.rng,
            self.labels.len(),
            self.conditioning_labels.len(),
            self.s_a,
            self.s_b,
            self.e,
        )?;
        let parameters = rng_params.random()?;

        // Construct GaussCPD
        GaussCPD::new(
            self.labels.clone(),
            self.conditioning_labels.clone(),
            parameters,
        )
    }
}

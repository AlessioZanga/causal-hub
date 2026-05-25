use ndarray::{Zip, prelude::*};
use statrs::distribution::{ChiSquared, ContinuousCDF, FisherSnedecor};

use crate::{
    estimators::CPDEstimator,
    models::{CIM, CatCIM, Labelled},
    types::{Error, Labels, Result, Set},
};

/// A trait for conditional independence testing.
pub trait CITest {
    /// Test for conditional independence as X _||_ Y | Z.
    ///
    /// # Arguments
    ///
    /// * `x` - The first variable.
    /// * `y` - The second variable.
    /// * `z` - The conditioning set.
    ///
    /// # Returns
    ///
    /// `true` if X _||_ Y | Z, `false` otherwise.
    ///
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool>;
}

/// A struct representing the Chi-squared test.
pub struct ChiSquaredTest<'a, E> {
    estimator: &'a E,
    alpha: f64,
}

impl<'a, E> ChiSquaredTest<'a, E> {
    /// Creates a new `ChiSquaredTest` instance.
    ///
    /// # Arguments
    ///
    /// * `estimator` - A reference to the estimator.
    /// * `alpha` - The significance level.
    ///
    /// # Returns
    ///
    /// A new `ChiSquaredTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Result<Self> {
        // Check that the significance level is in [0, 1].
        if !(0.0..=1.0).contains(&alpha) {
            return Err(Error::InvalidParameter("alpha", "must be in [0, 1]"));
        }

        Ok(Self { estimator, alpha })
    }
}

impl<'a, E> Labelled for ChiSquaredTest<'a, E>
where
    E: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.estimator.labels()
    }
}

impl<E> CITest for ChiSquaredTest<'_, E>
where
    E: CPDEstimator<CatCIM>,
{
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool> {
        // Check Y contains exactly one label.
        // TODO: Refactor code and remove this assumption.
        if y.len() != 1 {
            return Err(Error::InvalidParameter(
                "y",
                "must contain exactly one label",
            ));
        }

        // Compute the extended separation set.
        let mut s = z.clone();
        // Get the ordered position of Y in the extended separation set.
        let s_y = match z.binary_search(&y[0]) {
            Ok(_) => return Err(Error::SetsNotDisjoint("Y", "Z")),
            Err(i) => i,
        };
        // Insert Y into the extended separation set in sorted order.
        s.shift_insert(s_y, y[0]);

        // Fit the intensity matrices.
        let q_xz = self.estimator.fit(x, z)?;
        let q_xs = self.estimator.fit(x, &s)?;
        // Get the sufficient statistics for the sets.
        let stats_xz = q_xz
            .fitted_statistics()
            .ok_or_else(|| Error::MissingSufficientStatistics())?;
        let n_xz = stats_xz.fitted_conditional_counts();
        let stats_xs = q_xs
            .fitted_statistics()
            .ok_or_else(|| Error::MissingSufficientStatistics())?;
        let n_xs = stats_xs.fitted_conditional_counts();

        // Get the shape of the extended separation set.
        let c_s = q_xs.conditioning_shape();
        // Get the shape of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        for j in 0..n_xs.shape()[0] {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the chi-squared distribution.
            let k_xz = n_xz.index_axis(Axis(0), i);
            let k_xs = n_xs.index_axis(Axis(0), j);
            // Compute the scaling factors.
            let k = &k_xz.sum_axis(Axis(1)) / &k_xs.sum_axis(Axis(1));
            let k = k.sqrt().insert_axis(Axis(1));
            let l = k.recip();
            // Compute the chi-squared statistic for uneven number of samples.
            let chi_sq_num = (&k * &k_xs - &l * &k_xz).powi(2);
            let chi_sq_den = &k_xs + &k_xz;
            let chi_sq = chi_sq_num / &chi_sq_den;
            // Fix division by zero.
            let chi_sq = chi_sq.mapv(|x| if x.is_finite() { x } else { 0. });
            // Compute the chi-squared statistic.
            let chi_sq = chi_sq.sum_axis(Axis(1));
            // For each chi-squared statistic ...
            for (c, d) in chi_sq.into_iter().zip(chi_sq_den.rows()) {
                // Count the non-zero degrees of freedom.
                let dof = d.mapv(|d| (d > 0.) as usize).sum();
                // Check if the degrees of freedom is at least 2.
                let dof = if dof >= 2 { dof } else { 2 };
                // Initialize the chi-squared distribution.
                let n = ChiSquared::new((dof - 1) as f64)
                    .map_err(|e| Error::Probability(&e.to_string()))?;
                // Compute the p-value.
                let p_value = n.cdf(c);
                // Check if the p-value is in the alpha range.
                if p_value >= (1. - self.alpha) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

/// A struct representing the F test.
pub struct FTest<'a, E> {
    estimator: &'a E,
    alpha: f64,
}

impl<'a, E> FTest<'a, E> {
    /// Creates a new `FTest` instance.
    ///
    /// # Arguments
    ///
    /// * `estimator` - A reference to the estimator.
    /// * `alpha` - The significance level.
    ///
    /// # Returns
    ///
    /// A new `FTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Result<Self> {
        // Check that the significance level is in [0, 1].
        if !(0.0..=1.0).contains(&alpha) {
            return Err(Error::InvalidParameter("alpha", "must be in [0, 1]"));
        }

        Ok(Self { estimator, alpha })
    }
}

impl<E> Labelled for FTest<'_, E>
where
    E: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.estimator.labels()
    }
}

impl<E> CITest for FTest<'_, E>
where
    E: CPDEstimator<CatCIM>,
{
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool> {
        // Check Y contains exactly one label.
        // TODO: Refactor code and remove this assumption.
        if y.len() != 1 {
            return Err(Error::InvalidParameter(
                "y",
                "must contain exactly one label",
            ));
        }

        // Compute the alpha range.
        let alpha = (self.alpha / 2.)..=(1. - self.alpha / 2.);

        // Compute the extended separation set.
        let mut s = z.clone();
        // Get the ordered position of Y in the extended separation set.
        let s_y = match z.binary_search(&y[0]) {
            Ok(_) => return Err(Error::SetsNotDisjoint("Y", "Z")),
            Err(i) => i,
        };
        // Insert Y into the extended separation set in sorted order.
        s.shift_insert(s_y, y[0]);

        // Fit the intensity matrices.
        let q_xz = self.estimator.fit(x, z)?;
        let q_xs = self.estimator.fit(x, &s)?;
        // Get the sufficient statistics for the sets.
        let stats_xz = q_xz
            .fitted_statistics()
            .ok_or_else(|| Error::MissingSufficientStatistics())?;
        let n_xz = stats_xz.fitted_conditional_counts();
        let stats_xs = q_xs
            .fitted_statistics()
            .ok_or_else(|| Error::MissingSufficientStatistics())?;
        let n_xs = stats_xs.fitted_conditional_counts();

        // Get the shape of the extended separation set.
        let c_s = q_xs.conditioning_shape();
        // Get the shape of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        for j in 0..n_xs.shape()[0] {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the Fisher-Snedecor distribution.
            let r_xz = n_xz.index_axis(Axis(0), i).sum_axis(Axis(1));
            let r_xs = n_xs.index_axis(Axis(0), j).sum_axis(Axis(1));
            // Get the intensity matrices for the separation sets.
            let q_xz = q_xz.parameters().index_axis(Axis(0), i);
            let q_xs = q_xs.parameters().index_axis(Axis(0), j);
            // Perform the F-test.
            let all_passed = Zip::from(&r_xz)
                .and(&r_xs)
                .and(q_xz.diag())
                .and(q_xs.diag())
                .fold(Ok(true), |acc, &r_xz, &r_xs, &q_xz, &q_xs| -> Result<_> {
                    if let Ok(true) = acc {
                        // Initialize the Fisher-Snedecor distribution.
                        let f = FisherSnedecor::new(r_xz, r_xs)
                            .map_err(|e| Error::Probability(&e.to_string()))?;
                        // Compute the p-value.
                        let p_value = f.cdf(q_xz / q_xs);
                        // Check if the p-value is in the alpha range.
                        if alpha.contains(&p_value) {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        acc
                    }
                })?;

            if !all_passed {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

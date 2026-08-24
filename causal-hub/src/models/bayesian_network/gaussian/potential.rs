use std::{
    borrow::Cow,
    ops::{Div, DivAssign, Mul, MulAssign},
};

use approx::{AbsDiffEq, RelativeEq};
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Determinant;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{GaussEv, GaussEvT},
    impl_json_io,
    models::{CPD, GaussCPD, GaussCPDP, GaussSupport, HasLabels, Phi},
    types::{Error, LN_2_PI, Labels, Result, Set},
    utils::PseudoInverse,
};

/// Parameters of a Gaussian potential.
#[derive(Clone, Debug)]
pub struct GaussPhiK {
    /// Precision matrix |X| x |X|.
    k: Array2<f64>,
    /// Information vector |X|.
    h: Array1<f64>,
    /// Log-normalization constant.
    graph: f64,
}

impl GaussPhiK {
    /// Creates a new Gaussian potential with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `k` - Precision matrix |X| x |X|.
    /// * `h` - Information vector |X|.
    /// * `g` - Log-normalization constant.
    ///
    /// # Panics
    ///
    /// * Panics if `k` is not square and symmetric.
    /// * Panics if the length of `h` does not match the size of `k`.
    /// * Panics if `k`, `h`, or `g` contain non-finite values.
    ///
    /// # Returns
    ///
    /// A new Gaussian potential instance.
    ///
    pub fn new(k: Array2<f64>, h: Array1<f64>, graph: f64) -> Result<Self> {
        // Check K is square.
        if !k.is_square() {
            return Err(Error::Shape("Precision matrix must be square."));
        }
        // Check the length of h matches the size of K.
        if k.nrows() != h.len() {
            return Err(Error::IncompatibleShape(
                &k.nrows().to_string(),
                &h.len().to_string(),
            ));
        }
        // Check K is finite.
        if !k.iter().all(|x| x.is_finite()) {
            return Err(Error::Linalg("Precision matrix must be finite."));
        }
        // Check K is symmetric.
        if k != k.t() {
            return Err(Error::Linalg("Precision matrix must be symmetric."));
        }
        // Check h is finite.
        if !h.iter().all(|x| x.is_finite()) {
            return Err(Error::Linalg("Information vector must be finite."));
        }
        // Check g is finite.
        if !graph.is_finite() {
            return Err(Error::Linalg("Log-normalization constant must be finite."));
        }

        Ok(Self { k, h, graph })
    }

    /// Internal constructor that assumes parameters are already valid.
    /// Only used within trait implementations where validation cannot fail.
    #[inline]
    fn from_valid_params(k: Array2<f64>, h: Array1<f64>, graph: f64) -> Self {
        Self { k, h, graph }
    }

    /// Returns the precision matrix.
    ///
    /// # Returns
    ///
    /// A reference to the precision matrix.
    ///
    #[inline]
    pub const fn precision_matrix(&self) -> &Array2<f64> {
        &self.k
    }

    /// Returns the information vector.
    ///
    /// # Returns
    ///
    /// A reference to the information vector.
    ///
    #[inline]
    pub const fn information_vector(&self) -> &Array1<f64> {
        &self.h
    }

    /// Returns the log-normalization constant.
    ///
    /// # Returns
    ///
    /// The log-normalization constant.
    ///
    #[inline]
    pub const fn log_normalization_constant(&self) -> f64 {
        self.graph
    }
}

impl PartialEq for GaussPhiK {
    fn eq(&self, other: &Self) -> bool {
        self.k.eq(&other.k) && self.h.eq(&other.h) && self.graph.eq(&other.graph)
    }
}

impl AbsDiffEq for GaussPhiK {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.k.abs_diff_eq(&other.k, epsilon)
            && self.h.abs_diff_eq(&other.h, epsilon)
            && self.graph.abs_diff_eq(&other.graph, epsilon)
    }
}

impl RelativeEq for GaussPhiK {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.k.relative_eq(&other.k, epsilon, max_relative)
            && self.h.relative_eq(&other.h, epsilon, max_relative)
            && self.graph.relative_eq(&other.graph, epsilon, max_relative)
    }
}

/// A Gaussian potential.
#[derive(Clone, Debug)]
pub struct GaussPhi {
    // Labels of the variables.
    labels: Labels,
    // Support (always (-inf, +inf) by default).
    support: GaussSupport,
    // Parameters.
    parameters: GaussPhiK,
}

impl GaussPhi {
    /// Creates a new Gaussian potential with the given labels and parameters.
    ///
    /// # Arguments
    ///
    /// * `labels` - Labels of the variables.
    /// * `parameters` - Parameters of the potential.
    ///
    /// # Results
    ///
    /// A new Gaussian potential instance.
    ///
    pub fn new(mut labels: Labels, mut parameters: GaussPhiK) -> Result<Self> {
        // Check parameters shape matches labels length.
        if parameters.precision_matrix().nrows() != labels.len() {
            return Err(Error::IncompatibleShape(
                "precision_matrix",
                "Precision matrix rows must match labels length.",
            ));
        }
        if parameters.information_vector().len() != labels.len() {
            return Err(Error::IncompatibleShape(
                "information_vector",
                "Information vector length must match labels length.",
            ));
        }

        // Sort labels if not sorted and permute parameters accordingly.
        if !labels.is_sorted() {
            // Get the new indices order w.r.t. sorted labels.
            let mut indices: Vec<_> = (0..labels.len()).collect();
            indices.sort_by(|&i, &j| labels.get_index(i).cmp(&labels.get_index(j)));
            // Sort the labels.
            labels.sort();

            // Clone the precision matrix.
            let mut k = parameters.k.clone();
            // Permute the precision matrix rows.
            indices.iter().enumerate().for_each(|(i, &j)| {
                k.row_mut(i).assign(&parameters.k.row(j));
            });
            parameters.k = k.clone();
            // Permute the precision matrix columns.
            indices.iter().enumerate().for_each(|(i, &j)| {
                k.column_mut(i).assign(&parameters.k.column(j));
            });
            parameters.k = k;

            // Clone the information vector.
            let mut h = parameters.h.clone();
            // Permute the information vector.
            indices.iter().enumerate().for_each(|(i, &j)| {
                h[i] = parameters.h[j];
            });
            parameters.h = h;
        }

        let support = labels
            .iter()
            .map(|l| (l.clone(), (f64::NEG_INFINITY, f64::INFINITY)))
            .collect();

        Ok(Self {
            labels,
            support,
            parameters,
        })
    }
}

impl HasLabels for GaussPhi {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for GaussPhi {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for GaussPhi {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self.parameters.abs_diff_eq(&other.parameters, epsilon)
    }
}

impl RelativeEq for GaussPhi {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl MulAssign<&GaussPhi> for GaussPhi {
    fn mul_assign(&mut self, rhs: &GaussPhi) {
        // Get the union of the labels.
        let mut labels = self.labels.clone();
        labels.extend(rhs.labels.clone());
        // Sort the labels.
        labels.sort();

        // Get the number of variables.
        let n = labels.len();

        // Order LHS indices w.r.t. new labels.
        let lhs_m: Vec<_> = labels.iter().map(|l| self.labels.get_index_of(l)).collect();
        // Allocate extended LHS parameters.
        let lhs_k = Array::from_shape_fn((n, n), |(i, j)| match (lhs_m[i], lhs_m[j]) {
            (Some(i), Some(j)) => self.parameters.k[[i, j]],
            _ => 0.,
        });
        let lhs_h = Array::from_shape_fn(n, |i| match lhs_m[i] {
            Some(i) => self.parameters.h[i],
            _ => 0.,
        });
        let lhs_g = self.parameters.graph;

        // Order RHS indices w.r.t. new labels.
        let rhs_m: Vec<_> = labels.iter().map(|l| rhs.labels.get_index_of(l)).collect();
        // Allocate extended RHS parameters.
        let rhs_k = Array::from_shape_fn((n, n), |(i, j)| match (rhs_m[i], rhs_m[j]) {
            (Some(i), Some(j)) => rhs.parameters.k[[i, j]],
            _ => 0.,
        });
        let rhs_h = Array::from_shape_fn(n, |i| match rhs_m[i] {
            Some(i) => rhs.parameters.h[i],
            _ => 0.,
        });
        let rhs_g = rhs.parameters.graph;

        // Sum parameters.
        let k = lhs_k + rhs_k;
        let h = lhs_h + rhs_h;
        let graph = lhs_g + rhs_g;
        // Assemble parameters. Since we're combining valid parameters, the result is valid.
        let parameters = GaussPhiK::from_valid_params(k, h, graph);

        // Update the labels.
        self.labels = labels;
        // Update the parameters.
        self.parameters = parameters;
        // Update the support.
        self.support = self
            .labels
            .iter()
            .map(|l| (l.clone(), (f64::NEG_INFINITY, f64::INFINITY)))
            .collect();
    }
}

impl Mul<&GaussPhi> for &GaussPhi {
    type Output = GaussPhi;

    #[inline]
    fn mul(self, rhs: &GaussPhi) -> Self::Output {
        let mut lhs = self.clone();
        lhs *= rhs;
        lhs
    }
}

impl DivAssign<&GaussPhi> for GaussPhi {
    fn div_assign(&mut self, rhs: &GaussPhi) {
        // Get the union of the labels.
        let mut labels = self.labels.clone();
        labels.extend(rhs.labels.clone());
        // Sort the labels.
        labels.sort();

        // Get the number of variables.
        let n = labels.len();

        // Order LHS indices w.r.t. new labels.
        let lhs_m: Vec<_> = labels.iter().map(|l| self.labels.get_index_of(l)).collect();
        // Allocate extended LHS parameters.
        let lhs_k = Array::from_shape_fn((n, n), |(i, j)| match (lhs_m[i], lhs_m[j]) {
            (Some(i), Some(j)) => self.parameters.k[[i, j]],
            _ => 0.,
        });
        let lhs_h = Array::from_shape_fn(n, |i| match lhs_m[i] {
            Some(i) => self.parameters.h[i],
            _ => 0.,
        });
        let lhs_g = self.parameters.graph;

        // Order RHS indices w.r.t. new labels.
        let rhs_m: Vec<_> = labels.iter().map(|l| rhs.labels.get_index_of(l)).collect();
        // Allocate extended RHS parameters.
        let rhs_k = Array::from_shape_fn((n, n), |(i, j)| match (rhs_m[i], rhs_m[j]) {
            (Some(i), Some(j)) => rhs.parameters.k[[i, j]],
            _ => 0.,
        });
        let rhs_h = Array::from_shape_fn(n, |i| match rhs_m[i] {
            Some(i) => rhs.parameters.h[i],
            _ => 0.,
        });
        let rhs_g = rhs.parameters.graph;

        // Sum parameters.
        let k_prime = lhs_k - rhs_k;
        let h_prime = lhs_h - rhs_h;
        let g_prime = lhs_g - rhs_g;
        // Assemble parameters. Since we're combining valid parameters, the result is valid.
        let parameters = GaussPhiK::from_valid_params(k_prime, h_prime, g_prime);

        // Update the labels.
        self.labels = labels;
        // Update the parameters.
        self.parameters = parameters;
        // Update the support.
        self.support = self
            .labels
            .iter()
            .map(|l| (l.clone(), (f64::NEG_INFINITY, f64::INFINITY)))
            .collect();
    }
}

impl Div<&GaussPhi> for &GaussPhi {
    type Output = GaussPhi;

    #[inline]
    fn div(self, rhs: &GaussPhi) -> Self::Output {
        let mut lhs = self.clone();
        lhs /= rhs;
        lhs
    }
}

impl Phi for GaussPhi {
    type CPD = GaussCPD;
    type Support = GaussSupport;
    type Parameters = GaussPhiK;
    type Evidence = GaussEv;

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
    }

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    #[inline]
    fn parameters_size(&self) -> usize {
        let k = {
            // Precision matrix is symmetric.
            let k = self.parameters.k.nrows();
            k * (k + 1) / 2
        };

        k + self.parameters.h.len() + 1
    }

    fn condition(&self, evidence: &Self::Evidence) -> Result<Self> {
        // Check that the evidence labels match the potential labels.
        if evidence.labels() != self.labels() {
            return Err(Error::InvalidParameter(
                "evidence",
                &format!(
                    "Failed to condition on evidence: \n\
                    \t expected:    evidence labels to match potential labels , \n\
                    \t found:       potential labels = {:?} , \n\
                    \t              evidence  labels = {:?} .",
                    self.labels(),
                    evidence.labels(),
                ),
            ));
        }

        // Get the evidence and remove nones.
        let evidence = evidence.evidences().iter().flatten().cloned();
        // Check that the evidence is certain and positive.
        let evidence = evidence.map(|evidence| match evidence {
            GaussEvT::CertainPositive { event, value } => (event, value),
            /* _ => panic! NOTE: No other variant so far. */
        });

        // Get X and Y from the evidence.
        let y: Set<_> = evidence.clone().map(|(event, _)| event).collect();
        let x: Set<_> = &Set::from_iter(0..self.labels.len()) - &y;

        // Select the labels of the conditioned potential.
        let labels: Labels = x.iter().map(|&x| self.labels[x].clone()).collect();

        // Get the values from the evidence.
        let _y = Array::from_iter(evidence.map(|(_, value)| value));

        // Get the precision matrix.
        let k = self.parameters.precision_matrix();
        // Get the information vector.
        let h = self.parameters.information_vector();
        // Get the log-normalization constant.
        let graph = self.parameters.log_normalization_constant();

        // Compute the precision matrix as K_xx from K and X.
        let k_prime = Array::from_shape_fn((x.len(), x.len()), |(i, j)| k[[x[i], x[j]]]);
        // Compute the information vector.
        let h_prime = {
            // Get K_xy from K, X and Y.
            let k_xy = Array::from_shape_fn((x.len(), y.len()), |(i, j)| k[[x[i], y[j]]]);
            // Get h_x from h and X.
            let h_x = Array::from_shape_fn(x.len(), |i| h[x[i]]);
            // Compute h as: h' = h_x - K_xy * y.
            h_x - k_xy.dot(&_y)
        };
        // Compute the log-normalization constant.
        let g_prime = {
            // Get K_yy from K and Y.
            let k_yy = Array::from_shape_fn((y.len(), y.len()), |(i, j)| k[[y[i], y[j]]]);
            // Get h_y from h and Y.
            let h_y = Array::from_shape_fn(y.len(), |i| h[y[i]]);
            // Compute g as: g' = g + h_y^T * y - 0.5 * y^T * K_yy * y.
            graph + h_y.dot(&_y) - 0.5 * _y.dot(&k_yy).dot(&_y)
        };

        // Assemble the parameters.
        let parameters = GaussPhiK::new(k_prime, h_prime, g_prime)?;

        // Return the conditioned potential.
        Self::new(labels, parameters)
    }

    fn marginalize(&self, x: &Set<usize>) -> Result<Self> {
        // Base case: if no variables to marginalize, return self.
        if x.is_empty() {
            return Ok(self.clone());
        }

        // Check X is a subset of the variables.
        x.iter().try_for_each(|&x| {
            if x >= self.labels.len() {
                return Err(Error::IndexOutOfBounds(x));
            }
            Ok(())
        })?;

        // Get Z as V \ X.
        let v: Set<_> = Set::from_iter(0..self.labels.len());
        let z: Set<_> = &v - x;

        // Get the labels of the marginalized potential.
        let labels_z: Labels = z.iter().map(|&i| self.labels[i].clone()).collect();

        // Get the precision matrix.
        let k = self.parameters.precision_matrix();
        // Get the information vector.
        let h = self.parameters.information_vector();
        // Get the log-normalization constant.
        let graph = self.parameters.log_normalization_constant();

        // Compute the covariance matrix as: S_xx = (K_xx)^(-1).
        let s_xx = {
            // Get K_xx from K and X.
            let k_xx = Array::from_shape_fn((x.len(), x.len()), |(i, j)| k[[x[i], x[j]]]);
            // Compute the covariance as: S = (K_xx)^(-1)
            k_xx.pinv()?
        };
        // Get K_zx from K, Z and X.
        let k_zx = Array::from_shape_fn((z.len(), x.len()), |(i, j)| k[[z[i], x[j]]]);
        // Get h_x from h and X.
        let h_x = Array::from_shape_fn(x.len(), |i| h[x[i]]);

        // Compute K_zx * S_xx once.
        let k_zx_dot_s_xx = k_zx.dot(&s_xx);

        // Compute the marginalized precision matrix.
        let k_prime = {
            // Get K_zz and K_xz from K, X and Z.
            let k_zz = Array::from_shape_fn((z.len(), z.len()), |(i, j)| k[[z[i], z[j]]]);
            let k_xz = Array::from_shape_fn((x.len(), z.len()), |(i, j)| k[[x[i], z[j]]]);
            // Compute the precision matrix as: K' = K_zz - K_zx * (K_xx)^(-1) * K_xz
            k_zz - k_zx_dot_s_xx.dot(&k_xz)
        };
        // Compute the marginalized information vector.
        let h_prime = {
            // Get h_z from h, X and Z.
            let h_z = Array::from_shape_fn(z.len(), |i| h[z[i]]);
            // Compute the information vector as: h' = h_z - K_zx * (K_xx)^(-1) * h_x
            h_z - k_zx_dot_s_xx.dot(&h_x)
        };
        // Compute the marginalized log-normalization constant.
        let g_prime = {
            // Compute the log-normalization constant as: g' = g + 0.5 * (ln|2 pi (K_xx)^-1| + h_x^T * (K_xx)^-1 * h_x)
            let n_ln_2_pi = s_xx.nrows() as f64 * LN_2_PI;
            let (_, ln_det) = s_xx.sln_det().map_err(|evidence| {
                Error::Linalg(&format!("Failed to compute the determinant: {evidence}"))
            })?;
            graph + 0.5 * (n_ln_2_pi + ln_det + h_x.dot(&s_xx).dot(&h_x))
        };

        // Assemble the parameters.
        let parameters = GaussPhiK::new(k_prime, h_prime, g_prime)?;

        // Return the marginalized potential.
        Self::new(labels_z, parameters)
    }

    #[inline]
    fn normalize(&self) -> Result<Self> {
        // The potential is already normalized.
        Ok(self.clone())
    }

    fn from_cpd(distribution: Self::CPD) -> Result<Self> {
        // Merge labels and conditioning labels in this order.
        let mut labels = distribution.labels().clone();
        labels.extend(distribution.conditioning_labels().clone());

        // Get the parameters from the CPD.
        let parameters = distribution.parameters();
        // Get the coefficients and covariance.
        let (a, b, stats) = (
            parameters.coefficients(),
            parameters.intercept(),
            parameters.covariance(),
        );

        // Compute the precision matrix as:
        //
        // | K_xx  K_xz |
        // | K_zx  K_zz |
        //
        let k_xx = stats.pinv()?; //                 Precision of X.
        let k_xz = -&k_xx.dot(a); //            Cross-precision of X and Z.
        let k_zx = -a.t().dot(&k_xx); //        Cross-precision of Z and X.
        let k_zz = a.t().dot(&k_xx).dot(a); //  Induced precision of Z.
        // Assemble the precision matrix.
        let k_prime = {
            let (n, model) = (a.nrows(), a.ncols());
            let mut k = Array::zeros((n + model, n + model));
            k.slice_mut(s![0..n, 0..n]).assign(&k_xx);
            k.slice_mut(s![0..n, n..n + model]).assign(&k_xz);
            k.slice_mut(s![n..n + model, 0..n]).assign(&k_zx);
            k.slice_mut(s![n..n + model, n..n + model]).assign(&k_zz);
            k
        };

        // Compute the information vector as:
        //
        // | h_x | = | K_xx * b |
        // | h_z | = | K_zx * b |
        //
        let h_x = k_xx.dot(b); // Information of X.
        let h_z = k_zx.dot(b); // Information of Z.
        // Assemble the information vector.
        let h_prime = {
            let mut h = Array::zeros(h_x.len() + h_z.len());
            h.slice_mut(s![0..h_x.len()]).assign(&h_x);
            h.slice_mut(s![h_x.len()..]).assign(&h_z);
            h
        };

        // Compute the log-normalization constant.
        let g_prime = {
            let n_ln_2_pi = stats.nrows() as f64 * LN_2_PI;
            let (_, ln_det) = stats.sln_det().map_err(|evidence| {
                Error::Linalg(&format!("Failed to compute the determinant: {evidence}"))
            })?;
            -0.5 * (n_ln_2_pi + ln_det + b.dot(&h_x))
        };

        // Construct the parameters.
        let parameters = GaussPhiK::new(k_prime, h_prime, g_prime)?;

        // Return the potential.
        Self::new(labels, parameters)
    }

    fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> Result<Self::CPD> {
        // Check that X and Z are disjoint.
        if !x.is_disjoint(z) {
            return Err(Error::SetsNotDisjoint(
                "variables",
                "conditioning variables",
            ));
        }
        // Check that X and Z cover all variables.
        if !(x | z).iter().sorted().cloned().eq(0..self.labels.len()) {
            return Err(Error::InvalidParameter(
                "variables",
                "Variables and conditioning variables must cover all potential variables.",
            ));
        }

        // Split labels into labels and conditioning labels.
        let labels_x: Labels = x.iter().map(|&i| self.labels[i].clone()).collect();
        let labels_z: Labels = z.iter().map(|&i| self.labels[i].clone()).collect();

        // Get the precision matrix.
        let k = self.parameters.precision_matrix();
        // Get the information vector.
        let h = self.parameters.information_vector();

        // Compute the covariance matrix.
        let stats = {
            // Get K_xx from K and X.
            let k_xx = Array::from_shape_fn((x.len(), x.len()), |(i, j)| k[[x[i], x[j]]]);
            // Compute the covariance as: S = (K_xx)^(-1)
            k_xx.pinv()?
        };
        // Compute the coefficient matrix.
        let a = {
            // Get K_xz from K, X, and Z.
            let k_xz = Array::from_shape_fn((x.len(), z.len()), |(i, j)| k[[x[i], z[j]]]);
            // Compute the coefficients as: A = - (K_xx)^(-1) * K_xz
            -stats.dot(&k_xz)
        };
        // Compute the intercept vector.
        let b = {
            // Get h_x from h and X.
            let h_x = Array::from_shape_fn(x.len(), |i| h[x[i]]);
            // Compute the intercept as: b = (K_xx)^(-1) * h_x
            stats.dot(&h_x)
        };

        // Assemble the parameters.
        let parameters = GaussCPDP::new(a, b, stats)?;

        // Create the new CPD.
        GaussCPD::new(labels_x, labels_z, parameters)
    }
}

impl Serialize for GaussPhiK {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(3))?;

        // Convert the precision matrix to a flat format.
        let precision_matrix: Vec<Vec<f64>> =
            self.k.rows().into_iter().map(|x| x.to_vec()).collect();
        // Serialize precision matrix.
        map.serialize_entry("precision_matrix", &precision_matrix)?;

        // Convert the information vector to a flat format.
        let information_vector = self.h.to_vec();
        // Serialize information vector.
        map.serialize_entry("information_vector", &information_vector)?;

        // Serialize log-normalization constant.
        map.serialize_entry("log_normalization_constant", &self.graph)?;

        // End the map.
        map.end()
    }
}

impl<'de> Deserialize<'de> for GaussPhiK {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            PrecisionMatrix,
            InformationVector,
            LogNormalizationConstant,
        }

        struct GaussPhiKVisitor;

        impl<'de> Visitor<'de> for GaussPhiKVisitor {
            type Value = GaussPhiK;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GaussPhiK")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GaussPhiK, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut precision_matrix = None;
                let mut information_vector = None;
                let mut log_normalization_constant = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::PrecisionMatrix => {
                            if precision_matrix.is_some() {
                                return Err(E::duplicate_field("precision_matrix"));
                            }
                            precision_matrix = Some(map.next_value()?);
                        }
                        Field::InformationVector => {
                            if information_vector.is_some() {
                                return Err(E::duplicate_field("information_vector"));
                            }
                            information_vector = Some(map.next_value()?);
                        }
                        Field::LogNormalizationConstant => {
                            if log_normalization_constant.is_some() {
                                return Err(E::duplicate_field("log_normalization_constant"));
                            }
                            log_normalization_constant = Some(map.next_value()?);
                        }
                    }
                }

                // Extract the fields.
                let precision_matrix: Vec<Vec<f64>> =
                    precision_matrix.ok_or_else(|| E::missing_field("precision_matrix"))?;
                let information_vector: Vec<f64> =
                    information_vector.ok_or_else(|| E::missing_field("information_vector"))?;
                let log_normalization_constant: f64 = log_normalization_constant
                    .ok_or_else(|| E::missing_field("log_normalization_constant"))?;

                // Convert precision matrix to array.
                let shape = (precision_matrix.len(), precision_matrix[0].len());
                let k = Array::from_iter(precision_matrix.into_iter().flatten())
                    .into_shape_with_order(shape)
                    .map_err(|_| E::custom("Invalid precision matrix shape"))?;
                // Convert information vector to array.
                let h = Array1::from_vec(information_vector);

                GaussPhiK::new(k, h, log_normalization_constant).map_err(E::custom)
            }
        }

        const FIELDS: &[&str] = &[
            "precision_matrix",
            "information_vector",
            "log_normalization_constant",
        ];

        deserializer.deserialize_struct("GaussPhiK", FIELDS, GaussPhiKVisitor)
    }
}

impl Serialize for GaussPhi {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(3))?;

        // Serialize labels.
        let labels: Vec<&str> = self.labels.iter().map(|l| l.as_str()).collect();
        map.serialize_entry("labels", &labels)?;

        // Serialize parameters.
        map.serialize_entry("parameters", &self.parameters)?;

        // Serialize type.
        map.serialize_entry("type", "gaussphi")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for GaussPhi {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Labels,
            Parameters,
            Type,
        }

        struct GaussPhiVisitor;

        impl<'de> Visitor<'de> for GaussPhiVisitor {
            type Value = GaussPhi;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct GaussPhi")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GaussPhi, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut labels = None;
                let mut parameters = None;
                let mut type_ = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Labels => {
                            if labels.is_some() {
                                return Err(E::duplicate_field("labels"));
                            }
                            labels = Some(map.next_value()?);
                        }
                        Field::Parameters => {
                            if parameters.is_some() {
                                return Err(E::duplicate_field("parameters"));
                            }
                            parameters = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                // Check required fields.
                let labels: Vec<String> = labels.ok_or_else(|| E::missing_field("labels"))?;
                let parameters: GaussPhiK =
                    parameters.ok_or_else(|| E::missing_field("parameters"))?;

                // Check type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                if type_ != "gaussphi" {
                    return Err(E::custom(format!(
                        "Invalid type for GaussPhi: expected 'gaussphi', found '{type_}'"
                    )));
                }

                let labels: Labels = labels.into_iter().collect();
                GaussPhi::new(labels, parameters).map_err(E::custom)
            }
        }

        const FIELDS: &[&str] = &["labels", "parameters", "type"];

        deserializer.deserialize_struct("GaussPhi", FIELDS, GaussPhiVisitor)
    }
}

// Implement `JsonIO` for `GaussPhi`.
impl_json_io!(GaussPhi);

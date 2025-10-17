use std::{
    f64::consts::PI,
    ops::{Div, DivAssign, Mul, MulAssign},
};

use approx::{AbsDiffEq, RelativeEq};
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Determinant;

use crate::{
    models::{CPD, GaussCPD, GaussCPDP, Labelled, Phi},
    types::{Labels, Set},
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
    g: f64,
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
    /// # Results
    ///
    /// A new Gaussian potential instance.
    ///
    pub fn new(k: Array2<f64>, h: Array1<f64>, g: f64) -> Self {
        // Assert K is square.
        assert!(k.is_square(), "Precision matrix must be square.");
        // Assert the length of h matches the size of K.
        assert_eq!(
            k.nrows(),
            h.len(),
            "Information vector length must match precision matrix size."
        );
        // Assert K is finite.
        assert!(
            k.iter().all(|x| x.is_finite()),
            "Precision matrix must be finite."
        );
        // Assert K is symmetric.
        assert_eq!(k, k.t(), "Precision matrix must be symmetric.");
        // Assert h is finite.
        assert!(
            h.iter().all(|x| x.is_finite()),
            "Information vector must be finite."
        );
        // Assert g is finite.
        assert!(g.is_finite(), "Log-normalization constant must be finite.");

        Self { k, h, g }
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
        self.g
    }
}

impl PartialEq for GaussPhiK {
    fn eq(&self, other: &Self) -> bool {
        self.k.eq(&other.k) && self.h.eq(&other.h) && self.g.eq(&other.g)
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
            && self.g.abs_diff_eq(&other.g, epsilon)
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
            && self.g.relative_eq(&other.g, epsilon, max_relative)
    }
}

/// A Gaussian potential.
#[derive(Clone, Debug)]
pub struct GaussPhi {
    // Labels of the variables.
    labels: Labels,
    // Parameters.
    parameters: GaussPhiK,
}

impl Labelled for GaussPhi {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for GaussPhi {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels) && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for GaussPhi {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels) && self.parameters.abs_diff_eq(&other.parameters, epsilon)
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
        let lhs_g = self.parameters.g;

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
        let rhs_g = rhs.parameters.g;

        // Sum parameters.
        let k = lhs_k + rhs_k;
        let h = lhs_h + rhs_h;
        let g = lhs_g + rhs_g;
        // Assemble parameters.
        let parameters = GaussPhiK::new(k, h, g);

        // Update the labels.
        self.labels = labels;
        // Update the parameters.
        self.parameters = parameters;
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
        let lhs_g = self.parameters.g;

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
        let rhs_g = rhs.parameters.g;

        // Sum parameters.
        let k = lhs_k - rhs_k;
        let h = lhs_h - rhs_h;
        let g = lhs_g - rhs_g;
        // Assemble parameters.
        let parameters = GaussPhiK::new(k, h, g);

        // Update the labels.
        self.labels = labels;
        // Update the parameters.
        self.parameters = parameters;
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
    type Parameters = GaussPhiK;
    type Evidence = (); // FIXME:

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

    fn condition(&self, _e: &Self::Evidence) -> Self {
        todo!() // FIXME:
    }

    fn marginalize(&self, _x: &Set<usize>) -> Self {
        todo!() // FIXME:
    }

    fn normalize(&self) -> Self {
        todo!() // FIXME:
    }

    fn from_cpd(cpd: Self::CPD) -> Self {
        // Merge labels and conditioning labels in this order.
        let mut labels = cpd.labels().clone();
        labels.extend(cpd.conditioning_labels().clone());

        // Get the parameters from the CPD.
        let parameters = cpd.parameters();
        // Get the coefficients and covariance.
        let (a, b, s) = (
            parameters.coefficients(),
            parameters.intercept(),
            parameters.covariance(),
        );

        // Compute the precision matrix as:
        //
        // | K_xx  K_xz |
        // | K_zx  K_zz |
        //
        let k_xx = s.pinv(); //                 Precision of X.
        let k_xz = -&k_xx.dot(a); //            Cross-precision of X and Z.    
        let k_zx = -a.t().dot(&k_xx); //        Cross-precision of Z and X.
        let k_zz = a.t().dot(&k_xx).dot(a); //  Induced precision of Z.
        // Assemble the precision matrix.
        let k = {
            let (n, m) = (a.nrows(), a.ncols());
            let mut k = Array::zeros((n + m, n + m));
            k.slice_mut(s![0..n, 0..n]).assign(&k_xx);
            k.slice_mut(s![0..n, n..n + m]).assign(&k_xz);
            k.slice_mut(s![n..n + m, 0..n]).assign(&k_zx);
            k.slice_mut(s![n..n + m, n..n + m]).assign(&k_zz);
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
        let h = {
            let mut h = Array::zeros(h_x.len() + h_z.len());
            h.slice_mut(s![0..h_x.len()]).assign(&h_x);
            h.slice_mut(s![h_x.len()..]).assign(&h_z);
            h
        };

        // Compute the log-normalization constant.
        let g = (2. * PI * s).det().expect("Failed to compute determinant.");
        let g = -0.5 * (b.dot(&h_x) + f64::ln(g));

        // Construct the parameters.
        let parameters = GaussPhiK::new(k, h, g);

        // Return the potential.
        Self::new(labels, parameters)
    }

    fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> Self::CPD {
        // Assert that X and Z are disjoint.
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );
        // Assert that X and Z cover all variables.
        assert!(
            (x | z).iter().sorted().cloned().eq(0..self.labels.len()),
            "Variables and conditioning variables must cover all potential variables."
        );

        // Split labels into labels and conditioning labels.
        let labels_x: Labels = x.iter().map(|&i| self.labels[i].clone()).collect();
        let labels_z: Labels = z.iter().map(|&i| self.labels[i].clone()).collect();

        // Get the precision matrix.
        let k = self.parameters.precision_matrix();
        // Get the information vector.
        let h = self.parameters.information_vector();

        // Compute the covariance matrix.
        let s = {
            // Get K_xx from K and X.
            let k_xx = Array::from_shape_fn((x.len(), x.len()), |(i, j)| k[[x[i], x[j]]]);
            // Compute the covariance as: S = (K_xx)^(-1)
            k_xx.pinv()
        };
        // Compute the coefficient matrix.
        let a = {
            // Get K_xz from K, X, and Z.
            let k_xz = Array::from_shape_fn((x.len(), z.len()), |(i, j)| k[[x[i], z[j]]]);
            // Compute the coefficients as: A = - (K_xx)^(-1) * K_xz
            -s.dot(&k_xz)
        };
        // Compute the intercept vector.
        let b = {
            // Get h_x from h and X.
            let h_x = Array::from_shape_fn(x.len(), |i| h[x[i]]);
            // Compute the intercept as: b = (K_xx)^(-1) * h_x
            s.dot(&h_x)
        };

        // Assemble the parameters.
        let parameters = GaussCPDP::new(a, b, s);

        // Create the new CPD.
        GaussCPD::new(labels_x, labels_z, parameters)
    }
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
    pub fn new(mut labels: Labels, mut parameters: GaussPhiK) -> Self {
        // Assert parameters shape matches labels length.
        assert_eq!(
            parameters.precision_matrix().nrows(),
            labels.len(),
            "Precision matrix rows must match labels length."
        );
        assert_eq!(
            parameters.information_vector().len(),
            labels.len(),
            "Information vector length must match labels length."
        );

        // Sort labels if not sorted and permute parameters accordingly.
        if !labels.is_sorted() {
            // Get the new indices order w.r.t. sorted labels.
            let mut indices: Vec<_> = (0..labels.len()).collect();
            indices.sort_by_key(|&i| labels.get_index(i).unwrap());
            // Sort the labels.
            labels.sort();

            // Clone the precision matrix.
            let mut k = parameters.k.clone();
            // Permute the precision matrix rows.
            for (i, &j) in indices.iter().enumerate() {
                k.row_mut(i).assign(&parameters.k.row(j));
            }
            parameters.k = k.clone();
            // Permute the precision matrix columns.
            for (i, &j) in indices.iter().enumerate() {
                k.column_mut(i).assign(&parameters.k.column(j));
            }
            parameters.k = k;

            // Clone the information vector.
            let mut h = parameters.h.clone();
            // Permute the information vector.
            for (i, &j) in indices.iter().enumerate() {
                h[i] = parameters.h[j];
            }
            parameters.h = h;
        }

        Self { labels, parameters }
    }
}

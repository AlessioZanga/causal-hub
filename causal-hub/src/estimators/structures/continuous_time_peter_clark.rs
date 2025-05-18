use itertools::Itertools;
use ndarray::{Zip, prelude::*};
use statrs::distribution::{ChiSquared, ContinuousCDF, FisherSnedecor};

use crate::{
    distributions::{CPD, CatCIM},
    estimators::CPDEstimator,
    graphs::{DiGraph, Graph},
};

/// A trait for conditional independence testing.
pub trait ConditionaIndependenceTest {
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
    fn test(&self, x: usize, y: usize, z: &[usize]) -> bool;
}

/// A type alias for a conditional independence test.
pub use ConditionaIndependenceTest as CIT;

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
    /// # Panics
    ///
    /// Panics if the significance level is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `ChiSquaredTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Self {
        // Assert that the significance level is in [0, 1].
        assert!((0.0..=1.0).contains(&alpha), "Alpha must be in [0, 1]");

        Self { estimator, alpha }
    }
}

impl<E> CIT for ChiSquaredTest<'_, E>
where
    E: CPDEstimator<CatCIM, SS = (Array3<f64>, Array2<f64>, f64)>,
{
    fn test(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute the extended separation set.
        let mut s = z.to_vec();
        // Get the ordered position of Y in the extended separation set.
        let s_y = z.binary_search(&y).unwrap_err();
        // Insert Y into the extended separation set in sorted order.
        s.insert(s_y, y);

        // Get the sufficient statistics and the intensity matrices for the sets.
        let ((n_xz, _, _), _q_xz) = self.estimator.fit_transform(x, &z);
        let ((n_xs, _, _), _q_xs) = self.estimator.fit_transform(x, &s);

        // Get the cardinality of the extended separation set.
        let c_s = _q_xs.conditioning_cardinality();
        // Get the cardinality of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        (0..n_xs.shape()[0]).all(|j| {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the chi-squared distribution.
            let k_xz = n_xz.index_axis(Axis(0), i).sum_axis(Axis(1));
            let k_xs = n_xs.index_axis(Axis(0), j).sum_axis(Axis(1));
            let k = (&k_xz / &k_xs).sqrt().insert_axis(Axis(1));
            let l = (&k).recip();
            // Compute the chi-squared statistic.
            let mut chi_sq = (&k * &k_xs - &l * &k_xz).powi(2) / (&k_xz + &k_xs);
            chi_sq.diag_mut().fill(0.);
            let chi_sq = chi_sq.sum_axis(Axis(1));
            // Initialize the chi-squared distribution.
            let n = ChiSquared::new((chi_sq.dim() - 1) as f64).unwrap();
            // For each chi-squared statistic ...
            chi_sq
                .into_iter()
                // Compute the p-value.
                .map(|x| n.cdf(x))
                // Check if the p-value is in the alpha range.
                .all(|p_value| p_value < (1. - self.alpha))
        })
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
    /// # Panics
    ///
    /// Panics if the significance level is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `FTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Self {
        // Assert that the significance level is in [0, 1].
        assert!((0.0..=1.0).contains(&alpha), "Alpha must be in [0, 1]");

        Self { estimator, alpha }
    }
}

impl<E> CIT for FTest<'_, E>
where
    E: CPDEstimator<CatCIM, SS = (Array3<f64>, Array2<f64>, f64)>,
{
    fn test(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute the alpha range.
        let alpha = (self.alpha / 2.)..=(1. - self.alpha / 2.);

        // Compute the extended separation set.
        let mut s = z.to_vec();
        // Get the ordered position of Y in the extended separation set.
        let s_y = z.binary_search(&y).unwrap_err();
        // Insert Y into the extended separation set in sorted order.
        s.insert(s_y, y);

        // Get the sufficient statistics and the intensity matrices for the sets.
        let ((n_xz, _, _), q_xz) = self.estimator.fit_transform(x, &z);
        let ((n_xs, _, _), q_xs) = self.estimator.fit_transform(x, &s);

        // Get the cardinality of the extended separation set.
        let c_s = q_xs.conditioning_cardinality();
        // Get the cardinality of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        (0..n_xs.shape()[0]).all(|j| {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the Fisher-Snedecor distribution.
            let r_xz = n_xz.index_axis(Axis(0), i).sum_axis(Axis(1));
            let r_xs = n_xs.index_axis(Axis(0), j).sum_axis(Axis(1));
            // Get the intensity matrices for the separation sets.
            let q_xz = q_xz.parameters().index_axis(Axis(0), i);
            let q_xs = q_xs.parameters().index_axis(Axis(0), j);
            // Perform the F-test.
            Zip::from(&r_xz)
                .and(&r_xs)
                .and(q_xz.diag())
                .and(q_xs.diag())
                .all(|&r_xz, &r_xs, &q_xz, &q_xs| {
                    // Initialize the Fisher-Snedecor distribution.
                    let f = FisherSnedecor::new(r_xz, r_xs).unwrap();
                    // Compute the p-value.
                    let p_value = f.cdf(q_xz / q_xs);
                    // Check if the p-value is in the alpha range.
                    alpha.contains(&p_value)
                })
        })
    }
}

/// A struct representing a continuous-time Peter-Clark estimator.
#[derive(Clone, Debug)]
pub struct ContinuousTimePeterClark<'a, T, S> {
    initial_graph: &'a DiGraph,
    null_time: &'a T,
    null_state: &'a S,
}

/// A type alias for the continuous-time Peter-Clark estimator.
pub type CTPC<'a, T, S> = ContinuousTimePeterClark<'a, T, S>;

impl<'a, T, S> ContinuousTimePeterClark<'a, T, S>
where
    T: CIT,
    S: CIT,
{
    /// Creates a new `ContinuousTimePeterClark` instance.
    ///
    /// # Arguments
    ///
    /// * `initial_graph` - A reference to the initial graph.
    /// * `null_time` - A reference to the null time to transition hypothesis test.
    /// * `null_state` - A reference to the null state-to-state transition hypothesis test.
    ///
    /// # Returns
    ///
    /// A new `ContinuousTimePeterClark` instance.
    ///
    #[inline]
    pub const fn new(initial_graph: &'a DiGraph, null_time: &'a T, null_state: &'a S) -> Self {
        Self {
            initial_graph,
            null_time,
            null_state,
        }
    }

    /// Execute the CTPC algorithm and return the fitted graph.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn fit(&self) -> DiGraph {
        // Clone the initial graph.
        let mut graph = self.initial_graph.clone();

        // For each vertex in the graph ...
        for i in graph.vertices() {
            // Get the parents of the vertex.
            let mut pa_i = graph.parents(i);

            // Initialize the counter.
            let mut k = 0;

            // While the counter is smaller than the number of parents ...
            while k < pa_i.len() {
                // Initialize the set of vertices to remove, to ensure stability.
                let mut not_pa_i = Vec::new();

                // For each parent ...
                for &j in &pa_i {
                    // Filter out X_j from the parents of X_i.
                    let pa_i_not_j = pa_i.iter().filter(|&&z| z != j).cloned();
                    // For any combination of size k of Pa(X_i) \ { X_j } ...
                    for s_ij in pa_i_not_j.combinations(k) {
                        // If X_i _||_ X_j | S_{X_i, X_j} ...
                        if self.null_time.test(i, j, &s_ij) && self.null_state.test(i, j, &s_ij) {
                            // Add the parent to the set of vertices to remove.
                            not_pa_i.push(j);
                            // Break the outer loop.
                            break;
                        }
                    }
                }

                // Remove the vertices from the graph.
                for &j in &not_pa_i {
                    // Remove the vertex from the parents.
                    pa_i.retain(|&x| x != j);
                    // Remove the edge from the graph.
                    graph.del_edge(j, i);
                }

                // Increment the counter.
                k += 1;
            }
        }

        // Return the fitted graph.
        graph
    }
}

use std::iter::Map;

use ndarray::prelude::*;
use statrs::function::gamma::gamma_lr;

use crate::{
    data::{CategoricalDataMatrix, JointConditionalCountMatrix, JointCountMatrix},
    prelude::{ConditionalIndependenceTest, DataSet, FxIndexSet},
    utils::nan_to_zero,
};

/// Chi Squared conditional independence test.
#[derive(Clone, Debug)]
pub struct ChiSquared<'a> {
    d: &'a CategoricalDataMatrix,
    alpha: f64,
}

impl<'a> ChiSquared<'a> {
    /// Construct Chi Squared conditional independence test with $\alpha = 0.05$ .
    #[inline]
    pub fn new(d: &'a CategoricalDataMatrix) -> Self {
        Self { d, alpha: 0.05 }
    }
}

impl<'a> From<&'a CategoricalDataMatrix> for ChiSquared<'a> {
    #[inline]
    fn from(d: &'a CategoricalDataMatrix) -> Self {
        Self::new(d)
    }
}

impl<'a> ConditionalIndependenceTest<'a> for ChiSquared<'a> {
    type LabelsIter<'b> =
        Map<indexmap::map::Keys<'b, String, FxIndexSet<String>>, fn(&'b String) -> &'b str> where Self: 'b;

    #[inline]
    fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64) {
        // Get cardinalities.
        let cards = self.d.cardinality();
        // Compute the degree of freedom as (|X| - 1) * (|Y| - 1) * \Pi(|Z|).
        let dof = (cards[x] as usize - 1)
            * (cards[y] as usize - 1)
            * z.iter().map(|&z| cards[z] as usize).product::<usize>();

        // Compute the joint contingency table.
        let n_ijk = match z.is_empty() {
            true => Array2::from(JointCountMatrix::new(self.d, x, y)).insert_axis(Axis(0)),
            false => JointConditionalCountMatrix::new(self.d, x, y, z).into(),
        };

        // Cast to float.
        let o_ijk = n_ijk.mapv(|x| x as f64);
        // Compute marginal counts.
        let o_ik = o_ijk.sum_axis(Axis(2)).insert_axis(Axis(2));
        let o_jk = o_ijk.sum_axis(Axis(1)).insert_axis(Axis(1));
        // Compute total counts.
        let o_k = o_ijk
            .sum_axis(Axis(2))
            .sum_axis(Axis(1))
            .insert_axis(Axis(1))
            .insert_axis(Axis(2));
        // Compute expected counts, mapping NaNs to zero.
        let e_ijk = ((o_ik * o_jk) / o_k).mapv(nan_to_zero);
        // Compute test statistic, mapping NaNs to zero.
        let stat = ((o_ijk - &e_ijk).mapv(|x| f64::powi(x, 2)) / e_ijk)
            .mapv(nan_to_zero)
            .sum();

        // Compute p-value as:
        //      |x| > \Phi^-1(1 - \alpha)
        //      \Phi(|x|) > 1 - \alpha
        //      1 - \Phi(|x|) < \alpha
        //      1 - P(k / 2, x / 2) < \alpha
        let pval = 1. - gamma_lr(dof as f64 * 0.5, stat * 0.5 + f64::EPSILON);

        (dof, stat, pval)
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute p-value.
        let (_, _, pval) = self.eval(x, y, z);

        pval > self.alpha
    }

    #[inline]
    fn with_significance_level(mut self, alpha: f64) -> Self {
        // Assert alpha in (0, 1).
        assert!((0. ..1.).contains(&alpha));
        // Set significance level.
        self.alpha = alpha;

        self
    }

    #[inline]
    fn labels(&self) -> Self::LabelsIter<'_> {
        self.d.labels_iter()
    }
}

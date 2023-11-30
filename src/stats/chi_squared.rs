use std::iter::Map;

use ndarray::prelude::*;
use statrs::function::gamma::gamma_lr;

use crate::{
    data::{CategoricalDataMatrix, JointConditionalCountMatrix, JointCountMatrix},
    prelude::{ConditionalIndependenceTest, DataSet, FxIndexSet},
    utils::nan_to_zero,
    L,
};

#[derive(Clone, Debug)]
pub struct ChiSquared<'a> {
    d: &'a CategoricalDataMatrix,
    alpha: f64,
}

impl<'a> ChiSquared<'a> {
    #[inline]
    pub fn new(d: &'a CategoricalDataMatrix, alpha: f64) -> Self {
        Self { d, alpha }
    }

    #[inline]
    pub fn eval(&self, x: usize, y: usize, z: &[usize]) -> (usize, f64, f64) {
        // Get cardinalities.
        let cards = self.d.cardinality();
        // Compute the degree of freedom as (|X| - 1) * (|Y| - 1) * \Pi(|Z|).
        let dof = (cards[x] as usize - 1)
            * (cards[y] as usize - 1)
            * z.iter().map(|&z| cards[z] as usize).product::<usize>();

        // Compute the joint contingency table.
        let n_ijk = if z.is_empty() {
            Array2::from(JointCountMatrix::new(self.d, x, y)).insert_axis(Axis(0))
        } else {
            JointConditionalCountMatrix::new(self.d, x, y, z).into()
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
}

impl<'a> ConditionalIndependenceTest for ChiSquared<'a> {
    type LabelsIter<'b> =
        Map<indexmap::map::Keys<'b, String, FxIndexSet<String>>, fn(&'b String) -> &'b str> where Self: 'b;

    #[inline]
    fn labels_iter(&self) -> Self::LabelsIter<'_> {
        L!(self.d)
    }

    #[inline]
    fn call(&self, x: usize, y: usize, z: &[usize]) -> bool {
        // Compute p-value.
        let (_, _, pval) = self.eval(x, y, z);

        pval > self.alpha
    }
}

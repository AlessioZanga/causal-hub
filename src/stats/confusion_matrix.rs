use std::ops::Deref;

use crate::{
    graphs::{DGraph, Graph, UGraph},
    V,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct ConfusionMatrix {
    // [TN, FP, FN, TP]
    c: [f64; 4],
}

impl Deref for ConfusionMatrix {
    type Target = [f64; 4];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

impl From<ConfusionMatrix> for [f64; 4] {
    fn from(other: ConfusionMatrix) -> Self {
        other.c
    }
}

impl<I, J> From<(I, J)> for ConfusionMatrix
where
    I: IntoIterator<Item = bool>,
    J: IntoIterator<Item = bool>,
{
    fn from((true_class, pred_class): (I, J)) -> Self {
        // Initialize confusion matrix.
        let mut c = [0.; 4];
        // Count classes pairs.
        for (p, t) in pred_class.into_iter().zip(true_class.into_iter()) {
            // Increment statistic.
            c[p as usize + 2 * t as usize] += 1.;
        }

        Self { c }
    }
}

impl<'a> From<(&'a UGraph, &'a UGraph)> for ConfusionMatrix {
    fn from((true_g, pred_g): (&'a UGraph, &'a UGraph)) -> Self {
        // Assert same vertices.
        assert_eq!(V!(true_g), V!(pred_g), "Graphs must have same vertex set");

        // Get graph order.
        let order = true_g.order();
        // Get edges iterators.
        let true_g = (0..order).flat_map(|x| (0..order).map(move |y| true_g.has_edge(x, y)));
        let pred_g = (0..order).flat_map(|x| (0..order).map(move |y| pred_g.has_edge(x, y)));

        Self::from((true_g, pred_g))
    }
}

impl<'a> From<(&'a DGraph, &'a DGraph)> for ConfusionMatrix {
    fn from((true_g, pred_g): (&'a DGraph, &'a DGraph)) -> Self {
        // Assert same vertices.
        assert_eq!(V!(true_g), V!(pred_g), "Graphs must have same vertex set");

        // Get graph order.
        let order = true_g.order();
        // Get edges iterators.
        let true_g = (0..order).flat_map(|x| (0..order).map(move |y| true_g.has_edge(x, y)));
        let pred_g = (0..order).flat_map(|x| (0..order).map(move |y| pred_g.has_edge(x, y)));

        Self::from((true_g, pred_g))
    }
}

impl ConfusionMatrix {
    #[inline]
    pub fn new(c: [f64; 4]) -> Self {
        // Check that all values are positives.
        assert!(
            c.iter().all(|&c| c >= 0.),
            "Confusion matrix entries must be non-negative"
        );

        Self { c }
    }

    #[inline]
    pub fn negative(&self) -> f64 {
        self.true_negative() + self.false_positive()
    }

    #[inline]
    pub fn true_negative(&self) -> f64 {
        self.c[0]
    }

    #[inline]
    pub fn false_negative(&self) -> f64 {
        self.c[2]
    }

    #[inline]
    pub fn true_negative_rate(&self) -> f64 {
        self.true_negative() / self.negative()
    }

    #[inline]
    pub fn false_negative_rate(&self) -> f64 {
        self.false_negative() / self.positive()
    }

    #[inline]
    pub fn negative_predictive_value(&self) -> f64 {
        self.true_negative() / (self.true_negative() + self.false_negative())
    }

    #[inline]
    pub fn false_omission_rate(&self) -> f64 {
        self.false_negative() / (self.false_negative() + self.true_negative())
    }

    #[inline]
    pub fn positive(&self) -> f64 {
        self.true_positive() + self.false_negative()
    }

    #[inline]
    pub fn true_positive(&self) -> f64 {
        self.c[3]
    }

    #[inline]
    pub fn false_positive(&self) -> f64 {
        self.c[1]
    }

    #[inline]
    pub fn true_positive_rate(&self) -> f64 {
        self.true_positive() / self.positive()
    }

    #[inline]
    pub fn false_positive_rate(&self) -> f64 {
        self.false_positive() / self.positive()
    }

    #[inline]
    pub fn positive_predictive_value(&self) -> f64 {
        self.true_positive() / (self.true_positive() + self.false_positive())
    }

    #[inline]
    pub fn false_discovery_rate(&self) -> f64 {
        self.false_positive() / (self.false_positive() + self.true_positive())
    }

    #[inline]
    pub fn accuracy(&self) -> f64 {
        (self.true_positive() + self.true_negative()) / (self.positive() + self.negative())
    }

    #[inline]
    pub fn balanced_accuracy(&self) -> f64 {
        (self.true_positive_rate() + self.true_negative_rate()) / 2.
    }

    #[inline]
    pub fn f1_score(&self) -> f64 {
        (2. * self.true_positive())
            / (2. * self.true_positive() + self.false_positive() + self.false_negative())
    }
}

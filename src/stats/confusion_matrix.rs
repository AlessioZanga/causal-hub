use std::ops::Deref;

use crate::{
    graphs::{BaseGraph, DiGraph, Graph},
    types::DenseAdjacencyMatrix,
    V,
};

/// (Binary) Confusion Matrix.
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

impl From<(Graph, Graph)> for ConfusionMatrix {
    fn from((true_graph, pred_graph): (Graph, Graph)) -> Self {
        // Assert same vertices.
        assert_eq!(
            V!(true_graph),
            V!(pred_graph),
            "Graphs must have same vertex set"
        );
        // Assert same memory layout.
        assert_eq!(
            true_graph.is_standard_layout(),
            pred_graph.is_standard_layout(),
            "Graphs must have same memory layout"
        );

        // Get adjacency matrices.
        let (_, true_graph): (_, DenseAdjacencyMatrix) = true_graph.into();
        let (_, pred_graph): (_, DenseAdjacencyMatrix) = pred_graph.into();

        // Get lower triangular.
        let true_graph = true_graph
            .indexed_iter()
            .filter_map(|((i, j), &f)| match i <= j {
                true => Some(f),
                false => None,
            });
        let pred_graph = pred_graph
            .indexed_iter()
            .filter_map(|((i, j), &f)| match i <= j {
                true => Some(f),
                false => None,
            });

        Self::from((true_graph, pred_graph))
    }
}

impl From<(DiGraph, DiGraph)> for ConfusionMatrix {
    fn from((true_graph, pred_graph): (DiGraph, DiGraph)) -> Self {
        // Assert same vertices.
        assert_eq!(
            V!(true_graph),
            V!(pred_graph),
            "Graphs must have same vertex set"
        );
        // Assert same memory layout.
        assert_eq!(
            true_graph.is_standard_layout(),
            pred_graph.is_standard_layout(),
            "Graphs must have same memory layout"
        );

        // Get adjacency matrices.
        let (_, true_graph): (_, DenseAdjacencyMatrix) = true_graph.into();
        let (_, pred_graph): (_, DenseAdjacencyMatrix) = pred_graph.into();

        Self::from((true_graph, pred_graph))
    }
}

impl ConfusionMatrix {
    /// Construct a new confusion matrix.
    #[inline]
    pub fn new(c: [f64; 4]) -> Self {
        // Check that all values are positives.
        assert!(
            c.iter().all(|&c| c >= 0.),
            "Confusion matrix entries must be non-negative"
        );

        Self { c }
    }

    /// Negative (N = TN + FP).
    #[inline]
    pub fn negative(&self) -> f64 {
        self.true_negative() + self.false_positive()
    }

    /// True negative (TN).
    #[inline]
    pub fn true_negative(&self) -> f64 {
        self.c[0]
    }

    /// False negative (FN).
    #[inline]
    pub fn false_negative(&self) -> f64 {
        self.c[2]
    }

    /// True negative rate, i.e. specificity (TNR = TN / N).
    #[inline]
    pub fn true_negative_rate(&self) -> f64 {
        self.true_negative() / self.negative()
    }

    /// False negative rate, i.e. miss-rate (FNR = FN / P).
    #[inline]
    pub fn false_negative_rate(&self) -> f64 {
        self.false_negative() / self.positive()
    }

    /// Negative predictive values (NPV = TN / (TN + FN)).
    #[inline]
    pub fn negative_predictive_value(&self) -> f64 {
        self.true_negative() / (self.true_negative() + self.false_negative())
    }

    /// False omission rate (FOR = FN / (FN + TN)).
    #[inline]
    pub fn false_omission_rate(&self) -> f64 {
        self.false_negative() / (self.false_negative() + self.true_negative())
    }

    /// Positive (P = TP + FN).
    #[inline]
    pub fn positive(&self) -> f64 {
        self.true_positive() + self.false_negative()
    }

    /// True positive (TP).
    #[inline]
    pub fn true_positive(&self) -> f64 {
        self.c[3]
    }

    /// False positive (FP).
    #[inline]
    pub fn false_positive(&self) -> f64 {
        self.c[1]
    }

    /// True positive rate, i.e. sensitivity, recall, hit-rate (TPR = TP / P).
    #[inline]
    pub fn true_positive_rate(&self) -> f64 {
        self.true_positive() / self.positive()
    }

    /// False positive rate, i.e. fall-out (FPR = FN / N).
    #[inline]
    pub fn false_positive_rate(&self) -> f64 {
        self.false_positive() / self.positive()
    }

    /// Positive predictive value, i.e. precision (PPV = TP / (TP + FP)).
    #[inline]
    pub fn positive_predictive_value(&self) -> f64 {
        self.true_positive() / (self.true_positive() + self.false_positive())
    }

    /// False discovery rate (FDR = FP / (FP + TP)).
    #[inline]
    pub fn false_discovery_rate(&self) -> f64 {
        self.false_positive() / (self.false_positive() + self.true_positive())
    }

    /// Accuracy (A = (TP + TN) / (P + N)).
    #[inline]
    pub fn accuracy(&self) -> f64 {
        (self.true_positive() + self.true_negative()) / (self.positive() + self.negative())
    }

    /// Balanced accuracy (BA = (TPR + TNR) / 2).
    #[inline]
    pub fn balanced_accuracy(&self) -> f64 {
        (self.true_positive_rate() + self.true_negative_rate()) / 2.
    }

    /// F1-Score (F1 = (2 * TP) / (2 * TP + FP + FN)).
    #[inline]
    pub fn f1_score(&self) -> f64 {
        (2. * self.true_positive())
            / (2. * self.true_positive() + self.false_positive() + self.false_negative())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use approx::*;
    use causal_hub::stats::ConfusionMatrix;

    #[test]
    fn from() {
        // Initialize classes.
        let true_class = vec![
            true, false, true, false, true, true, true, true, false, false, true, true, false,
            true, false, true, true, false, false, false, true, true, true, true, false, true,
            true, true, true, false, true, true, false, true, false, false, true, false, true,
            false, true, false, true, false, false, true, false, true, true, true, true, true,
            false, true, true, true, true, false, false, true, false, true, true, true, false,
            false, true, true, true, true, false, true, false, false, false, false, true, false,
            true, true, true, false, false, true, true, false, false, false, true, false, true,
            true, true, false, true, true, false, false, false, true,
        ];
        let pred_class = vec![
            false, true, true, true, false, false, false, false, false, false, false, true, false,
            true, false, false, false, false, false, false, false, false, true, true, false, true,
            true, false, false, true, false, true, false, true, false, false, false, false, true,
            false, false, false, true, false, true, true, true, false, false, false, true, false,
            false, false, false, true, false, true, true, false, true, true, false, false, true,
            false, true, true, false, false, true, false, true, false, false, false, false, true,
            true, true, false, true, false, false, false, false, true, true, true, false, false,
            false, true, true, false, true, false, false, false, false,
        ];
        // Construct confusion matrix.
        let cm = ConfusionMatrix::from((true_class, pred_class));
        // Deref slice.
        assert_eq!(
            cm.deref(),
            &[
                cm.true_negative(),
                cm.false_positive(),
                cm.false_negative(),
                cm.true_positive()
            ]
        );
        // Test confusion matrix.
        assert_relative_eq!(cm.accuracy(), 0.48);
        assert_relative_eq!(cm.balanced_accuracy(), 0.49917898193760263);
        assert_relative_eq!(cm.f1_score(), 0.45833333333333337);
        assert_relative_eq!(cm.false_discovery_rate(), 0.42105263157894735);
        assert_relative_eq!(cm.false_negative(), 36.);
        assert_relative_eq!(cm.false_negative_rate(), 0.6206896551724138);
        assert_relative_eq!(cm.false_omission_rate(), 0.5806451612903226);
        assert_relative_eq!(cm.false_positive(), 16.);
        assert_relative_eq!(cm.false_positive_rate(), 0.27586206896551724);
        assert_relative_eq!(cm.negative(), 42.);
        assert_relative_eq!(cm.negative_predictive_value(), 0.41935483870967744);
        assert_relative_eq!(cm.positive(), 58.);
        assert_relative_eq!(cm.positive_predictive_value(), 0.5789473684210527);
        assert_relative_eq!(cm.true_negative(), 26.);
        assert_relative_eq!(cm.true_negative_rate(), 0.6190476190476191);
        assert_relative_eq!(cm.true_positive(), 22.);
        assert_relative_eq!(cm.true_positive_rate(), 0.3793103448275862);
    }
}

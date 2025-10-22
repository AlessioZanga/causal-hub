#[cfg(test)]
mod tests {
    use causal_hub::{estimators::PK, labels};

    #[test]
    fn new() {
        // Initialize a list of labels.
        let labels = labels!["A", "B", "C"];
        // Set the forbidden edges.
        let forbidden = vec![(0, 1), (1, 2)];
        // Set the required edges.
        let required = vec![(0, 2)];
        // Set the temporal order.
        let temporal_order = vec![vec![0], vec![1, 2]];
        // Create a new instance of prior knowledge.
        let pk = PK::new(labels, forbidden, required, temporal_order);

        // Assert a single forbidden edge.
        assert!(pk.is_forbidden(0, 1));
        // Assert the forbidden edges.
        assert_eq!(pk.forbidden_edges(), &[(0, 1), (1, 0), (1, 2), (2, 0)]);
        // Assert a single required edge.
        assert!(pk.is_required(0, 2));
        // Assert the required edges.
        assert_eq!(pk.required_edges(), &[(0, 2)]);
    }
}

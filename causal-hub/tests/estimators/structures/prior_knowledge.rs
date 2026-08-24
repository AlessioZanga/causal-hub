#[cfg(test)]
mod tests {
    use causal_hub::{estimators::PK, labels, types::Result};

    #[test]
    fn new() -> Result<()> {
        // Initialize a list of labels.
        let labels = labels!["A", "B", "C"];
        // Set the forbidden edges.
        let forbidden = vec![(0, 1), (1, 2)];
        // Set the required edges.
        let required = vec![(0, 2)];
        // Set the temporal order.
        let temporal_order = vec![vec![0], vec![1, 2]];
        // Create a new instance of prior knowledge.
        let prior_knowledge = PK::new(labels, forbidden, required, temporal_order)?;

        // Assert a single forbidden edge.
        assert!(prior_knowledge.is_forbidden(0, 1));
        // Assert the forbidden edges.
        assert_eq!(
            prior_knowledge.forbidden_edges(),
            &[(0, 1), (1, 0), (1, 2), (2, 0)]
        );
        // Assert a single required edge.
        assert!(prior_knowledge.is_required(0, 2));
        // Assert the required edges.
        assert_eq!(prior_knowledge.required_edges(), &[(0, 2)]);

        Ok(())
    }
}

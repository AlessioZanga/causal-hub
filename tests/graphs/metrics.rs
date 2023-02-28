#[cfg(test)]
mod tests {
    use causal_hub::{graphs::metrics::structural_hamming_distance as shd, prelude::*};

    #[test]
    fn call() {
        // Initialize graphs.
        let true_graph = DiGraph::new(["A", "B", "C"], [("A", "B"), ("B", "C"), ("C", "C")]);
        let pred_graph = DiGraph::new(["A", "B", "C"], [("B", "A"), ("B", "C"), ("C", "A")]);

        assert_eq!(shd(&true_graph, &pred_graph), 3.);
    }
}

#[cfg(test)]
mod tests {
    use causal_hub_next::distribution::{CategoricalCPD, Distribution};
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let probabilities = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let categorical = CategoricalCPD::new(variables, probabilities.clone());

        assert!(categorical.labels().iter().eq(["A", "B", "C"]));
        assert!(
            categorical
                .states()
                .values()
                .all(|x| x.iter().eq(["no", "yes"]))
        );
        assert_eq!(categorical.parameters(), &probabilities);
    }

    #[test]
    #[should_panic(expected = "Variable labels must be unique.")]
    fn test_unique_labels() {
        let variables = vec![("A", vec!["no", "yes"]), ("A", vec!["no", "yes"])];
        let probabilities = array![[0.1, 0.9], [0.2, 0.8]];
        CategoricalCPD::new(variables, probabilities);
    }

    #[test]
    #[should_panic(expected = "Variable states must be unique.")]
    fn test_unique_states() {
        let variables = vec![("A", vec!["no", "no"]), ("B", vec!["no", "yes"])];
        let probabilities = array![[0.1, 0.9], [0.2, 0.8]];
        CategoricalCPD::new(variables, probabilities);
    }

    #[test]
    #[should_panic(expected = "Failed to sum probability to one: [].")]
    fn test_empty_labels() {
        let variables = vec![];
        let probabilities = array![[]];
        CategoricalCPD::new(variables, probabilities);
    }

    #[test]
    fn test_display() {
        let variables = vec![("A", vec!["no", "yes"]), ("B", vec!["no", "yes"])];
        let probabilities = array![[0.1, 0.9], [0.2, 0.8]];
        let categorical = CategoricalCPD::new(variables, probabilities);

        assert_eq!(
            categorical.to_string(),
            concat!(
                "----------------------------------\n",
                "|          | A        |          |\n",
                "| -------- | -------- | -------- |\n",
                "| B        | no       | yes      |\n",
                "| -------- | -------- | -------- |\n",
                "| no       | 0.100000 | 0.900000 |\n",
                "| yes      | 0.200000 | 0.800000 |\n",
                "----------------------------------\n",
            )
        );
    }
}

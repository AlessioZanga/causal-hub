#[cfg(test)]
mod tests {
    use causal_hub_next::dataset::{CategoricalDataset, Dataset};
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let variables = vec![
            ("B", vec!["no", "yes"]),
            ("C", vec!["yes", "no"]),
            ("A", vec!["no", "yes"]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CategoricalDataset::new(variables, values.clone());

        assert!(dataset.labels().iter().eq(["A", "B", "C"]));
        assert!(dataset.labels().iter().is_sorted());
        assert!(dataset.states().values().all(|x| x.iter().eq(["no", "yes"])));
        assert_eq!(
            dataset.values(),
            &array![
                [0, 0, 0], //
                [0, 0, 1], //
                [0, 1, 1], //
                [1, 1, 1]
            ]
        );
    }

    #[test]
    fn test_display() {
        let variables = vec![
            ("B", vec!["no", "yes"]),
            ("C", vec!["yes", "no"]),
            ("A", vec!["no", "yes"]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CategoricalDataset::new(variables, values);

        assert_eq!(
            dataset.to_string(),
            concat!(
                "-------------------\n",
                "| A   | B   | C   |\n",
                "| --- | --- | --- |\n",
                "| no  | no  | no  |\n",
                "| no  | no  | yes |\n",
                "| no  | yes | yes |\n",
                "| yes | yes | yes |\n",
                "-------------------\n",
            )
        );
    }
}

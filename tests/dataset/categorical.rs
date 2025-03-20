#[cfg(test)]
mod tests {
    use causal_hub_next::dataset::categorical::CategoricalDataset;
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];
        let categorical = CategoricalDataset::new(&variables, values.clone());

        assert!(categorical.labels().iter().eq(["A", "B", "C"]));
        assert!(categorical
            .states()
            .values()
            .all(|x| x.iter().eq(["no", "yes"])));
        assert_eq!(categorical.values(), &values);
    }

    #[test]
    fn test_display() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];
        let categorical = CategoricalDataset::new(&variables, values);

        assert_eq!(
            format!("{}", categorical),
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

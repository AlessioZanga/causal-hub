#[cfg(test)]
mod tests {
    use causal_hub_next::data::{CategoricalData, Data};
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];
        let data = CategoricalData::new(variables, values.clone());

        assert!(data.labels().iter().eq(["A", "B", "C"]));
        assert!(data.states().values().all(|x| x.iter().eq(["no", "yes"])));
        assert_eq!(data.values(), &values);
    }

    #[test]
    fn test_display() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];
        let data = CategoricalData::new(variables, values);

        assert_eq!(
            data.to_string(),
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

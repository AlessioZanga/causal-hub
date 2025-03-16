#[cfg(test)]
mod tests {
    use causal_hub_next::categorical::Categorical;
    use ndarray::array;

    #[test]
    fn test_get_probability() {
        let variables = vec![
            ("A".to_string(), vec!["a1".to_string(), "a2".to_string()]),
            ("B".to_string(), vec!["b1".to_string(), "b2".to_string()]),
        ];
        let probabilities = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let categorical = Categorical::new(variables, probabilities);

        assert_eq!(categorical.get_probability(vec![0, 0]), 0.1);
        assert_eq!(categorical.get_probability(vec![0, 1]), 0.9);
        assert_eq!(categorical.get_probability(vec![1, 0]), 0.2);
        assert_eq!(categorical.get_probability(vec![1, 1]), 0.8);
        assert_eq!(categorical.get_probability(vec![2, 0]), 0.3);
        assert_eq!(categorical.get_probability(vec![2, 1]), 0.7);
        assert_eq!(categorical.get_probability(vec![3, 0]), 0.4);
        assert_eq!(categorical.get_probability(vec![3, 1]), 0.6);
    }
}

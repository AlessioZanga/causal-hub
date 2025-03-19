#[cfg(test)]
mod tests {
    use causal_hub_next::categorical::Categorical;
    use ndarray::array;

    #[test]
    fn test_get_probability() {
        let variables = vec![
            ("A", vec!["no", "yes"]),
            ("B", vec!["no", "yes"]),
            ("C", vec!["no", "yes"]),
        ];
        let probabilities = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let categorical = Categorical::new(&variables, probabilities);
    }
}

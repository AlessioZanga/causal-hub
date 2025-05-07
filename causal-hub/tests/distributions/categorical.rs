#[cfg(test)]
mod tests {
    use causal_hub::distributions::{CPD, CategoricalCPD};
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let x = ("A", vec!["no", "yes"]);
        let z = vec![("B", vec!["no", "yes"]), ("C", vec!["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let categorical = CategoricalCPD::new(x, z, p.clone());

        assert_eq!(categorical.label(), "A");
        assert!(categorical.states().iter().eq(["no", "yes"]));
        assert!(categorical.conditioning_labels().iter().eq(["B", "C"]));
        assert!(
            categorical
                .conditioning_states()
                .values()
                .all(|x| x.iter().eq(["no", "yes"]))
        );
        assert_eq!(categorical.parameters(), &p);
    }

    #[test]
    #[should_panic(expected = "Conditioned variable cannot be a conditioning variable.")]
    fn test_unique_labels() {
        let x = ("A", vec!["no", "yes"]);
        let z = vec![("A", vec!["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        CategoricalCPD::new(x, z, p);
    }

    #[test]
    #[should_panic(expected = "Variables states must be unique.")]
    fn test_unique_states() {
        let x = ("A", vec!["no", "no"]);
        let z = vec![("B", vec!["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        CategoricalCPD::new(x, z, p);
    }

    #[test]
    #[should_panic(expected = "Failed to sum probability to one: [].")]
    fn test_empty_labels() {
        let x: (&str, Vec<&str>) = ("", vec![]);
        let z: Vec<(&str, Vec<&str>)> = vec![];
        let p = array![[]];
        CategoricalCPD::new(x, z, p);
    }

    #[test]
    fn test_display() {
        let x = ("A", vec!["no", "yes"]);
        let z = vec![("B", vec!["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        let categorical = CategoricalCPD::new(x, z, p);

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

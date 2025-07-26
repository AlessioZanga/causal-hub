#[cfg(test)]
mod tests {
    use causal_hub::{
        distributions::{CPD, CatCPD},
        map, set,
    };
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let s = set!["no".to_string(), "yes".to_string()];
        let x = map![("A".to_string(), s.clone())];
        let z = map![("B".to_string(), s.clone()), ("C".to_string(), s)];
        let p = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let categorical = CatCPD::new(x, z, p.clone());

        assert_eq!(categorical.labels()[0], "A");
        assert!(categorical.states()[0].iter().eq(["no", "yes"]));
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
        let s = set!["no".to_string(), "yes".to_string()];
        let x = map![("A".to_string(), s.clone())];
        let z = map![("A".to_string(), s.clone())];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        CatCPD::new(x, z, p);
    }

    #[test]
    #[should_panic(expected = "Variables states must be unique.")]
    fn test_unique_states() {
        let x = map![("A".to_string(), set!["no".to_string(), "no".to_string()])];
        let z = map![("A".to_string(), set!["no".to_string(), "yes".to_string()])];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        CatCPD::new(x, z, p);
    }

    #[test]
    #[should_panic(expected = "Failed to sum probability to one: [].")]
    fn test_empty_labels() {
        let x = map![];
        let z = map![];
        let p = array![[]];
        CatCPD::new(x, z, p);
    }

    #[test]
    fn test_display() {
        let s = set!["no".to_string(), "yes".to_string()];
        let x = map![("A".to_string(), s.clone())];
        let z = map![("B".to_string(), s.clone())];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        let categorical = CatCPD::new(x, z, p);

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

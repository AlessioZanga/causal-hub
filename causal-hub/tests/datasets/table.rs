#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTable, Dataset},
        map, set,
        types::Set,
    };
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("C".to_string(), set!["yes".to_string(), "no".to_string()]),
            ("A".to_string(), set!["no".to_string(), "yes".to_string()]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CatTable::new(states, values.clone());

        assert!(dataset.labels().iter().eq(["A", "B", "C"]));
        assert!(dataset.labels().iter().is_sorted());
        assert!(
            dataset
                .states()
                .values()
                .all(|x| x.iter().eq(["no", "yes"]))
        );
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
    fn test_new_different_states() {
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            (
                "C".to_string(),
                set!["yes".to_string(), "no".to_string(), "maybe".to_string()]
            ),
            ("A".to_string(), set!["no".to_string(), "yes".to_string()]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CatTable::new(states, values.clone());

        assert!(dataset.labels().iter().eq(["A", "B", "C"]));
        assert!(dataset.labels().iter().is_sorted());
        assert_eq!(
            dataset.values(),
            &array![
                [0, 0, 1], //
                [0, 0, 2], //
                [0, 1, 2], //
                [1, 1, 2]
            ]
        );
    }

    #[test]
    #[should_panic = "Number of variables must be equal to the number of columns: \n\t expected:    |states| == |values.columns()| , \n\t found:       |states| == 3 and |values.columns()| == 4 ."]
    fn test_new_non_unique_labels() {
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("C".to_string(), set!["yes".to_string(), "no".to_string()]),
            ("A".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("A".to_string(), set!["maybe".to_string()]), // 'A' is repeated
        ];
        let values = array![
            [0, 1, 0, 0], //
            [0, 0, 0, 1], //
            [1, 0, 0, 1], //
            [1, 0, 1, 0]
        ];
        CatTable::new(states, values);
    }

    #[test]
    #[should_panic = "Variable 'A' should have less than 256 states: \n\t expected:    |states| <  256 , \n\t found:       |states| == 256 ."]
    fn test_new_too_many_states() {
        let too_many_states: Vec<_> = (0..256).map(|i| i.to_string()).collect();
        let too_many_states: Set<_> = too_many_states.iter().map(|s| s.to_string()).collect();
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("C".to_string(), set!["yes".to_string(), "no".to_string()]),
            ("A".to_string(), too_many_states),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        CatTable::new(states, values);
    }

    #[test]
    #[should_panic = "Number of variables must be equal to the number of columns: \n\t expected:    |states| == |values.columns()| , \n\t found:       |states| == 3 and |values.columns()| == 2 ."]
    fn test_new_wrong_variables_and_columns() {
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("C".to_string(), set!["yes".to_string(), "no".to_string()]),
            ("A".to_string(), set!["no".to_string(), "yes".to_string()]),
        ];
        let values = array![
            [0, 1], //
            [0, 0], //
            [1, 0], //
            [1, 0]
        ];
        CatTable::new(states, values);
    }

    #[test]
    #[should_panic = "Values of variable 'A' must be less than the number of states: \n\t expected: values[.., 'A'] < |states['A']| , \n\t found:    values[.., 'A'] == 2 and |states['A']| == 2 ."]
    fn test_new_wrong_values() {
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("C".to_string(), set!["yes".to_string(), "no".to_string()]),
            ("A".to_string(), set!["no".to_string(), "yes".to_string()]),
        ];
        let values = array![
            [0, 1, 2], // 'A' has a value of 2 which is not valid
            [0, 0, 0],
            [1, 0, 0],
            [1, 0, 1]
        ];
        CatTable::new(states, values);
    }

    #[test]
    fn test_display() {
        let states = map![
            ("B".to_string(), set!["no".to_string(), "yes".to_string()]),
            ("C".to_string(), set!["yes".to_string(), "no".to_string()]),
            ("A".to_string(), set!["no".to_string(), "yes".to_string()]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CatTable::new(states, values);

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

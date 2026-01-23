#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTable, Dataset},
        labels,
        models::Labelled,
        states,
        types::{Result, Set},
    };
    use ndarray::prelude::*;

    #[test]
    fn new() -> Result<()> {
        let states = states![
            ("B", ["no", "yes"]),
            ("C", ["yes", "no"]),
            ("A", ["no", "yes"]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CatTable::new(states, values.clone())?;

        assert_eq!(dataset.labels(), &labels!["A", "B", "C"]);
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

        Ok(())
    }

    #[test]
    fn new_unordered_states() -> Result<()> {
        let states = states![
            ("B", ["no", "yes"]),
            ("C", ["yes", "no", "maybe"]),
            ("A", ["no", "yes"]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CatTable::new(states, values)?;

        assert_eq!(dataset.labels(), &labels!["A", "B", "C"]);
        assert_eq!(
            dataset.values(),
            &array![
                [0, 0, 1], //
                [0, 0, 2], //
                [0, 1, 2], //
                [1, 1, 2]
            ]
        );

        Ok(())
    }

    #[test]
    fn new_unordered_states_2() -> Result<()> {
        // Initialize the dataset.
        let dataset = CatTable::new(
            states![
                ("A", ["0", "1", "2", "3"]), //
                ("B", ["1", "2", "3", "0"]), //
            ],
            array![
                [0, 0], //
                [0, 1], //
                [1, 1], //
                [2, 1], //
                [2, 2]  //
            ],
        )?;

        // Check the labels.
        assert_eq!(dataset.labels(), &labels!["A", "B"]);
        // Check the states.
        assert_eq!(
            dataset.states(),
            &states![
                ("A", ["0", "1", "2", "3"]), //
                ("B", ["0", "1", "2", "3"]), //
            ]
        );
        // Check the events of the first trajectory.
        assert_eq!(
            dataset.values(),
            &array![
                [0, 1], //
                [0, 2], //
                [1, 2], //
                [2, 2], //
                [2, 3]  //
            ]
        );

        Ok(())
    }

    #[test]
    fn new_non_unique_labels() {
        let states = states![
            ("B", ["no", "yes"]),
            ("C", ["yes", "no"]),
            ("A", ["no", "yes"]),
            ("A", ["maybe"]), // 'A' is repeated
        ];
        let values = array![
            [0, 1, 0, 0], //
            [0, 0, 0, 1], //
            [1, 0, 0, 1], //
            [1, 0, 1, 0]
        ];
        assert!(CatTable::new(states, values).is_err());
    }

    #[test]
    fn new_too_many_states() {
        let too_many_states: Vec<_> = (0..256).map(|i| i.to_owned()).collect();
        let too_many_states: Set<_> = too_many_states.iter().map(|s| s.to_string()).collect();
        let mut states = states![("B", ["no", "yes"]), ("C", ["yes", "no"]),];
        states.insert("A".to_owned(), too_many_states);

        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        assert!(CatTable::new(states, values).is_err());
    }

    #[test]
    fn new_wrong_variables_and_columns() {
        let states = states![
            ("B", ["no", "yes"]),
            ("C", ["yes", "no"]),
            ("A", ["no", "yes"]),
        ];
        let values = array![
            [0, 1], //
            [0, 0], //
            [1, 0], //
            [1, 0]
        ];
        assert!(CatTable::new(states, values).is_err());
    }

    #[test]
    fn new_wrong_values() {
        let states = states![
            ("B", ["no", "yes"]),
            ("C", ["yes", "no"]),
            ("A", ["no", "yes"]),
        ];
        let values = array![
            [0, 1, 2], // 'A' has a value of 2 which is not valid
            [0, 0, 0],
            [1, 0, 0],
            [1, 0, 1]
        ];
        assert!(CatTable::new(states, values).is_err());
    }

    #[test]
    fn display() -> Result<()> {
        let states = states![
            ("B", ["no", "yes"]),
            ("C", ["yes", "no"]),
            ("A", ["no", "yes"]),
        ];
        let values = array![
            [0, 1, 0], //
            [0, 0, 0], //
            [1, 0, 0], //
            [1, 0, 1]
        ];
        let dataset = CatTable::new(states, values)?;

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

        Ok(())
    }
}

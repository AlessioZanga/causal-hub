#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTable, Dataset},
        labels,
        models::Labelled,
        states,
        types::Set,
    };
    use ndarray::prelude::*;

    mod categorical {
        use super::*;

        #[test]
        fn new() {
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
            let dataset = CatTable::new(states, values.clone());

            assert_eq!(&labels!["A", "B", "C"], dataset.labels());
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
        fn new_unordered_states() {
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
            let dataset = CatTable::new(states, values);

            assert_eq!(&labels!["A", "B", "C"], dataset.labels());
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
        fn new_unordered_states_2() {
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
            );

            // Check the labels.
            assert_eq!(&labels!["A", "B"], dataset.labels());
            // Check the states.
            assert_eq!(
                &states![
                    ("A", ["0", "1", "2", "3"]), //
                    ("B", ["0", "1", "2", "3"]), //
                ],
                dataset.states()
            );
            // Check the events of the first trajectory.
            assert_eq!(
                &array![
                    [0, 1], //
                    [0, 2], //
                    [1, 2], //
                    [2, 2], //
                    [2, 3]  //
                ],
                dataset.values()
            );
        }

        #[test]
        #[should_panic = "Number of variables must be equal to the number of columns: \n\t expected:    |states| == |values.columns()| , \n\t found:       |states| == 3 and |values.columns()| == 4 ."]
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
            CatTable::new(states, values);
        }

        #[test]
        #[should_panic = "Variable 'A' should have less than 256 states: \n\t expected:    |states| <  256 , \n\t found:       |states| == 256 ."]
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
            CatTable::new(states, values);
        }

        #[test]
        #[should_panic = "Number of variables must be equal to the number of columns: \n\t expected:    |states| == |values.columns()| , \n\t found:       |states| == 3 and |values.columns()| == 2 ."]
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
            CatTable::new(states, values);
        }

        #[test]
        #[should_panic = "Values of variable 'A' must be less than the number of states: \n\t expected: values[.., 'A'] < |states['A']| , \n\t found:    values[.., 'A'] == 2 and |states['A']| == 2 ."]
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
            CatTable::new(states, values);
        }

        #[test]
        fn display() {
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
}

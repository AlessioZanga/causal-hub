#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTable, Dataset, GaussTable},
        io::CsvIO,
        labels,
        models::Labelled,
    };
    use ndarray::prelude::*;

    mod table {
        use super::*;

        mod categorical {
            use super::*;

            #[test]
            fn from_csv_reader() {
                let csv = concat!(
                    "A,B,C\n",
                    "no,no,no\n",
                    "no,no,yes\n",
                    "no,yes,yes\n",
                    "yes,yes,yes"
                );
                let dataset = CatTable::from_csv_string(csv);

                let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];

                assert_eq!(&labels!["A", "B", "C"], dataset.labels());
                assert!(
                    dataset
                        .states()
                        .values()
                        .all(|x| x.iter().eq(["no", "yes"]))
                );
                assert_eq!(dataset.values(), &values);

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

            #[test]
            #[should_panic(expected = "Malformed record on line 2.")]
            fn from_csv_reader_malformed_record() {
                let csv = concat!(
                    "A,B,C\n",
                    "no,no,no\n",
                    "no,no\n",
                    "no,yes,yes\n",
                    "yes,yes,yes"
                );
                let _ = CatTable::from_csv_string(csv);
            }

            #[test]
            #[should_panic(expected = "Missing value on line 2.")]
            fn from_csv_reader_missing_value() {
                let csv = concat!(
                    "A,B,C\n",
                    "no,no,no\n",
                    "no,,no\n",
                    "no,yes,yes\n",
                    "yes,yes,yes"
                );
                let _ = CatTable::from_csv_string(csv);
            }

            #[test]
            fn to_csv_writer() {
                let csv = concat!(
                    "A,B,C\n",
                    "no,no,no\n",
                    "no,no,yes\n",
                    "no,yes,yes\n",
                    "yes,yes,yes\n"
                );
                let dataset = CatTable::from_csv_string(csv);

                // Create a buffer to write the CSV data.
                let mut buffer = Vec::new();
                dataset.to_csv_writer(&mut buffer);

                // Assert that the written CSV matches the original.
                assert_eq!(String::from_utf8(buffer).unwrap(), csv);
            }
        }

        mod gaussian {
            use super::*;

            #[test]
            fn from_csv_reader() {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,5,6\n", //
                    "7,8,9\n"
                );
                let dataset = GaussTable::from_csv_string(csv);

                let values = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

                assert_eq!(&labels!["X", "Y", "Z"], dataset.labels());
                assert_eq!(dataset.values(), &values);
            }

            #[test]
            #[should_panic(expected = "Malformed record on line 2.")]
            fn from_csv_reader_malformed_record() {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,5\n",   //
                    "7,8,9\n"
                );
                let _ = GaussTable::from_csv_string(csv);
            }

            #[test]
            #[should_panic(expected = "Missing value on line 2.")]
            fn from_csv_reader_missing_value() {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,,6\n",  //
                    "7,8,9\n"
                );
                let _ = GaussTable::from_csv_string(csv);
            }

            #[test]
            fn to_csv_writer() {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,5,6\n", //
                    "7,8,9\n"
                );
                let dataset = GaussTable::from_csv_string(csv);

                // Create a buffer to write the CSV data.
                let mut buffer = Vec::new();
                dataset.to_csv_writer(&mut buffer);

                // Assert that the written CSV matches the original.
                assert_eq!(String::from_utf8(buffer).unwrap(), csv);
            }
        }
    }
}

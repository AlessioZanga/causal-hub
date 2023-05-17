#[cfg(test)]
mod tests {
    mod discrete {
        use std::collections::BTreeMap;

        use causal_hub::prelude::*;
        use itertools::Itertools;
        use ndarray::prelude::*;
        use polars::prelude::*;

        #[test]
        fn from() {
            // Set in-memory sample data file.
            let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data = DiscreteDataMatrix::from(df);

            assert_eq!(
                data.values(),
                array![[0, 0, 0, 0], [1, 0, 1, 1], [2, 0, 0, 2]]
            );

            assert!(data.labels().into_iter().eq(&["W", "X", "Y", "Z"]));

            let states: BTreeMap<&str, FxIndexSet<&str>> = BTreeMap::from([
                ("W", vec!["I", "J", "K"].into_iter().collect()),
                ("X", vec!["A"].into_iter().collect()),
                ("Y", vec!["A", "B"].into_iter().collect()),
                ("Z", vec!["A", "B", "C"].into_iter().collect()),
            ]);

            assert!(data
                .states()
                .into_iter()
                .sorted_by(|a, b| a.0.cmp(b.0))
                .zip(states.into_iter())
                .all(|((x_k, x_v), (y_k, y_v))| x_k == y_k
                    && x_v.into_iter().zip(y_v.into_iter()).all(|(x, y)| x == y)));

            assert_eq!(data.cardinality(), &vec![3, 1, 2, 3]);
        }

        #[test]
        fn with_states() {
            // Set in-memory sample data file.
            let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data = DiscreteDataMatrix::from(df).with_states([
                ("X", vec!["A", "B"]),
                ("Y", vec!["A", "B", "C"]),
                ("W", vec!["G", "H", "I", "J", "K", "L", "M", "N"]),
            ]);

            assert_eq!(
                data.values(),
                array![[2, 0, 0, 0], [3, 0, 1, 1], [4, 0, 0, 2]]
            );

            assert!(data.labels().into_iter().eq(&["W", "X", "Y", "Z"]));

            let states: BTreeMap<&str, FxIndexSet<&str>> = BTreeMap::from([
                (
                    "W",
                    vec!["G", "H", "I", "J", "K", "L", "M", "N"]
                        .into_iter()
                        .collect(),
                ),
                ("X", vec!["A", "B"].into_iter().collect()),
                ("Y", vec!["A", "B", "C"].into_iter().collect()),
                ("Z", vec!["A", "B", "C"].into_iter().collect()),
            ]);

            assert!(data
                .states()
                .into_iter()
                .sorted_by(|a, b| a.0.cmp(b.0))
                .zip(states.into_iter())
                .all(|((x_k, x_v), (y_k, y_v))| x_k == y_k
                    && x_v.into_iter().zip(y_v.into_iter()).all(|(x, y)| x == y)));

            assert_eq!(data.cardinality(), &vec![8, 2, 3, 3]);
        }
    }

    mod continuous {
        use causal_hub::prelude::*;
        use ndarray::prelude::*;
        use polars::prelude::*;

        #[test]
        fn from() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\n1.0,1.0,1.0\n1.0,2.0,2.0\n1.0,1.0,3.0\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data = ContinuousDataMatrix::from(df);

            assert_eq!(
                data.values(),
                array![[1.0, 1.0, 1.0], [1.0, 2.0, 2.0], [1.0, 1.0, 3.0]]
            );

            assert!(data.labels().into_iter().eq(&["X", "Y", "Z"]));
        }
    }
}

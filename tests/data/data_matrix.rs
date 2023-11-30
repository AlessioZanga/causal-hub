#[cfg(test)]
mod tests {
    mod categorical {
        use std::collections::BTreeMap;

        use causal_hub::{polars::prelude::*, prelude::*};
        use itertools::Itertools;
        use ndarray::prelude::*;

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
            let data_set = CategoricalDataMatrix::from(df);

            assert_eq!(
                data_set.data(),
                array![[0, 0, 0, 0], [1, 0, 1, 1], [2, 0, 0, 2]]
            );

            assert!(data_set.labels_iter().into_iter().eq(["W", "X", "Y", "Z"]));

            let states: BTreeMap<&str, FxIndexSet<&str>> = BTreeMap::from([
                ("W", vec!["I", "J", "K"].into_iter().collect()),
                ("X", vec!["A"].into_iter().collect()),
                ("Y", vec!["A", "B"].into_iter().collect()),
                ("Z", vec!["A", "B", "C"].into_iter().collect()),
            ]);

            assert!(data_set
                .states()
                .into_iter()
                .sorted_by(|a, b| a.0.cmp(b.0))
                .zip(states.into_iter())
                .all(|((x_k, x_v), (y_k, y_v))| x_k == y_k
                    && x_v.into_iter().zip(y_v.into_iter()).all(|(x, y)| x == y)));

            assert_eq!(data_set.cardinality(), &vec![3, 1, 2, 3]);
        }

        #[test]
        fn into() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\nA,A,A\nA,B,B\nA,A,C\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let true_df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = CategoricalDataMatrix::from(true_df.clone());

            // Cast datamatrix to dataframe.
            let pred_df: DataFrame = data_set.into();

            assert_eq!(pred_df, true_df);
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
            let data_set = CategoricalDataMatrix::from(df).with_states([
                ("X", vec!["A", "B"]),
                ("Y", vec!["A", "B", "C"]),
                ("W", vec!["G", "H", "I", "J", "K", "L", "M", "N"]),
            ]);

            assert_eq!(
                data_set.data(),
                array![[2, 0, 0, 0], [3, 0, 1, 1], [4, 0, 0, 2]]
            );

            assert!(data_set.labels_iter().into_iter().eq(["W", "X", "Y", "Z"]));

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

            assert!(data_set
                .states()
                .into_iter()
                .sorted_by(|a, b| a.0.cmp(b.0))
                .zip(states.into_iter())
                .all(|((x_k, x_v), (y_k, y_v))| x_k == y_k
                    && x_v.into_iter().zip(y_v.into_iter()).all(|(x, y)| x == y)));

            assert_eq!(data_set.cardinality(), &vec![8, 2, 3, 3]);
        }

        #[test]
        fn sample() {
            // Set in-memory sample data file.
            let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = CategoricalDataMatrix::from(df);

            // Define random number generator.
            let mut rng = rand::thread_rng();
            // Sample from the data set.
            let sample = data_set.sample(&mut rng, 2);
            // Assert labels, states, cardinalities and values.
            assert!(data_set.labels_iter().eq(sample.labels_iter()));
            assert_eq!(data_set.states(), sample.states());
            assert_eq!(data_set.cardinality(), sample.cardinality());
            assert_eq!(data_set.data().ncols(), sample.data().ncols());
            assert!(data_set.sample_size() > sample.sample_size());
            assert_eq!(sample.sample_size(), 2);
        }

        #[test]
        #[should_panic]
        fn sample_should_panic() {
            // Set in-memory sample data file.
            let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = CategoricalDataMatrix::from(df);

            // Define random number generator.
            let mut rng = rand::thread_rng();
            // Sample from the data set.
            data_set.sample(&mut rng, 4);
        }

        #[test]
        fn sample_with_replacement() {
            // Set in-memory sample data file.
            let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = CategoricalDataMatrix::from(df);

            // Define random number generator.
            let mut rng = rand::thread_rng();
            // Sample from the data set.
            let sample = data_set.sample_with_replacement(&mut rng, 4);
            // Assert labels, states, cardinalities and values.
            assert!(data_set.labels_iter().eq(sample.labels_iter()));
            assert_eq!(data_set.states(), sample.states());
            assert_eq!(data_set.cardinality(), sample.cardinality());
            assert_eq!(data_set.data().ncols(), sample.data().ncols());
            assert!(data_set.sample_size() < sample.sample_size());
            assert_eq!(sample.sample_size(), 4);
        }
    }

    mod continuous {
        use causal_hub::{polars::prelude::*, prelude::*};
        use ndarray::prelude::*;

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
            let data_set = GaussianDataMatrix::from(df);

            assert_eq!(
                data_set.data(),
                array![[1.0, 1.0, 1.0], [1.0, 2.0, 2.0], [1.0, 1.0, 3.0]]
            );

            assert!(data_set.labels_iter().into_iter().eq(["X", "Y", "Z"]));
        }

        #[test]
        fn into() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\n1.0,1.0,1.0\n1.0,2.0,2.0\n1.0,1.0,3.0\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let true_df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = GaussianDataMatrix::from(true_df.clone());

            // Cast datamatrix to dataframe.
            let pred_df: DataFrame = data_set.into();

            assert_eq!(pred_df, true_df);
        }

        #[test]
        fn sample() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\n1.0,1.0,1.0\n1.0,2.0,2.0\n1.0,1.0,3.0\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = GaussianDataMatrix::from(df);

            // Define random number generator.
            let mut rng = rand::thread_rng();
            // Sample from the data set.
            let sample = data_set.sample(&mut rng, 2);
            // Assert labels and values.
            assert!(data_set.labels_iter().eq(sample.labels_iter()));
            assert_eq!(data_set.data().ncols(), sample.data().ncols());
            assert!(data_set.sample_size() > sample.sample_size());
            assert_eq!(sample.sample_size(), 2);
        }

        #[test]
        #[should_panic]
        fn sample_should_panic() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\n1.0,1.0,1.0\n1.0,2.0,2.0\n1.0,1.0,3.0\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = GaussianDataMatrix::from(df);

            // Define random number generator.
            let mut rng = rand::thread_rng();
            // Sample from the data set.
            data_set.sample(&mut rng, 4);
        }

        #[test]
        fn sample_with_replacement() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\n1.0,1.0,1.0\n1.0,2.0,2.0\n1.0,1.0,3.0\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file)
                .finish()
                .expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data_set = GaussianDataMatrix::from(df);

            // Define random number generator.
            let mut rng = rand::thread_rng();
            // Sample from the data set.
            let sample = data_set.sample_with_replacement(&mut rng, 4);
            // Assert labels and values.
            assert!(data_set.labels_iter().eq(sample.labels_iter()));
            assert_eq!(data_set.data().ncols(), sample.data().ncols());
            assert!(data_set.sample_size() < sample.sample_size());
            assert_eq!(sample.sample_size(), 4);
        }
    }
}

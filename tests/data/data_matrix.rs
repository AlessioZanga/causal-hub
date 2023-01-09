#[cfg(test)]
mod tests {
    mod discrete {
        use std::collections::BTreeMap;

        use causal_hub::data::DiscreteDataMatrix;

        use itertools::Itertools;
        use ndarray::prelude::*;
        use polars::prelude::*;

        #[test]
        fn from() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\nA,A,A\nA,B,B\nA,A,C\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file).finish().expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data = DiscreteDataMatrix::from(df);

            assert_eq!(*data, array![[0, 0, 0], [0, 1, 1], [0, 0, 2]]);

            assert!(data.labels().into_iter().eq(&["X", "Y", "Z"]));

            let levels: BTreeMap<String, Vec<String>> = BTreeMap::from([
                ("X".to_string(), vec!["A".to_string()]),
                ("Y".to_string(), vec!["A".to_string(), "B".to_string()]),
                ("Z".to_string(), vec!["A".to_string(), "B".to_string(), "C".to_string()]),
            ]);

            assert!(data.levels().into_iter().sorted_by(|a, b| a.0.cmp(b.0)).eq(&levels));

            assert_eq!(data.cardinality(), array![1, 2, 3]);
        }
    }

    mod continuous {
        use causal_hub::data::ContinuousDataMatrix;

        use ndarray::prelude::*;
        use polars::prelude::*;

        #[test]
        fn from() {
            // Set in-memory sample data file.
            let file = "X,Y,Z\n1.0,1.0,1.0\n1.0,2.0,2.0\n1.0,1.0,3.0\n";
            // Initialize an file cursor over the string.
            let file = std::io::Cursor::new(&file);
            // Parse the CSV file into a dataframe.
            let df = CsvReader::new(file).finish().expect("Failed to read from CSV file");
            // Cast dataframe to datamatrix.
            let data = ContinuousDataMatrix::from(df);

            assert_eq!(*data, array![[1.0, 1.0, 1.0], [1.0, 2.0, 2.0], [1.0, 1.0, 3.0]]);

            assert!(data.labels().into_iter().eq(&["X", "Y", "Z"]));
        }
    }
}

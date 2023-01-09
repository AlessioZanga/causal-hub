#[cfg(test)]
mod tests {
    use causal_hub::data::{DiscreteDataMatrix, MarginalCountMatrix, ConditionalCountMatrix, JointCountMatrix};

    use ndarray::prelude::*;
    use polars::prelude::*;

    #[test]
    fn marginal_count_matrix() {
        // Set in-memory sample data file.
        let file = "X,Y,Z\nA,A,A\nA,B,B\nA,A,C\n";
        // Initialize an file cursor over the string.
        let file = std::io::Cursor::new(&file);
        // Parse the CSV file into a dataframe.
        let df = CsvReader::new(file).finish().expect("Failed to read from CSV file");
        // Cast dataframe to datamatrix.
        let data = DiscreteDataMatrix::from(df);

        let count = MarginalCountMatrix::new(&data, 0);
        assert_eq!(*count, array![3]);

        let count = MarginalCountMatrix::new(&data, 1);
        assert_eq!(*count, array![2, 1]);

        let count = MarginalCountMatrix::new(&data, 2);
        assert_eq!(*count, array![1, 1, 1]);
    }

    #[test]
    fn conditional_count_matrix() {
        // Set in-memory sample data file.
        let file = "X,Y,Z\nA,A,A\nA,B,B\nA,A,C\n";
        // Initialize an file cursor over the string.
        let file = std::io::Cursor::new(&file);
        // Parse the CSV file into a dataframe.
        let df = CsvReader::new(file).finish().expect("Failed to read from CSV file");
        // Cast dataframe to datamatrix.
        let data = DiscreteDataMatrix::from(df);

        let count = ConditionalCountMatrix::new(&data, 0, vec![1]);
        assert_eq!(*count, array![[2], [1]]);

        let count = ConditionalCountMatrix::new(&data, 1, vec![0]);
        assert_eq!(*count, array![[2, 1]]);

        let count = ConditionalCountMatrix::new(&data, 2, vec![0]);
        assert_eq!(*count, array![[1, 1, 1]]);

        let count = ConditionalCountMatrix::new(&data, 0, vec![1, 2]);
        assert_eq!(*count, array![[1], [0], [1], [1], [0], [0]]);
    }

    #[test]
    fn joint_count_matrix() {
        // Set in-memory sample data file.
        let file = "X,Y,Z\nA,A,A\nA,B,B\nA,A,C\n";
        // Initialize an file cursor over the string.
        let file = std::io::Cursor::new(&file);
        // Parse the CSV file into a dataframe.
        let df = CsvReader::new(file).finish().expect("Failed to read from CSV file");
        // Cast dataframe to datamatrix.
        let data = DiscreteDataMatrix::from(df);

        let count = JointCountMatrix::new(&data, 0, 1, vec![2]);
        assert_eq!(*count, array![[[1, 0]], [[0, 1]], [[1, 0]]]);

        let count = JointCountMatrix::new(&data, 0, 2, vec![1]);
        assert_eq!(*count, array![[[1, 0, 1]], [[0, 1, 0]]]);
    }
}

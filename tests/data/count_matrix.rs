#[cfg(test)]
mod tests {
    use causal_hub::{polars::prelude::*, prelude::*};
    use ndarray::prelude::*;

    #[test]
    fn marginal_count_matrix() {
        // Set in-memory sample data file.
        let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
        // Initialize an file cursor over the string.
        let file = std::io::Cursor::new(&file);
        // Parse the CSV file into a dataframe.
        let d = CsvReader::new(file)
            .finish()
            .expect("Failed to read from CSV file");
        // Cast dataframe to datamatrix.
        let d = CategoricalDataMatrix::from(d);

        let n = MarginalCountMatrix::new(&d, 0);
        assert_eq!(n.values(), array![1, 1, 1]);

        let n = MarginalCountMatrix::new(&d, 1);
        assert_eq!(n.values(), array![3]);

        let n = MarginalCountMatrix::new(&d, 2);
        assert_eq!(n.values(), array![2, 1]);

        let n = MarginalCountMatrix::new(&d, 3);
        assert_eq!(n.values(), array![1, 1, 1]);
    }

    #[test]
    fn conditional_count_matrix() {
        // Set in-memory sample data file.
        let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
        // Initialize an file cursor over the string.
        let file = std::io::Cursor::new(&file);
        // Parse the CSV file into a dataframe.
        let d = CsvReader::new(file)
            .finish()
            .expect("Failed to read from CSV file");
        // Cast dataframe to datamatrix.
        let d = CategoricalDataMatrix::from(d);

        let n = ConditionalCountMatrix::new(&d, 1, &[2]);
        assert_eq!(n.values(), array![[2], [1]]);

        let n = ConditionalCountMatrix::new(&d, 2, &[1]);
        assert_eq!(n.values(), array![[2, 1]]);

        let n = ConditionalCountMatrix::new(&d, 3, &[1]);
        assert_eq!(n.values(), array![[1, 1, 1]]);

        let n = ConditionalCountMatrix::new(&d, 1, &[2, 3]);
        assert_eq!(n.values(), array![[1], [0], [1], [0], [1], [0]]);

        let n = ConditionalCountMatrix::new(&d, 0, &[1, 2, 3]);
        assert_eq!(
            n.values(),
            array![
                [1, 0, 0],
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 0],
                [0, 1, 0],
                [0, 0, 0]
            ]
        );
    }

    #[test]
    fn joint_count_matrix() {
        // Set in-memory sample data file.
        let file = "X,Y,Z,W\nA,A,A,I\nA,B,B,J\nA,A,C,K\n";
        // Initialize an file cursor over the string.
        let file = std::io::Cursor::new(&file);
        // Parse the CSV file into a dataframe.
        let d = CsvReader::new(file)
            .finish()
            .expect("Failed to read from CSV file");
        // Cast dataframe to datamatrix.
        let d = CategoricalDataMatrix::from(d);

        let n = JointConditionalCountMatrix::new(&d, 1, 2, &[3]);
        assert_eq!(n.values(), array![[[1, 0]], [[0, 1]], [[1, 0]]]);

        let n = JointConditionalCountMatrix::new(&d, 1, 3, &[2]);
        assert_eq!(n.values(), array![[[1, 0, 1]], [[0, 1, 0]]]);
    }
}

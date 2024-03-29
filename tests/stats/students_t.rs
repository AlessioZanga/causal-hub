#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn eval() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/students_t/gaussian.json").unwrap();
        let data: Vec<(String, String, Vec<String>, (usize, f64, f64))> =
            serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = GaussianDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels_iter());

        // Initialize conditional independence test.
        let test = StudentsT::from(&d);

        for (x, y, z, (true_dof, true_stat, true_pval)) in data {
            let x = g.get_vertex_index(&x);
            let y = g.get_vertex_index(&y);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            let (pred_dof, pred_stat, pred_pval) = test.eval(x, y, &z);

            assert_eq!(pred_dof, true_dof);
            assert_relative_eq!(pred_stat, true_stat, max_relative = 1e-8);
            assert_relative_eq!(pred_pval, true_pval, max_relative = 1e-8);
        }
    }

    #[test]
    fn call() {
        // Read test database from file.
        let data = std::fs::read_to_string("./tests/assets/students_t/gaussian.json").unwrap();
        let data: Vec<(String, String, Vec<String>, (usize, f64, f64))> =
            serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = GaussianDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels_iter());

        // Initialize conditional independence test.
        let test = StudentsT::from(&d);

        for (x, y, z, (_, _, true_pval)) in data {
            let x = g.get_vertex_index(&x);
            let y = g.get_vertex_index(&y);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            let pred_call = test.call(x, y, &z);

            assert_eq!(pred_call, true_pval > 0.05);
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call() {
        // Read test database from file.
        let data =
            std::fs::read_to_string("./tests/assets/partial_correlation/gaussian.json").unwrap();
        let data: Vec<(String, String, Vec<String>, f64)> = serde_json::from_str(&data).unwrap();

        // Load the data set from file.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .expect("Failed to read the data from file")
            .finish()
            .unwrap();
        let d = GaussianDataMatrix::from(d);

        // Build an empty the graph.
        let g = DiGraph::empty(d.labels());

        // Compute covariance matrix.
        let sigma = CovarianceMatrix::from(&d);
        // Initialize partial correlation.
        let pcorr = PartialCorrelation::from(sigma);

        for (x, y, z, r) in data {
            let x = g.get_vertex_index(&x);
            let y = g.get_vertex_index(&y);
            let z: Vec<_> = z.into_iter().map(|z| g.get_vertex_index(&z)).collect();

            assert_relative_eq!(pcorr.call(x, y, &z), r, max_relative = 1e-8);
        }
    }
}

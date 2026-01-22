#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatIncTable, GaussIncTable, GaussTable, IncDataset, MissingMethod},
        estimators::{CSSEstimator, SSE},
        labels,
        models::{CatCPDS, GaussCPDS, Labelled},
        set, states,
        types::Map,
    };
    use ndarray::prelude::*;

    mod categorical {
        use super::*;

        #[test]
        fn incomplete_ipw() {
            // Define the missing value.
            const M: u8 = CatIncTable::MISSING;

            // Define states.
            let states = states![("X", ["0", "1"]), ("Y", ["0", "1"])];

            // Define values (incomplete).
            let values = array![
                [0, 0],
                [1, 0],
                [0, 0],
                [M, 1], // Missing X
                [M, 1], // Missing X
                [1, 1],
            ]
            .mapv(|x| x as u8);

            // Create dataset.
            let d = CatIncTable::new(states, values);

            // Define missing mechanism. R_X (idx 0) depends on Y (idx 1). R_Y (idx 1) depends on nothing.
            let mut missing_mechanism = Map::default();
            missing_mechanism.insert(0, set![1]);
            missing_mechanism.insert(1, set![]);

            // Create estimator with IPW.
            let sse =
                SSE::new(&d).with_missing_method(Some(MissingMethod::IPW), Some(missing_mechanism));

            // Fit P(X | Y). X (0), Y (1).
            let x = set![0];
            let z = set![1];

            let cpd: CatCPDS = sse.fit(&x, &z);

            // Check sample size matches dataset samples
            assert_eq!(cpd.sample_size(), 4.0);

            // Check conditional counts.
            // IPW Weights:
            // (0, 0) -> R_X=1 => w=1.0. Count=2 (Rows 0, 2). N=2.0
            // (1, 0) -> R_X=1 => w=1.0. Count=1 (Row 1). N=1.0
            // (0, 1) -> R_X=1 => w=3.0. Count=0.
            // (1, 1) -> R_X=1 => w=3.0. Count=1 (Row 5). N=3.0
            //
            // N_XZ = [[1.6, 0.8], [0.0, 1.6]]
            // Indexing: Row=Z (Y=0, Y=1), Col=X (X=0, X=1)
            let expected = array![[1.6, 0.8], [0.0, 1.6]];
            assert_eq!(cpd.sample_conditional_counts(), &expected);
        }

        #[test]
        fn incomplete_aipw() {
            // Define the missing value.
            const M: u8 = CatIncTable::MISSING;

            // Define states.
            let states = states![("X", ["0", "1"]), ("Y", ["0", "1"])];

            // Define values (incomplete).
            let values = array![
                [0, 0],
                [1, 0],
                [0, 0],
                [M, 1], // Missing X
                [M, 1], // Missing X
                [1, 1],
            ]
            .mapv(|x| x as u8);

            // Create dataset.
            let d = CatIncTable::new(states, values);

            // Define missing mechanism. R_X (idx 0) depends on Y (idx 1). R_Y (idx 1) depends on nothing.
            let mut missing_mechanism = Map::default();
            missing_mechanism.insert(0, set![1]);
            missing_mechanism.insert(1, set![]);

            // Create estimator with AIPW.
            let sse = SSE::new(&d)
                .with_missing_method(Some(MissingMethod::AIPW), Some(missing_mechanism));

            let x = set![0];
            let z = set![1];

            let cpd: CatCPDS = sse.fit(&x, &z);

            assert_eq!(cpd.sample_size(), 4.0);

            // Check conditional counts for AIPW.
            // AIPW is more complex to calculate by hand, but we verify it produces non-zero
            // valid counts for observed combinations.
            let counts = cpd.sample_conditional_counts();
            assert!(counts[[0, 0]] > 0.0);
            assert!(counts[[0, 1]] > 0.0);
            assert_eq!(counts[[1, 0]], 0.0); // No observed samples for (0, 1)
            assert!(counts[[1, 1]] > 0.0);
        }
    }

    mod gaussian {
        use super::*;

        #[test]
        fn complete() {
            // Define values.
            let values = array![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
            let labels = labels!["X", "Y"];
            let d = GaussTable::new(labels, values);

            let sse = SSE::new(&d);

            // Fit P(X | Y).
            let x = set![0];
            let z = set![1];

            // Compute the sufficient statistics.
            let cpd: GaussCPDS = sse.fit(&x, &z);

            // Check sample size.
            assert_eq!(cpd.sample_size(), 4.0);

            // mu_X = (0+1+0+1)/4 = 0.5
            // mu_Y = (0+0+1+1)/4 = 0.5

            // Check means.
            assert_eq!(cpd.sample_response_mean(), &array![0.5]);
            assert_eq!(cpd.sample_design_mean(), &array![0.5]);
        }

        #[test]
        fn incomplete_pw() {
            // Define missing value.
            const M: f64 = f64::NAN;

            // Define values.
            // X, Y, W
            let values = array![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 1.0, M], // Missing W
                [1.0, 1.0, M], // Missing W
                [M, 0.0, 0.0], // Missing X
                [M, 1.0, 1.0]  // Missing X
            ];

            let labels = labels!["X", "Y", "W"];
            let d = GaussIncTable::new(labels, values);

            // PW Deletion.
            // Fit P(X | Y). X=0, Y=1.
            // Subset {X, Y}.
            // Rows with missing X or Y are removed.
            // Rows 4, 5 (Missing X) are removed.
            // Rows 0, 1, 2, 3 have valid X and Y.
            // Values:
            // [0, 0]
            // [1, 0]
            // [0, 1]
            // [1, 1]

            let sse = SSE::new(&d).with_missing_method(Some(MissingMethod::PW), None);

            let x_idx = d.labels().get_index_of("X").unwrap();
            let y_idx = d.labels().get_index_of("Y").unwrap();

            let x = set![x_idx];
            let z = set![y_idx];

            // Compute the sufficient statistics.
            let cpd: GaussCPDS = sse.fit(&x, &z);

            // Check sample size.
            assert_eq!(cpd.sample_size(), 4.0);

            // Check means.
            assert_eq!(cpd.sample_response_mean(), &array![0.5]);
            assert_eq!(cpd.sample_design_mean(), &array![0.5]);
        }
    }
}

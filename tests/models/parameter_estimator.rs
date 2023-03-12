#[cfg(test)]
mod maximum_likelihood_estimator {
    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call() {
        // Test cases.
        let data = [
            ("asia", array![0.9916, 0.0084].into_dyn()),
            (
                "bronc",
                array![
                    [0.7006036217303823, 0.2823061630218688],
                    [0.2993963782696177, 0.7176938369781312]
                ]
                .into_dyn(),
            ),
            (
                "dysp",
                array![
                    [
                        [0.9001728608470182, 0.21373056994818654],
                        [0.09982713915298184, 0.7862694300518135]
                    ],
                    [
                        [0.2773722627737226, 0.1459227467811159],
                        [0.7226277372262774, 0.8540772532188842]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "either",
                array![[[1., 0.], [0., 0.]], [[0., 1.], [1., 1.]]].into_dyn(),
            ),
            (
                "lung",
                array![
                    [0.986317907444668, 0.8823061630218688],
                    [0.013682092555331992, 0.11769383697813121]
                ]
                .into_dyn(),
            ),
            ("smoke", array![0.497, 0.503].into_dyn()),
            (
                "tub",
                array![
                    [0.991528842275111, 0.008471157724889069],
                    [0.9523809523809523, 0.047619047619047616]
                ]
                .into_dyn(),
            ),
            (
                "xray",
                array![
                    [0.9565874730021598, 0.04341252699784017],
                    [0.005405405405405406, 0.9945945945945946]
                ]
                .into_dyn(),
            ),
        ];

        // Read data.
        let d: DiscreteDataMatrix = CsvReader::from_path("tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap()
            .into();
        // Read Bayesian network.
        let b: DiscreteBayesianNetwork = BIF::read("tests/assets/bif/asia.bif").unwrap().into();

        // Fit Bayesian network given data and true graph.
        let c: DiscreteBayesianNetwork = MLE::call(&d, &b.graph());

        // Check reference and fitted BN have the same underlying graph.
        assert_eq!(b.graph(), c.graph());

        // Check fitted BN is fitted correctly.
        for ((x, phi), (y, psi)) in data.into_iter().zip(c.parameters().into_iter()) {
            // Assert same target variable.
            assert_eq!(x, y);
            // Assert same underlying values.
            assert_relative_eq!(phi, psi.values());
        }
    }
}

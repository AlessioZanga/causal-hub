#[cfg(test)]
mod maximum_likelihood_estimation {
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

#[cfg(test)]
mod bayesian_estimation {
    use approx::*;
    use causal_hub::prelude::*;
    use ndarray::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call() {
        // Test cases.
        let data = [
            (
                "asia",
                array![0.9914034386245502, 0.00859656137544982].into_dyn(),
            ),
            (
                "bronc",
                array![
                    [0.7004422999597909, 0.2824791418355185],
                    [0.2995577000402091, 0.7175208581644815]
                ]
                .into_dyn(),
            ),
            (
                "dysp",
                array![
                    [
                        [0.8998272884283247, 0.21397756686798963],
                        [0.1001727115716753, 0.7860224331320104]
                    ],
                    [
                        [0.2805755395683453, 0.14893617021276595],
                        [0.7194244604316546, 0.851063829787234]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "either",
                array![
                    [
                        [0.9997841105354058, 0.003048780487804878],
                        [0.023809523809523808, 0.16666666666666666]
                    ],
                    [
                        [0.0002158894645941278, 0.9969512195121951],
                        [0.9761904761904762, 0.8333333333333334]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "lung",
                array![
                    [0.9859268194611982, 0.8820023837902264],
                    [0.01407318053880177, 0.11799761620977355]
                ]
                .into_dyn(),
            ),
            (
                "smoke",
                array![0.49700119952019195, 0.502998800479808].into_dyn(),
            ),
            (
                "tub",
                array![
                    [0.9913306451612903, 0.008669354838709677],
                    [0.9318181818181818, 0.06818181818181818]
                ]
                .into_dyn(),
            ),
            (
                "xray",
                array![
                    [0.9563903281519862, 0.043609671848013815],
                    [0.008064516129032258, 0.9919354838709677]
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
        let c: DiscreteBayesianNetwork = BE::call(&d, &b.graph());

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

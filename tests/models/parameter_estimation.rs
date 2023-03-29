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

    #[test]
    fn parallel_call() {
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
        let c: DiscreteBayesianNetwork = ParallelMLE::call(&d, &b.graph());

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
                array![0.991501699660068, 0.008498300339932013].into_dyn(),
            ),
            (
                "bronc",
                array![
                    [0.7005632669483002, 0.2823494335122242],
                    [0.2994367330516999, 0.7176505664877758]
                ]
                .into_dyn(),
            ),
            (
                "dysp",
                array![
                    [
                        [0.9001296316301177, 0.21376146788990827],
                        [0.09987036836988225, 0.7862385321100918]
                    ],
                    [
                        [0.2777777777777778, 0.14630225080385853],
                        [0.7222222222222222, 0.8536977491961415]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "either",
                array![
                    [
                        [0.9999730036175153, 0.0003831417624521073],
                        [0.003105590062111801, 0.029411764705882353]
                    ],
                    [
                        [2.6996382484747044e-5, 0.9996168582375479],
                        [0.9968944099378882, 0.9705882352941176]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "lung",
                array![
                    [0.9862200764433715, 0.8822301729278473],
                    [0.013779923556628444, 0.11776982707215265]
                ]
                .into_dyn(),
            ),
            (
                "smoke",
                array![0.497000599880024, 0.502999400119976].into_dyn(),
            ),
            (
                "tub",
                array![
                    [0.9914792780074619, 0.008520721992538066],
                    [0.9470588235294117, 0.052941176470588235]
                ]
                .into_dyn(),
            ),
            (
                "xray",
                array![
                    [0.9565381708238851, 0.04346182917611489],
                    [0.006072874493927126, 0.9939271255060729]
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

    #[test]
    fn parallel_call() {
        // Test cases.
        let data = [
            (
                "asia",
                array![0.991501699660068, 0.008498300339932013].into_dyn(),
            ),
            (
                "bronc",
                array![
                    [0.7005632669483002, 0.2823494335122242],
                    [0.2994367330516999, 0.7176505664877758]
                ]
                .into_dyn(),
            ),
            (
                "dysp",
                array![
                    [
                        [0.9001296316301177, 0.21376146788990827],
                        [0.09987036836988225, 0.7862385321100918]
                    ],
                    [
                        [0.2777777777777778, 0.14630225080385853],
                        [0.7222222222222222, 0.8536977491961415]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "either",
                array![
                    [
                        [0.9999730036175153, 0.0003831417624521073],
                        [0.003105590062111801, 0.029411764705882353]
                    ],
                    [
                        [2.6996382484747044e-5, 0.9996168582375479],
                        [0.9968944099378882, 0.9705882352941176]
                    ]
                ]
                .into_dyn(),
            ),
            (
                "lung",
                array![
                    [0.9862200764433715, 0.8822301729278473],
                    [0.013779923556628444, 0.11776982707215265]
                ]
                .into_dyn(),
            ),
            (
                "smoke",
                array![0.497000599880024, 0.502999400119976].into_dyn(),
            ),
            (
                "tub",
                array![
                    [0.9914792780074619, 0.008520721992538066],
                    [0.9470588235294117, 0.052941176470588235]
                ]
                .into_dyn(),
            ),
            (
                "xray",
                array![
                    [0.9565381708238851, 0.04346182917611489],
                    [0.006072874493927126, 0.9939271255060729]
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
        let c: DiscreteBayesianNetwork = ParallelBE::call(&d, &b.graph());

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

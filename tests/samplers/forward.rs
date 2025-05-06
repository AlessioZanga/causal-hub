#[cfg(test)]
mod tests {
    mod bayesian_network {
        use approx::assert_relative_eq;
        use causal_hub_next::{
            assets::{load_cancer, load_child},
            datasets::Dataset,
            distributions::CPD,
            estimators::{BNEstimator, MLE},
            models::{BayesianNetwork, CategoricalBN},
            samplers::{BNSampler, ForwardSampler},
        };
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        #[test]
        fn test_forward_sampling() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Load BN.
            let bn = load_child();
            // Initialize sampler.
            let mut sampler = ForwardSampler::new(&mut rng, &bn);
            // Sample from BN.
            let dataset = sampler.sample_n(10);

            // Check labels.
            assert!(dataset.labels().eq(bn.labels()));
            // Check sample size.
            assert_eq!(dataset.sample_size(), 10);

            // Check dataset layout.
            assert_eq!(
                dataset.to_string(),
                concat!(
                    "-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------\n",
                    "| Age            | BirthAsphyxia  | CO2            | CO2Report      | CardiacMixing  | ChestXray      | Disease        | DuctFlow       | Grunting       | GruntingReport | HypDistrib     | HypoxiaInO2    | LVH            | LVHreport      | LowerBodyO2    | LungFlow       | LungParench    | RUQO2          | Sick           | XrayReport     |\n",
                    "| -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- |\n",
                    "| 0-3_days       | yes            | High           | >=7.5          | Transp.        | Plethoric      | Lung           | Rt_to_Lt       | yes            | no             | Equal          | Severe         | yes            | yes            | 12+            | High           | Congested      | 5-12           | no             | Normal         |\n",
                    "| 0-3_days       | yes            | High           | >=7.5          | Mild           | Oligaemic      | PFC            | Rt_to_Lt       | yes            | no             | Equal          | Moderate       | yes            | no             | 5-12           | High           | Abnormal       | 5-12           | yes            | Grd_Glass      |\n",
                    "| 11-30_days     | yes            | Normal         | <7.5           | None           | Oligaemic      | Fallot         | Lt_to_Rt       | no             | no             | Equal          | Mild           | no             | yes            | 5-12           | Low            | Abnormal       | <5             | yes            | Grd_Glass      |\n",
                    "| 0-3_days       | yes            | High           | >=7.5          | Transp.        | Oligaemic      | TGA            | Rt_to_Lt       | no             | yes            | Equal          | Severe         | yes            | no             | 12+            | Normal         | Abnormal       | <5             | yes            | Asy/Patchy     |\n",
                    "| 0-3_days       | yes            | Normal         | <7.5           | Complete       | Normal         | Lung           | None           | yes            | no             | Equal          | Moderate       | no             | yes            | 5-12           | High           | Normal         | 12+            | no             | Asy/Patchy     |\n",
                    "| 0-3_days       | yes            | High           | >=7.5          | Complete       | Plethoric      | Lung           | None           | yes            | no             | Equal          | Severe         | yes            | no             | <5             | High           | Normal         | 5-12           | yes            | Normal         |\n",
                    "| 11-30_days     | yes            | High           | >=7.5          | Mild           | Asy/Patch      | Lung           | Rt_to_Lt       | yes            | no             | Equal          | Moderate       | yes            | no             | 5-12           | Normal         | Normal         | <5             | no             | Normal         |\n",
                    "| 0-3_days       | yes            | High           | >=7.5          | Complete       | Plethoric      | Lung           | Lt_to_Rt       | yes            | no             | Equal          | Moderate       | yes            | no             | 12+            | High           | Normal         | 5-12           | no             | Normal         |\n",
                    "| 11-30_days     | yes            | Low            | <7.5           | Complete       | Plethoric      | Lung           | None           | yes            | no             | Equal          | Moderate       | yes            | no             | 5-12           | High           | Normal         | 5-12           | no             | Normal         |\n",
                    "| 0-3_days       | yes            | Low            | <7.5           | Mild           | Plethoric      | Lung           | None           | no             | yes            | Equal          | Severe         | no             | yes            | 12+            | High           | Congested      | 12+            | no             | Normal         |\n",
                    "-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------\n"
                )
            );
        }

        #[test]
        fn test_forward_sampling_refit() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Load BN.
            let bn = load_cancer();
            // Initialize sampler.
            let mut sampler = ForwardSampler::new(&mut rng, &bn);
            // Sample from BN.
            let dataset = sampler.sample_n(150_000);

            // Initialize estimator.
            let estimator = MLE::new(&dataset);
            // Fit with generated dataset.
            let fitted_bn: CategoricalBN = estimator.fit(bn.graph().clone());

            // Check fitted CDPs.
            for ((_, cpd), (_, fitted_cpd)) in bn.cpds().iter().zip(fitted_bn.cpds()) {
                // Check values.
                assert_relative_eq!(cpd.parameters(), fitted_cpd.parameters(), epsilon = 1e-2);
            }
        }
    }

    mod continuous_time_bayesian_network {
        use std::cell::LazyCell;

        use approx::assert_relative_eq;
        use causal_hub_next::{
            datasets::Dataset,
            distributions::{CPD, CategoricalCIM},
            estimators::{CTBNEstimator, MLE},
            graphs::{DiGraph, Graph},
            models::{CategoricalCTBN, ContinuousTimeBayesianNetwork},
            samplers::{CTBNSampler, ForwardSampler, ParCTBNSampler},
        };
        use ndarray::prelude::*;
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        const EATING: LazyCell<CategoricalCTBN> = LazyCell::new(|| {
            // Initialize the graph.
            let mut graph = DiGraph::empty(vec!["Hungry", "Eating", "FullStomach"]);
            graph.add_edge(0, 1); // Hungry -> Eating
            graph.add_edge(1, 2); // Eating -> FullStomach
            graph.add_edge(2, 0); // FullStomach -> Hungry

            // Initialize the distributions.
            let cims = vec![
                CategoricalCIM::new(
                    // P(Hungry | FullStomach)
                    ("Hungry", vec!["no", "yes"]),
                    [("FullStomach", vec!["no", "yes"])],
                    array![
                        [
                            [-0.1, 0.1], //
                            [10., -10.]  //
                        ],
                        [
                            [-2., 2.],   //
                            [0.1, -0.1]  //
                        ],
                    ],
                ),
                CategoricalCIM::new(
                    // P(Eating | Hungry)
                    ("Eating", vec!["no", "yes"]),
                    [("Hungry", vec!["no", "yes"])],
                    array![
                        [
                            [-0.1, 0.1], //
                            [10., -10.]  //
                        ],
                        [
                            [-2., 2.],   //
                            [0.1, -0.1]  //
                        ],
                    ],
                ),
                CategoricalCIM::new(
                    // P(FullStomach | Eating)
                    ("FullStomach", vec!["no", "yes"]),
                    [("Eating", vec!["no", "yes"])],
                    array![
                        [
                            [-0.1, 0.1], //
                            [10., -10.]  //
                        ],
                        [
                            [-2., 2.],   //
                            [0.1, -0.1]  //
                        ],
                    ],
                ),
            ];

            // Initialize the model.
            let ctbn = CategoricalCTBN::new(graph.clone(), cims.clone());

            ctbn
        });

        #[test]
        fn test_forward_sampling_by_length() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the model.
            let ctbn = EATING.clone();
            // Initialize sampler.
            let mut sampler = ForwardSampler::new(&mut rng, &ctbn);
            // Sample from CTBN.
            let trajectory = sampler.sample_by_length(10);

            // Check labels.
            assert!(trajectory.labels().eq(ctbn.labels()));
            // Check sample size.
            assert_eq!(trajectory.sample_size(), 10);
        }

        #[test]
        fn test_forward_sampling_by_time() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the model.
            let ctbn = EATING.clone();
            // Initialize sampler.
            let mut sampler = ForwardSampler::new(&mut rng, &ctbn);
            // Sample from CTBN.
            let trajectory = sampler.sample_by_time(100.);

            // Check labels.
            assert!(trajectory.labels().eq(ctbn.labels()));
            // Check sample size.
            assert!(*trajectory.times().iter().last().unwrap() < 100.);
        }

        #[test]
        fn test_forward_sampling_refit() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the model.
            let ctbn = EATING.clone();
            // Initialize sampler.
            let mut sampler = ForwardSampler::new(&mut rng, &ctbn);
            // Sample from CTBN.
            let trajectory = sampler.par_sample_n_by_length(1000, 1000);

            // Initialize estimator.
            let estimator = MLE::new(&trajectory);
            // Fit with generated dataset.
            let fitted_ctbn: CategoricalCTBN = estimator.fit(ctbn.graph().clone());

            // Check fitted CIMs.
            for ((_, cim), (_, fitted_cim)) in ctbn.cims().iter().zip(fitted_ctbn.cims()) {
                // Check values.
                assert_relative_eq!(cim.parameters(), fitted_cim.parameters(), epsilon = 5e-2);
            }
        }
    }
}

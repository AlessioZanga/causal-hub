#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        assets::*,
        datasets::Dataset,
        estimators::{BNEstimator, MLE, ParCTBNEstimator},
        models::{BN, CTBN, CatBN, CatCTBN, GaussBN, Labelled},
        samplers::{BNSampler, CTBNSampler, ForwardSampler, ParCTBNSampler},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    mod bayesian_network {
        use super::*;

        mod categorical {
            use super::*;

            #[test]
            fn forward_sampling() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Load BN.
                let model = load_child();
                // Initialize sampler.
                let forward = ForwardSampler::new(&mut rng, &model);
                // Sample from BN.
                let dataset = forward.sample_n(10);

                // Check labels.
                assert!(dataset.labels().eq(model.labels()));
                // Check sample size.
                assert_eq!(dataset.sample_size(), 10.);

                // Check dataset layout.
                assert_eq!(
                    dataset.to_string(),
                    concat!(
                        "-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------\n",
                        "| Age            | BirthAsphyxia  | CO2            | CO2Report      | CardiacMixing  | ChestXray      | Disease        | DuctFlow       | Grunting       | GruntingReport | HypDistrib     | HypoxiaInO2    | LVH            | LVHreport      | LowerBodyO2    | LungFlow       | LungParench    | RUQO2          | Sick           | XrayReport     |\n",
                        "| -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- |\n",
                        "| 0-3_days       | no             | Normal         | <7.5           | Transp.        | Normal         | Lung           | Rt_to_Lt       | yes            | yes            | Equal          | Severe         | no             | no             | 12+            | Normal         | Abnormal       | <5             | yes            | Normal         |\n",
                        "| 0-3_days       | no             | High           | >=7.5          | Transp.        | Plethoric      | TGA            | None           | yes            | yes            | Equal          | Severe         | no             | no             | 5-12           | High           | Normal         | <5             | yes            | Plethoric      |\n",
                        "| 4-10_days      | no             | Normal         | <7.5           | Complete       | Oligaemic      | Fallot         | Lt_to_Rt       | no             | no             | Equal          | Moderate       | no             | no             | 5-12           | Low            | Normal         | 5-12           | no             | Oligaemic      |\n",
                        "| 0-3_days       | no             | High           | >=7.5          | Transp.        | Plethoric      | TGA            | Rt_to_Lt       | no             | no             | Equal          | Severe         | yes            | no             | 5-12           | High           | Normal         | <5             | yes            | Asy/Patchy     |\n",
                        "| 0-3_days       | no             | Normal         | <7.5           | Complete       | Normal         | PAIVS          | Lt_to_Rt       | no             | no             | Equal          | Moderate       | no             | no             | <5             | Low            | Normal         | 5-12           | no             | Normal         |\n",
                        "| 0-3_days       | no             | Normal         | <7.5           | Complete       | Oligaemic      | PAIVS          | Lt_to_Rt       | no             | no             | Equal          | Severe         | yes            | yes            | <5             | Low            | Normal         | <5             | yes            | Oligaemic      |\n",
                        "| 4-10_days      | no             | Normal         | <7.5           | Complete       | Normal         | PAIVS          | Lt_to_Rt       | no             | no             | Equal          | Moderate       | yes            | yes            | 5-12           | Normal         | Normal         | <5             | no             | Normal         |\n",
                        "| 11-30_days     | no             | Normal         | <7.5           | Complete       | Oligaemic      | Fallot         | Lt_to_Rt       | no             | no             | Equal          | Moderate       | no             | no             | 12+            | Low            | Normal         | 5-12           | no             | Oligaemic      |\n",
                        "| 4-10_days      | no             | Normal         | <7.5           | Complete       | Oligaemic      | Fallot         | Lt_to_Rt       | no             | no             | Equal          | Moderate       | no             | no             | <5             | Low            | Normal         | 5-12           | no             | Oligaemic      |\n",
                        "| 0-3_days       | no             | Normal         | <7.5           | Complete       | Asy/Patch      | PAIVS          | Lt_to_Rt       | no             | no             | Equal          | Severe         | no             | no             | 5-12           | Low            | Abnormal       | 5-12           | no             | Asy/Patchy     |\n",
                        "-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------\n"
                    )
                );
            }

            #[test]
            fn forward_sampling_refit() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Load BN.
                let model = load_cancer();
                // Initialize sampler.
                let forward = ForwardSampler::new(&mut rng, &model);
                // Sample from BN.
                let dataset = forward.sample_n(150_000);

                // Initialize estimator.
                let estimator = MLE::new(&dataset);
                // Fit with generated dataset.
                let fitted_model: CatBN = estimator.fit(model.graph().clone());

                // Check fitted CDPs.
                assert_relative_eq!(model, fitted_model, epsilon = 1e-2);
            }
        }

        mod gaussian {
            use super::*;

            #[test]
            fn forward_sampling_refit() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Load BN.
                let model = load_ecoli70();
                // Initialize sampler.
                let forward = ForwardSampler::new(&mut rng, &model);
                // Sample from BN.
                let dataset = forward.sample_n(5_000);

                // Initialize estimator.
                let estimator = MLE::new(&dataset);
                // Fit with generated dataset.
                let fitted_model: GaussBN = estimator.fit(model.graph().clone());

                // Check fitted CDPs.
                assert_relative_eq!(model, fitted_model, epsilon = 1e-1);
            }
        }
    }

    mod continuous_time_bayesian_network {
        use super::*;

        mod categorical {
            use super::*;

            #[test]
            fn forward_sampling_by_length() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_eating();
                // Initialize sampler.
                let forward = ForwardSampler::new(&mut rng, &model);
                // Sample from CTBN.
                let trajectory = forward.sample_by_length(10);

                // Check labels.
                assert!(trajectory.labels().eq(model.labels()));
                // Check sample size.
                assert_eq!(trajectory.sample_size(), 10.);
            }

            #[test]
            fn forward_sampling_by_time() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_eating();
                // Initialize sampler.
                let forward = ForwardSampler::new(&mut rng, &model);
                // Sample from CTBN.
                let trajectory = forward.sample_by_time(100.);

                // Check labels.
                assert!(trajectory.labels().eq(model.labels()));
                // Check sample size.
                assert!(*trajectory.times().iter().last().unwrap() < 100.);
            }

            #[test]
            fn forward_sampling_refit() {
                // Initialize RNG.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Initialize the model.
                let model = load_eating();
                // Initialize sampler.
                let forward = ForwardSampler::new(&mut rng, &model);
                // Sample from CTBN.
                let trajectory = forward.par_sample_n_by_length(1_000, 1_000);

                // Initialize estimator.
                let estimator = MLE::new(&trajectory);
                // Fit with generated dataset.
                let fitted_model: CatCTBN = estimator.par_fit(model.graph().clone());

                // Check fitted CIMs.
                assert_relative_eq!(model, fitted_model, epsilon = 5e-2);
            }
        }
    }
}

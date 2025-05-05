#[cfg(test)]
mod tests {
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

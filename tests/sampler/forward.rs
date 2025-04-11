#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub_next::{
        assets::{load_cancer, load_child},
        data::Data,
        distribution::CPD,
        estimator::{BNEstimator, MLE},
        model::{BayesianNetwork, CategoricalBN},
        sampler::{BNSampler, ForwardSampler},
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
        let sampler = ForwardSampler::new();
        // Sample from BN.
        let data = sampler.sample(&mut rng, &bn, 10);

        // Check labels.
        assert!(data.labels().eq(bn.labels()));
        // Check sample size.
        assert_eq!(data.sample_size(), 10);

        // Check data layout.
        assert_eq!(
            data.to_string(),
            concat!(
                "-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------\n",
                "| Age            | BirthAsphyxia  | CO2            | CO2Report      | CardiacMixing  | ChestXray      | Disease        | DuctFlow       | Grunting       | GruntingReport | HypDistrib     | HypoxiaInO2    | LVH            | LVHreport      | LowerBodyO2    | LungFlow       | LungParench    | RUQO2          | Sick           | XrayReport     |\n",
                "| -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- | -------------- |\n",
                "| 0-3_days       | yes            | Normal         | <7.5           | None           | Oligaemic      | PAIVS          | Lt_to_Rt       | yes            | no             | Equal          | Mild           | no             | yes            | <5             | Low            | Abnormal       | 5-12           | no             | Grd_Glass      |\n",
                "| 0-3_days       | yes            | High           | >=7.5          | None           | Oligaemic      | PFC            | Rt_to_Lt       | yes            | no             | Equal          | Mild           | yes            | no             | <5             | Low            | Normal         | <5             | yes            | Grd_Glass      |\n",
                "| 0-3_days       | yes            | High           | >=7.5          | None           | Plethoric      | PAIVS          | Lt_to_Rt       | no             | yes            | Equal          | Moderate       | no             | yes            | 12+            | Low            | Congested      | <5             | yes            | Normal         |\n",
                "| 4-10_days      | yes            | High           | >=7.5          | None           | Oligaemic      | Fallot         | Lt_to_Rt       | yes            | no             | Equal          | Mild           | yes            | no             | <5             | Low            | Normal         | <5             | no             | Grd_Glass      |\n",
                "| 0-3_days       | yes            | Normal         | <7.5           | Mild           | Plethoric      | Lung           | Rt_to_Lt       | yes            | no             | Equal          | Severe         | no             | yes            | <5             | High           | Normal         | 12+            | no             | Normal         |\n",
                "| 0-3_days       | yes            | High           | >=7.5          | None           | Normal         | PAIVS          | Lt_to_Rt       | no             | yes            | Equal          | Severe         | no             | yes            | 12+            | Low            | Abnormal       | 5-12           | yes            | Asy/Patchy     |\n",
                "| 0-3_days       | yes            | Low            | <7.5           | Mild           | Oligaemic      | Lung           | Rt_to_Lt       | yes            | no             | Equal          | Moderate       | yes            | no             | 5-12           | Low            | Normal         | 12+            | no             | Grd_Glass      |\n",
                "| 0-3_days       | yes            | Normal         | <7.5           | None           | Plethoric      | Fallot         | Lt_to_Rt       | no             | yes            | Equal          | Moderate       | yes            | no             | 5-12           | Low            | Congested      | 5-12           | yes            | Normal         |\n",
                "| 11-30_days     | yes            | Normal         | <7.5           | None           | Oligaemic      | PAIVS          | Lt_to_Rt       | no             | yes            | Equal          | Severe         | no             | yes            | 12+            | Low            | Abnormal       | 5-12           | yes            | Grd_Glass      |\n",
                "| 0-3_days       | yes            | High           | <7.5           | Mild           | Grd_Glass      | Lung           | Rt_to_Lt       | yes            | no             | Equal          | Moderate       | yes            | no             | 12+            | High           | Abnormal       | 5-12           | no             | Normal         |\n",
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
        let sampler = ForwardSampler::new();
        // Sample from BN.
        let data = sampler.sample(&mut rng, &bn, 100_000);

        // Initialize estimator.
        let estimator = MLE::new();
        // Fit with generated data.
        let fitted_bn: CategoricalBN = estimator.fit(&data, bn.graph().clone());

        // Check fitted CDPs.
        for ((_, cpd), (_, fitted_cpd)) in bn.cpds().iter().zip(fitted_bn.cpds()) {
            // Check values.
            assert_relative_eq!(
                cpd.parameters(),
                fitted_cpd.parameters(),
                epsilon = 1e-2
            );
        }
    }
}

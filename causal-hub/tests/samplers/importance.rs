#[cfg(test)]
mod tests {
    mod bayesian_network {}

    mod continuous_time_bayesian_network {
        use causal_hub::{
            assets::load_eating,
            datasets::{CategoricalTrjEv, CategoricalTrjEvT as E, Dataset},
            models::CTBN,
            samplers::ImportanceSampler,
        };
        use ndarray::prelude::*;
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        #[test]
        fn test_forward_sampling_by_length() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the model.
            let ctbn = load_eating();

            // Initialize evidence.
            let evidence = CategoricalTrjEv::new(
                ctbn.states(),
                [
                    (
                        "Hungry",
                        E::CertainPositiveInterval {
                            state: 1,
                            start_time: 0.,
                            end_time: 0.2,
                        },
                    ),
                    (
                        "Eating",
                        E::CertainNegativeInterval {
                            not_states: [0].into_iter().collect(),
                            start_time: 0.,
                            end_time: 0.2,
                        },
                    ),
                    (
                        "FullStomach",
                        E::UncertainPositiveInterval {
                            p_states: array![0.3, 0.7],
                            start_time: 0.1,
                            end_time: 0.2,
                        },
                    ),
                    (
                        "Hungry",
                        E::UncertainNegativeInterval {
                            p_not_states: array![0.1, 0.9],
                            start_time: 0.3,
                            end_time: 0.5,
                        },
                    ),
                ],
            );

            // Initialize sampler.
            let mut importance = ImportanceSampler::new(&mut rng, &ctbn);
            // Sample from CTBN.
            let (trajectory, _weight) =
                importance.sample_by_length_or_time(&evidence, 100, f64::INFINITY);

            // Check labels.
            assert!(trajectory.labels().eq(ctbn.labels()));
            // Check sample size.
            assert_eq!(trajectory.sample_size(), 100);
        }
    }
}

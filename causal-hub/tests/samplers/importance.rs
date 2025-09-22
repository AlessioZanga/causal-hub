#[cfg(test)]
mod tests {
    mod bayesian_network {}

    mod continuous_time_bayesian_network {
        use causal_hub::{
            assets::load_eating,
            datasets::{CatTrjEv, CatTrjEvT as E, Dataset},
            models::Labelled,
            samplers::{ImportanceSampler, ParCTBNSampler},
        };
        use ndarray::prelude::*;
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        #[test]
        fn importance_sampling_by_length() {
            // Initialize RNG.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Initialize the model.
            let model = load_eating();

            // Initialize evidence.
            let evidence = CatTrjEv::new(
                model.states().clone(),
                [
                    E::CertainPositiveInterval {
                        event: 2, // "Hungry"
                        state: 0,
                        start_time: 0.,
                        end_time: 0.2,
                    },
                    E::CertainNegativeInterval {
                        event: 0, // "Eating"
                        not_states: [0].into_iter().collect(),
                        start_time: 0.,
                        end_time: 0.2,
                    },
                    E::UncertainPositiveInterval {
                        event: 1, // "FullStomach"
                        p_states: array![0.3, 0.7],
                        start_time: 0.1,
                        end_time: 0.2,
                    },
                    E::UncertainNegativeInterval {
                        event: 2, // "Hungry"
                        p_not_states: array![0.9, 0.1],
                        start_time: 0.3,
                        end_time: 0.5,
                    },
                ],
            );

            // Initialize sampler.
            let importance = ImportanceSampler::new(&mut rng, &model, &evidence);
            // Sample from CTBN.
            let weighted_trajectory = importance.par_sample_n_by_length(10, 10);

            // Get trajectory.
            let trajectory = weighted_trajectory.into_iter().next().unwrap().trajectory();

            // Check labels.
            assert!(trajectory.labels().eq(model.labels()));
            // Check sample size.
            assert_eq!(trajectory.sample_size(), 10.);
        }
    }
}

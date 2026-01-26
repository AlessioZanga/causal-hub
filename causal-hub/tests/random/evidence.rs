#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTrj, CatTrjEvT as E, CatTrjs, Dataset},
        random::RngEv,
        states,
        types::Result,
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    // Helper function to create a sample trajectory.
    fn create_sample_trajectory() -> Result<CatTrj> {
        let states = states![("A", ["0", "1", "2"]), ("B", ["0", "1"]), ("C", ["0", "1"])];
        let events = array![
            [0, 0, 0],
            [1, 0, 0],
            [1, 1, 0],
            [1, 0, 0],
            [2, 0, 0],
            [2, 0, 1]
        ];
        let times = array![0.0, 0.1, 0.2, 0.3, 0.4, 0.5];
        CatTrj::new(states, events, times)
    }

    // Helper function to create sample trajectories.
    fn create_sample_trajectories() -> Result<CatTrjs> {
        let trj_0 = create_sample_trajectory()?;
        let trj_1 = {
            let states = states![("A", ["0", "1", "2"]), ("B", ["0", "1"]), ("C", ["0", "1"])];
            // Only one state change per transition.
            let events = array![[0, 0, 0], [0, 1, 0], [1, 1, 0], [2, 1, 0]];
            let times = array![0.0, 0.2, 0.4, 0.6];
            CatTrj::new(states, events, times)?
        };
        CatTrjs::new([trj_0, trj_1])
    }

    mod rng_ev_validation {
        use super::*;

        #[test]
        fn new_with_valid_probability_zero() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let rng_ev = RngEv::new(&mut rng, &trj, 0.0);
            assert!(rng_ev.is_ok());
            Ok(())
        }

        #[test]
        fn new_with_valid_probability_one() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let rng_ev = RngEv::new(&mut rng, &trj, 1.0);
            assert!(rng_ev.is_ok());
            Ok(())
        }

        #[test]
        fn new_with_valid_probability_half() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let rng_ev = RngEv::new(&mut rng, &trj, 0.5);
            assert!(rng_ev.is_ok());
            Ok(())
        }

        #[test]
        fn new_with_invalid_probability_negative() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let res = RngEv::new(&mut rng, &trj, -0.1);
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.to_string(), "Invalid parameter p: must be in [0, 1]");
            Ok(())
        }

        #[test]
        fn new_with_invalid_probability_greater_than_one() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let res = RngEv::new(&mut rng, &trj, 1.1);
            assert!(res.is_err());
            let err = res.err().unwrap();
            assert_eq!(err.to_string(), "Invalid parameter p: must be in [0, 1]");
            Ok(())
        }

        #[test]
        fn new_with_invalid_probability_large() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let res = RngEv::new(&mut rng, &trj, 100.0);
            assert!(res.is_err());
            Ok(())
        }
    }

    mod rng_ev_single_trajectory {
        use super::*;

        #[test]
        fn random_with_probability_zero() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let mut rng_ev = RngEv::new(&mut rng, &trj, 0.0)?;
            let evidence = rng_ev.random()?;

            // With p=0.0, no evidence should be selected.
            let total_evidence: usize = evidence.evidences().iter().map(|v| v.len()).sum();
            assert_eq!(total_evidence, 0);
            Ok(())
        }

        #[test]
        fn random_with_probability_one() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let mut rng_ev = RngEv::new(&mut rng, &trj, 1.0)?;
            let evidence = rng_ev.random()?;

            // With p=1.0, some evidence should be selected.
            // evidences() returns &Vec<Vec<CatTrjEvT>> - one Vec per variable.
            let total_evidence: usize = evidence.evidences().iter().map(|v| v.len()).sum();
            assert!(total_evidence > 0);

            // Check evidence is within valid bounds.
            for var_evidences in evidence.evidences() {
                for ev in var_evidences {
                    if let E::CertainPositiveInterval {
                        event,
                        start_time,
                        end_time,
                        ..
                    } = ev
                    {
                        // Event should be within bounds (0, 1, 2 for 3 variables).
                        assert!(*event < 3);
                        // Start time should be before end time.
                        assert!(*start_time < *end_time);
                        // Times should be within trajectory range [0.0, 0.5].
                        assert!(*start_time >= 0.0);
                        assert!(*end_time <= 0.5);
                    }
                }
            }
            Ok(())
        }

        #[test]
        fn random_with_probability_half_reproducibility() -> Result<()> {
            let mut rng1 = Xoshiro256PlusPlus::seed_from_u64(42);
            let mut rng2 = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;

            let mut rng_ev1 = RngEv::new(&mut rng1, &trj, 0.5)?;
            let mut rng_ev2 = RngEv::new(&mut rng2, &trj, 0.5)?;

            let evidence1 = rng_ev1.random()?;
            let evidence2 = rng_ev2.random()?;

            // Same seed should produce same results.
            let total1: usize = evidence1.evidences().iter().map(|v| v.len()).sum();
            let total2: usize = evidence2.evidences().iter().map(|v| v.len()).sum();
            assert_eq!(total1, total2);
            Ok(())
        }

        #[test]
        fn random_multiple_calls() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trj = create_sample_trajectory()?;
            let mut rng_ev = RngEv::new(&mut rng, &trj, 0.8)?;

            // Call random.
            let evidence1 = rng_ev.random()?;

            // Create a new generator with the same (now advanced) rng.
            let mut rng_ev2 = RngEv::new(&mut rng, &trj, 0.8)?;
            let evidence2 = rng_ev2.random()?;

            // Both should return valid evidence (may or may not be equal).
            // At least test they don't crash.
            let total1: usize = evidence1.evidences().iter().map(|v| v.len()).sum();
            let total2: usize = evidence2.evidences().iter().map(|v| v.len()).sum();
            assert!(total1 <= trj.times().len() * 3);
            assert!(total2 <= trj.times().len() * 3);
            Ok(())
        }
    }

    mod rng_ev_multiple_trajectories {
        use causal_hub::models::Labelled;

        use super::*;

        #[test]
        fn new_with_multiple_trajectories() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trjs = create_sample_trajectories()?;
            let rng_ev = RngEv::new(&mut rng, &trjs, 0.5);
            assert!(rng_ev.is_ok());
            Ok(())
        }

        #[test]
        fn random_with_probability_zero() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trjs = create_sample_trajectories()?;
            let mut rng_ev = RngEv::new(&mut rng, &trjs, 0.0)?;
            let evidences = rng_ev.random()?;

            // With p=0.0, no evidence should be selected from each trajectory.
            // evidences.evidences() returns &Vec<CatTrjEv>.
            for ev in evidences.evidences() {
                // ev.evidences() returns &Vec<Vec<CatTrjEvT>>.
                let total: usize = ev.evidences().iter().map(|v| v.len()).sum();
                assert_eq!(total, 0);
            }
            Ok(())
        }

        #[test]
        fn random_with_probability_one() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trjs = create_sample_trajectories()?;
            let mut rng_ev = RngEv::new(&mut rng, &trjs, 1.0)?;
            let evidences = rng_ev.random()?;

            // With p=1.0, some evidence should be selected from each trajectory.
            assert_eq!(evidences.evidences().len(), trjs.values().len());

            // Each evidence set should have some evidence (with high probability).
            for ev in evidences.evidences() {
                // Verify the evidence has the same labels as the trajectories.
                assert_eq!(ev.labels(), trjs.labels());
            }
            Ok(())
        }

        #[test]
        fn random_preserves_trajectory_count() -> Result<()> {
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            let trjs = create_sample_trajectories()?;
            let n_trajectories = trjs.values().len();

            let mut rng_ev = RngEv::new(&mut rng, &trjs, 0.7)?;
            let evidences = rng_ev.random()?;

            // Should have the same number of evidence sets as trajectories.
            assert_eq!(evidences.evidences().len(), n_trajectories);
            Ok(())
        }

        #[test]
        fn random_reproducibility() -> Result<()> {
            let mut rng1 = Xoshiro256PlusPlus::seed_from_u64(123);
            let mut rng2 = Xoshiro256PlusPlus::seed_from_u64(123);
            let trjs = create_sample_trajectories()?;

            let mut rng_ev1 = RngEv::new(&mut rng1, &trjs, 0.6)?;
            let mut rng_ev2 = RngEv::new(&mut rng2, &trjs, 0.6)?;

            let evidences1 = rng_ev1.random()?;
            let evidences2 = rng_ev2.random()?;

            // Same seed should give same number of evidence entries.
            assert_eq!(evidences1.evidences().len(), evidences2.evidences().len());
            for (ev1, ev2) in evidences1.evidences().iter().zip(evidences2.evidences()) {
                // Compare total evidence count for each trajectory.
                let total1: usize = ev1.evidences().iter().map(|v| v.len()).sum();
                let total2: usize = ev2.evidences().iter().map(|v| v.len()).sum();
                assert_eq!(total1, total2);
            }
            Ok(())
        }
    }
}

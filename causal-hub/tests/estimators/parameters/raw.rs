#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_eating,
        datasets::{CatTrjEv, CatTrjEvT, Dataset},
        estimators::RE,
    };
    use ndarray::prelude::*;

    #[test]
    fn test_raw_fill_1() {
        // Short the evidence name.
        use CatTrjEvT as E;
        // Load the model.
        let model = load_eating();
        // Initialize the evidence.
        let evidence = CatTrjEv::new(
            model.states(),
            [
                (
                    "Hungry",
                    E::CertainPositiveInterval {
                        state: 0,
                        start_time: 0.1,
                        end_time: 0.2,
                    },
                ),
                (
                    "Eating",
                    E::CertainPositiveInterval {
                        state: 0,
                        start_time: 0.3,
                        end_time: 0.4,
                    },
                ),
                (
                    "FullStomach",
                    E::CertainPositiveInterval {
                        state: 0,
                        start_time: 0.5,
                        end_time: 0.6,
                    },
                ),
            ],
        );
        // Fill the evidence.
        let filled_evidence = RE::fill(&evidence);
        // Check the filled evidence times.
        assert_eq!(filled_evidence.times(), array![0., 0.1, 0.3, 0.5, 0.6]);
        // Check the filled evidence.
        assert_eq!(
            filled_evidence.values(),
            array![
                [0, 0, 0], // 0.
                [0, 0, 0], // 0.1
                [0, 0, 0], // 0.3
                [0, 0, 0], // 0.5
                [0, 0, 0], // 0.6 (Ending time of the last event)
            ]
        );
    }

    #[test]
    fn test_raw_fill_2() {
        // Short the evidence name.
        use CatTrjEvT as E;
        // Load the model.
        let model = load_eating();
        // Initialize the evidence.
        let evidence = CatTrjEv::new(
            model.states(),
            [
                (
                    "Hungry",
                    E::CertainPositiveInterval {
                        state: 0,
                        start_time: 0.1,
                        end_time: 0.2,
                    },
                ),
                (
                    "Eating",
                    E::CertainPositiveInterval {
                        state: 0,
                        start_time: 0.1,
                        end_time: 0.3,
                    },
                ),
                (
                    "Eating",
                    E::CertainPositiveInterval {
                        state: 1,
                        start_time: 0.3,
                        end_time: 0.4,
                    },
                ),
                (
                    "FullStomach",
                    E::CertainPositiveInterval {
                        state: 0,
                        start_time: 0.5,
                        end_time: 0.6,
                    },
                ),
            ],
        );
        // Fill the evidence.
        let filled_evidence = RE::fill(&evidence);
        // Check the filled evidence times.
        assert_eq!(filled_evidence.times(), array![0., 0.1, 0.3, 0.5, 0.6]);
        // Check the filled evidence.
        assert_eq!(
            filled_evidence.values(),
            array![
                [0, 0, 0], // 0.
                [0, 0, 0], // 0.1
                [1, 0, 0], // 0.3
                [1, 0, 0], // 0.5
                [1, 0, 0], // 0.6 (Ending time of the last event)
            ]
        );
    }
}

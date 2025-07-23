#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTrjEv, CatTrjEvT as E},
        set,
        types::Map,
    };
    use ndarray::prelude::*;

    #[test]
    fn test_new_evidence() {
        // Initialize the model.
        let states = Map::from_iter([
            ("B", set!["0", "1"]),
            ("A", set!["0", "1", "2"]),
            ("C", set!["0", "1"]),
        ]);

        // Initialize evidence.
        let _evidence = CatTrjEv::new(
            states,
            [
                E::CertainPositiveInterval {
                    event: 2,
                    state: 0,
                    start_time: 0.,
                    end_time: 0.2,
                },
                E::CertainNegativeInterval {
                    event: 0,
                    not_states: [0].into_iter().collect(),
                    start_time: 0.,
                    end_time: 0.2,
                },
                E::UncertainPositiveInterval {
                    event: 1,
                    p_states: array![0.3, 0.7, 0.],
                    start_time: 0.1,
                    end_time: 0.2,
                },
                E::UncertainNegativeInterval {
                    event: 2,
                    p_not_states: array![0.9, 0.1],
                    start_time: 0.3,
                    end_time: 0.5,
                },
            ],
        );
    }
}

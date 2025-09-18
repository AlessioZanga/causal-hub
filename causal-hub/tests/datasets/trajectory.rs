#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatTrjEv, CatTrjEvT as E},
        map, set,
    };
    use ndarray::prelude::*;

    #[test]
    fn new_evidence() {
        // Initialize the model.
        let states = map![
            ("B".to_owned(), set!["0".to_owned(), "1".to_owned()]),
            (
                "A".to_owned(),
                set!["0".to_owned(), "1".to_owned(), "2".to_owned()]
            ),
            ("C".to_owned(), set!["0".to_owned(), "1".to_owned()]),
        ];

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

#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_eating,
        datasets::{CatTrj, CatTrjEv, CatTrjEvT, Dataset},
        estimators::RAWE,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn raw_fill_1() -> Result<()> {
        // Short the evidence name.
        use CatTrjEvT as E;
        // Load the model.
        let model = load_eating()?;
        // Initialize the evidence.
        let evidence = CatTrjEv::new(
            model.support().clone(),
            [
                E::CertainPositiveInterval {
                    event: 2, // "Hungry"
                    state: 0,
                    start_time: 0.1,
                    end_time: 0.2,
                },
                E::CertainPositiveInterval {
                    event: 0, // "Eating"
                    state: 0,
                    start_time: 0.3,
                    end_time: 0.4,
                },
                E::CertainPositiveInterval {
                    event: 1, // "FullStomach"
                    state: 0,
                    start_time: 0.5,
                    end_time: 0.6,
                },
            ],
        )?;
        // Fill the evidence.
        let filled_evidence = RAWE::<CatTrj>::par_new(&evidence)?;
        // Check the filled evidence times.
        assert_eq!(
            filled_evidence.dataset().times(),
            array![0., 0.1, 0.3, 0.5, 0.6]
        );
        // Check the filled evidence.
        assert_eq!(
            filled_evidence.dataset().values(),
            array![
                [0, 0, 0], // 0.
                [0, 0, 0], // 0.1
                [0, 0, 0], // 0.3
                [0, 0, 0], // 0.5
                [0, 0, 0], // 0.6 (Ending time of the last event)
            ]
        );

        Ok(())
    }

    #[test]
    fn raw_fill_2() -> Result<()> {
        // Short the evidence name.
        use CatTrjEvT as E;
        // Load the model.
        let model = load_eating()?;
        // Initialize the evidence.
        let evidence = CatTrjEv::new(
            model.support().clone(),
            [
                E::CertainPositiveInterval {
                    event: 2, // "Hungry"
                    state: 0,
                    start_time: 0.1,
                    end_time: 0.2,
                },
                E::CertainPositiveInterval {
                    event: 0, // "Eating"
                    state: 0,
                    start_time: 0.1,
                    end_time: 0.3,
                },
                E::CertainPositiveInterval {
                    event: 0, // "Eating"
                    state: 1,
                    start_time: 0.3,
                    end_time: 0.4,
                },
                E::CertainPositiveInterval {
                    event: 1, // "FullStomach"
                    state: 0,
                    start_time: 0.5,
                    end_time: 0.6,
                },
            ],
        )?;
        // Fill the evidence.
        let filled_evidence = RAWE::<CatTrj>::par_new(&evidence)?;
        // Check the filled evidence times.
        assert_eq!(
            filled_evidence.dataset().times(),
            array![0., 0.1, 0.3, 0.5, 0.6]
        );
        // Check the filled evidence.
        assert_eq!(
            filled_evidence.dataset().values(),
            array![
                [0, 0, 0], // 0.
                [0, 0, 0], // 0.1
                [1, 0, 0], // 0.3
                [1, 0, 0], // 0.5
                [1, 0, 0], // 0.6 (Ending time of the last event)
            ]
        );

        Ok(())
    }
}

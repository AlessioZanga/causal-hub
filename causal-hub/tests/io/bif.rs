#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{assets::*, io::BifIO, models::CatBN, types::Result};
    use dry::macro_for;
    use paste::paste;

    mod bayesian_networks {
        use super::*;
        mod categorical {
            use super::*;

            macro_for!(
                $bn in [
                    alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
                    hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
                    pigs, sachs, survey, water, win95pts
                ] {
                paste! {
                    #[test]
                    fn [<from_bif_ $bn>]() -> Result<()> {
                        // Load model.
                        let true_model = [<load_ $bn>]()?;
                        // Serialize model to BIF.
                        let bif = true_model.to_bif_string()?;
                        // Deserialize model from BIF.
                        let pred_model = CatBN::from_bif_string(bif.as_str())?;
                        // Assert the models are equal.
                        assert_relative_eq!(true_model, pred_model);

                        Ok(())
                    }
                }
            });
        }
    }
}

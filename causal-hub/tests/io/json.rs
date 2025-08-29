#[cfg(test)]
mod tests {
    mod categorical {
        mod bayesian_networks {
            use approx::assert_relative_eq;
            use causal_hub::{assets::*, io::JsonIO, models::CatBN};
            use dry::macro_for;
            use paste::paste;

            macro_for!(
                $bn in [
                    alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
                    hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
                    pigs, sachs, survey, water, win95pts
                ] {
                paste! {
                    #[test]
                    fn [<test_from_json_ $bn>]() {
                        // Load model.
                        let true_model = [<load_ $bn>]();
                        // Serialize model to JSON.
                        let json = true_model.to_json();
                        // Deserialize model from JSON.
                        let pred_model = CatBN::from_json(json.as_str());
                        // Assert the models are equal.
                        assert_relative_eq!(true_model, pred_model);
                    }
                }
            });

            #[test]
            fn test_to_json_asia() {
                // Load model.
                let true_model = load_asia();
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Assert the JSON string is correct.
                assert_eq!(
                    json,
                    r#"{"graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]]},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.99,0.01]]},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.7,0.3],[0.4,0.6]]},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.9,0.1],[0.3,0.7],[0.2,0.8],[0.1,0.9]]},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[1.0,0.0],[0.0,1.0],[0.0,1.0],[0.0,1.0]]},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.99,0.01],[0.9,0.1]]},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.99,0.01],[0.95,0.05]]},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.95,0.05],[0.02,0.98]]}]}"#
                );
            }
        }

        mod continuous_time_bayesian_network {
            use causal_hub::{assets::load_eating, io::JsonIO, models::CatCTBN};

            #[test]
            fn test_from_json_eating() {
                // Load model.
                let true_model = load_eating();
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Deserialize model from JSON.
                let pred_model = CatCTBN::from_json(json.as_str());
                // Assert the models are equal.
                assert_eq!(true_model, pred_model);
            }

            #[test]
            fn test_to_json_eating() {
                // Load model.
                let true_model = load_eating();
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Assert the JSON string is correct.
                assert_eq!(
                    json,
                    r#"{"initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[]},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]}]},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]]},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]}]}"#
                );
            }
        }
    }
}

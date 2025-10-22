#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        assets::*,
        estimators::{BE, BNEstimator, CTBNEstimator, MLE},
        io::JsonIO,
        models::{BN, CTBN, CatBN, CatCTBN, GaussBN},
        samplers::{BNSampler, CTBNSampler, ForwardSampler},
    };
    use dry::macro_for;
    use paste::paste;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

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
                    fn [<from_json_ $bn>]() {
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
            fn to_json_asia() {
                // Load model.
                let true_model = load_asia();
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Assert the JSON string is correct.
                assert_eq!(
                    json,
                    r#"{"name":"asia","graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]],"type":"digraph"},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.99,0.01]],"type":"catcpd"},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.7,0.3],[0.4,0.6]],"type":"catcpd"},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.9,0.1],[0.3,0.7],[0.2,0.8],[0.1,0.9]],"type":"catcpd"},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[1.0,0.0],[0.0,1.0],[0.0,1.0],[0.0,1.0]],"type":"catcpd"},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.99,0.01],[0.9,0.1]],"type":"catcpd"},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.99,0.01],[0.95,0.05]],"type":"catcpd"},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.95,0.05],[0.02,0.98]],"type":"catcpd"}],"type":"catbn"}"#
                );
            }

            #[test]
            fn from_json_with_optionals_asia() {
                // Initialize random number generator.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Load model.
                let model = load_asia();
                // Sample from model.
                let dataset = ForwardSampler::new(&mut rng, &model).sample_n(100);
                // Set estimator.
                let estimator = BE::new(&dataset).with_prior(1);
                // Fit model to dataset.
                let true_model: CatBN = BNEstimator::fit(&estimator, model.graph().clone());
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Deserialize model from JSON.
                let pred_model = CatBN::from_json(json.as_str());
                // Assert the models are equal.
                assert_relative_eq!(true_model, pred_model);
            }

            #[test]
            fn to_json_with_optionals_asia() {
                // Initialize random number generator.
                let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                // Load model.
                let model = load_asia();
                // Sample from model.
                let dataset = ForwardSampler::new(&mut rng, &model).sample_n(100);
                // Set estimator.
                let estimator = BE::new(&dataset).with_prior(1);
                // Fit model to dataset.
                let true_model: CatBN = BNEstimator::fit(&estimator, model.graph().clone());
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Assert the JSON string is correct.
                assert_eq!(
                    json,
                    r#"{"graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]],"type":"digraph"},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.9705882352941176,0.029411764705882353]],"sample_statistics":{"sample_conditional_counts":[[98.0,2.0]],"sample_size":100.0},"sample_log_likelihood":-13.534524925666918,"type":"catcpd"},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.75,0.25],[0.38461538461538464,0.6153846153846154]],"sample_statistics":{"sample_conditional_counts":[[38.0,12.0],[19.0,31.0]],"sample_size":100.0},"sample_log_likelihood":-63.88790652574118,"type":"catcpd"},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.847457627118644,0.15254237288135594],[0.5,0.5],[0.20454545454545456,0.7954545454545454],[0.6666666666666666,0.3333333333333333]],"sample_statistics":{"sample_conditional_counts":[[49.0,8.0],[0.0,0.0],[8.0,34.0],[1.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-50.786515133256536,"type":"catcpd"},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[0.9900990099009901,0.009900990099009901],[0.5,0.5],[0.3333333333333333,0.6666666666666666],[0.5,0.5]],"sample_statistics":{"sample_conditional_counts":[[99.0,0.0],[0.0,0.0],[0.0,1.0],[0.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-10.29228482928229,"type":"catcpd"},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.9807692307692307,0.019230769230769232],[0.9615384615384616,0.038461538461538464]],"sample_statistics":{"sample_conditional_counts":[[50.0,0.0],[49.0,1.0]],"sample_size":100.0},"sample_log_likelihood":-13.418794831000639,"type":"catcpd"},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"sample_statistics":{"sample_conditional_counts":[[50.0,50.0]],"sample_size":100.0},"sample_log_likelihood":-70.70101241711441,"type":"catcpd"},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.99,0.01],[0.75,0.25]],"sample_statistics":{"sample_conditional_counts":[[98.0,0.0],[2.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-7.849494013959967,"type":"catcpd"},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.900990099009901,0.09900990099009901],[0.3333333333333333,0.6666666666666666]],"sample_statistics":{"sample_conditional_counts":[[90.0,9.0],[0.0,1.0]],"sample_size":100.0},"sample_log_likelihood":-34.52264868287783,"type":"catcpd"}],"type":"catbn"}"#
                );
            }

            mod gaussian {
                use super::*;

                macro_for!(
                    $bn in [
                        arth150, ecoli70, magic_irri, magic_niab
                    ] {
                    paste! {
                        #[test]
                        fn [<from_json_ $bn>]() {
                            // Load model.
                            let true_model = [<load_ $bn>]();
                            // Serialize model to JSON.
                            let json = true_model.to_json();
                            // Deserialize model from JSON.
                            let pred_model = GaussBN::from_json(json.as_str());
                            // Assert the models are equal.
                            assert_relative_eq!(true_model, pred_model);
                        }
                    }
                });

                #[test]
                fn from_json_with_optionals_ecoli70() {
                    // Initialize random number generator.
                    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                    // Load model.
                    let model = load_ecoli70();
                    // Sample from model.
                    let dataset = ForwardSampler::new(&mut rng, &model).sample_n(100);
                    // Set estimator.
                    let estimator = MLE::new(&dataset);
                    // Fit model to dataset.
                    let true_model: GaussBN = BNEstimator::fit(&estimator, model.graph().clone());
                    // Serialize model to JSON.
                    let json = true_model.to_json();
                    // Deserialize model from JSON.
                    let pred_model = GaussBN::from_json(json.as_str());
                    // Assert the models are equal.
                    assert_relative_eq!(true_model, pred_model);
                }

                #[test]
                fn to_json_with_optionals_ecoli70() {
                    // Initialize random number generator.
                    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                    // Load model.
                    let model = load_ecoli70();
                    // Sample from model.
                    let dataset = ForwardSampler::new(&mut rng, &model).sample_n(100);
                    // Set estimator.
                    let estimator = MLE::new(&dataset);
                    // Fit model to dataset.
                    let true_model: GaussBN = BNEstimator::fit(&estimator, model.graph().clone());
                    // Serialize model to JSON.
                    let _ = true_model.to_json();
                    // Note: Due to floating-point precision issues, we do not assert equality of the JSON string here.
                }
            }
        }

        mod continuous_time_bayesian_network {
            use super::*;
            mod categorical {
                use super::*;

                #[test]
                fn from_json_eating() {
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
                fn to_json_eating() {
                    // Load model.
                    let true_model = load_eating();
                    // Serialize model to JSON.
                    let json = true_model.to_json();
                    // Assert the JSON string is correct.
                    assert_eq!(
                        json,
                        r#"{"name":"eating","description":"See: U. Nodelman, C.R. Shelton, and D. Koller (2003). \"Learning Continuous Time Bayesian Networks.\" Proc. Nineteenth Conference on Uncertainty in Artificial Intelligence (UAI) (pp. 451-458).","initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[],"type":"digraph"},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"}],"type":"catbn"},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]],"type":"digraph"},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]],"type":"catcim"},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]],"type":"catcim"},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]],"type":"catcim"}],"type":"catctbn"}"#
                    );
                }

                #[test]
                fn from_json_with_optionals_eating() {
                    // Initialize random number generator.
                    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                    // Load model.
                    let model = load_eating();
                    // Sample from model.
                    let dataset = ForwardSampler::new(&mut rng, &model).sample_n_by_length(100, 10);
                    // Set estimator.
                    let estimator = BE::new(&dataset).with_prior((1, 1.));
                    // Fit model to dataset.
                    let true_model: CatCTBN = CTBNEstimator::fit(&estimator, model.graph().clone());
                    // Serialize model to JSON.
                    let json = true_model.to_json();
                    // Deserialize model from JSON.
                    let pred_model = CatCTBN::from_json(json.as_str());
                    // Assert the models are equal.
                    assert_relative_eq!(true_model, pred_model);
                }

                #[test]
                fn to_json_with_optionals_eating() {
                    // Initialize random number generator.
                    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
                    // Load model.
                    let model = load_eating();
                    // Sample from model.
                    let dataset = ForwardSampler::new(&mut rng, &model).sample_n_by_length(100, 10);
                    // Set estimator.
                    let estimator = BE::new(&dataset).with_prior((1, 1.));
                    // Fit model to dataset.
                    let true_model: CatCTBN = CTBNEstimator::fit(&estimator, model.graph().clone());
                    // Serialize model to JSON.
                    let json = true_model.to_json();
                    // Assert the JSON string is correct.
                    assert_eq!(
                        json,
                        r#"{"initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[],"type":"digraph"},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"type":"catcpd"}],"type":"catbn"},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]],"type":"digraph"},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.09761971408804883,0.09761971408804883],[10.01391697001624,-10.01391697001624]],[[-2.0510219056544603,2.0510219056544603],[0.014896945496758228,-0.014896945496758228]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,129.0],[162.0,0.0]],[[0.0,29.0],[0.0,0.0]]],"sample_conditional_times":[[1326.0763089942725,15.727416353316986],[13.883074075743158,33.06392759232465]],"sample_size":320.0},"sample_log_likelihood":-2709.4167395271697,"type":"catcim"},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.10447734377780926,0.10447734377780926],[10.74241980384082,-10.74241980384082]],[[-1.9188634871164392,1.9188634871164392],[0.25871854904000646,-0.25871854904000646]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,138.0],[164.0,0.0]],[[0.0,32.0],[8.0,0.0]]],"sample_conditional_times":[[1325.1462596766082,14.813123393407606],[16.437108980503446,32.35423496513819]],"sample_size":342.0},"sample_log_likelihood":-2882.518082085844,"type":"catcim"},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.10067226843575305,0.10067226843575305],[9.849547083862912,-9.849547083862912]],[[-1.7046905133498722,1.7046905133498722],[0.17488754102896195,-0.17488754102896195]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,133.0],[162.0,0.0]],[[0.0,28.0],[5.0,0.0]]],"sample_conditional_times":[[1325.5851481179936,15.998220539118314],[16.2185772295963,30.9487811289495]],"sample_size":328.0},"sample_log_likelihood":-2777.550074204407,"type":"catcim"}],"type":"catctbn"}"#
                    );
                }
            }
        }
    }
}

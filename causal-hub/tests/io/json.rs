#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        assets::*,
        estimation::{BE, BNEstimator, CTBNEstimator},
        io::JsonIO,
        models::{BN, CTBN, CatBN, CatCTBN, GaussBN},
        samplers::{BNSampler, CTBNSampler, ForwardSampler},
    };
    use dry::macro_for;
    use paste::paste;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256StarStar;

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
                    r#"{"name":"asia","graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]]},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.99,0.01]]},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.7,0.3],[0.4,0.6]]},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.9,0.1],[0.3,0.7],[0.2,0.8],[0.1,0.9]]},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[1.0,0.0],[0.0,1.0],[0.0,1.0],[0.0,1.0]]},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.99,0.01],[0.9,0.1]]},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.99,0.01],[0.95,0.05]]},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.95,0.05],[0.02,0.98]]}]}"#
                );
            }

            #[test]
            fn from_json_with_optionals_asia() {
                // Initialize random number generator.
                let mut rng = Xoshiro256StarStar::seed_from_u64(42);
                // Load model.
                let model = load_asia();
                // Sample from model.
                let data = ForwardSampler::new(&mut rng, &model).sample_n(100);
                // Set estimator.
                let estimator = BE::new(&data, 1);
                // Fit model to data.
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
                let mut rng = Xoshiro256StarStar::seed_from_u64(42);
                // Load model.
                let model = load_asia();
                // Sample from model.
                let data = ForwardSampler::new(&mut rng, &model).sample_n(100);
                // Set estimator.
                let estimator = BE::new(&data, 1);
                // Fit model to data.
                let true_model: CatBN = BNEstimator::fit(&estimator, model.graph().clone());
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Assert the JSON string is correct.
                assert_eq!(
                    json,
                    r#"{"graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]]},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.9901960784313726,0.00980392156862745]],"sample_statistics":{"sample_conditional_counts":[[100.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-5.620054754028442},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.7083333333333334,0.2916666666666667],[0.375,0.625]],"sample_statistics":{"sample_conditional_counts":[[33.0,13.0],[20.0,34.0]],"sample_size":100.0},"sample_log_likelihood":-66.02212940886265},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.9056603773584906,0.09433962264150944],[0.25,0.75],[0.18181818181818182,0.8181818181818182],[0.14285714285714285,0.8571428571428571]],"sample_statistics":{"sample_conditional_counts":[[47.0,4.0],[0.0,2.0],[7.0,35.0],[0.0,5.0]],"sample_size":100.0},"sample_log_likelihood":-42.542917913552124},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[0.9894736842105263,0.010526315789473684],[0.25,0.75],[0.14285714285714285,0.8571428571428571],[0.5,0.5]],"sample_statistics":{"sample_conditional_counts":[[93.0,0.0],[0.0,2.0],[0.0,5.0],[0.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-12.055044336285004},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.9375,0.0625],[0.9285714285714286,0.07142857142857142]],"sample_statistics":{"sample_conditional_counts":[[44.0,2.0],[51.0,3.0]],"sample_size":100.0},"sample_log_likelihood":-25.631843488364616},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.46078431372549017,0.5392156862745098]],"sample_statistics":{"sample_conditional_counts":[[46.0,54.0]],"sample_size":100.0},"sample_log_likelihood":-70.386964486837},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.9705882352941176,0.029411764705882353],[0.5,0.5]],"sample_statistics":{"sample_conditional_counts":[[98.0,2.0],[0.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-14.92081928678681},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.8842105263157894,0.11578947368421053],[0.1111111111111111,0.8888888888888888]],"sample_statistics":{"sample_conditional_counts":[[83.0,10.0],[0.0,7.0]],"sample_size":100.0},"sample_log_likelihood":-37.192334461018255}]}"#
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
                        r#"{"name":"eating","description":"See: U. Nodelman, C.R. Shelton, and D. Koller (2003). \"Learning Continuous Time Bayesian Networks.\" Proc. Nineteenth Conference on Uncertainty in Artificial Intelligence (UAI) (pp. 451-458).","initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[]},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]}]},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]]},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]}]}"#
                    );
                }

                #[test]
                fn from_json_with_optionals_eating() {
                    // Initialize random number generator.
                    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
                    // Load model.
                    let model = load_eating();
                    // Sample from model.
                    let data = ForwardSampler::new(&mut rng, &model).sample_n_by_length(100, 10);
                    // Set estimator.
                    let estimator = BE::new(&data, (1, 1.));
                    // Fit model to data.
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
                    let mut rng = Xoshiro256StarStar::seed_from_u64(42);
                    // Load model.
                    let model = load_eating();
                    // Sample from model.
                    let data = ForwardSampler::new(&mut rng, &model).sample_n_by_length(100, 10);
                    // Set estimator.
                    let estimator = BE::new(&data, (1, 1.));
                    // Fit model to data.
                    let true_model: CatCTBN = CTBNEstimator::fit(&estimator, model.graph().clone());
                    // Serialize model to JSON.
                    let json = true_model.to_json();
                    // Assert the JSON string is correct.
                    assert_eq!(
                        json,
                        r#"{"initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[]},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]}]},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]]},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.09406907280842255,0.09406907280842255],[9.590371935974677,-9.590371935974677]],[[-1.4237643489909868,1.4237643489909868],[0.05751544497215996,-0.05751544497215996]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,129.0],[147.0,0.0]],[[0.0,24.0],[3.0,0.0]]],"sample_conditional_times":[[1376.1479899693995,14.880008302567408],[16.707903834200515,60.35321954292722]],"sample_size":303.0},"sample_log_likelihood":-2576.871295098585},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.1058520429397253,0.1058520429397253],[9.40621931852698,-9.40621931852698]],[[-2.374840197341832,2.374840197341832],[0.07292653503202201,-0.07292653503202201]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,145.0],[181.0,0.0]],[[0.0,34.0],[4.0,0.0]]],"sample_conditional_times":[[1374.0601498012768,18.79574400232282],[14.027293263191345,61.2059345823033]],"sample_size":364.0},"sample_log_likelihood":-3139.3065886835175},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.09139034314491055,0.09139034314491055],[9.806385119724002,-9.806385119724002]],[[-1.8353435502123672,1.8353435502123672],[0.15272303246669033,-0.15272303246669033]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,125.0],[155.0,0.0]],[[0.0,34.0],[9.0,0.0]]],"sample_conditional_times":[[1372.7304276503746,15.357015414093436],[18.297570621591806,61.70410796303431]],"sample_size":323.0},"sample_log_likelihood":-2694.431194670671}]}"#
                    );
                }
            }
        }
    }
}

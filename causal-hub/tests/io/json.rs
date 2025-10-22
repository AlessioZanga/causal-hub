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
                    r#"{"name":"asia","graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]]},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.99,0.01]]},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.7,0.3],[0.4,0.6]]},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.9,0.1],[0.3,0.7],[0.2,0.8],[0.1,0.9]]},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[1.0,0.0],[0.0,1.0],[0.0,1.0],[0.0,1.0]]},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.99,0.01],[0.9,0.1]]},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.99,0.01],[0.95,0.05]]},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.95,0.05],[0.02,0.98]]}]}"#
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
                    r#"{"graph":{"labels":["asia","bronc","dysp","either","lung","smoke","tub","xray"],"edges":[["asia","tub"],["bronc","dysp"],["either","dysp"],["either","xray"],["lung","either"],["smoke","bronc"],["smoke","lung"],["tub","either"]]},"cpds":[{"states":{"asia":["no","yes"]},"conditioning_states":{},"parameters":[[0.9705882352941176,0.029411764705882353]],"sample_statistics":{"sample_conditional_counts":[[98.0,2.0]],"sample_size":100.0},"sample_log_likelihood":-13.534524925666918},{"states":{"bronc":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.75,0.25],[0.38461538461538464,0.6153846153846154]],"sample_statistics":{"sample_conditional_counts":[[38.0,12.0],[19.0,31.0]],"sample_size":100.0},"sample_log_likelihood":-63.88790652574118},{"states":{"dysp":["no","yes"]},"conditioning_states":{"bronc":["no","yes"],"either":["no","yes"]},"parameters":[[0.847457627118644,0.15254237288135594],[0.5,0.5],[0.20454545454545456,0.7954545454545454],[0.6666666666666666,0.3333333333333333]],"sample_statistics":{"sample_conditional_counts":[[49.0,8.0],[0.0,0.0],[8.0,34.0],[1.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-50.786515133256536},{"states":{"either":["no","yes"]},"conditioning_states":{"lung":["no","yes"],"tub":["no","yes"]},"parameters":[[0.9900990099009901,0.009900990099009901],[0.5,0.5],[0.3333333333333333,0.6666666666666666],[0.5,0.5]],"sample_statistics":{"sample_conditional_counts":[[99.0,0.0],[0.0,0.0],[0.0,1.0],[0.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-10.29228482928229},{"states":{"lung":["no","yes"]},"conditioning_states":{"smoke":["no","yes"]},"parameters":[[0.9807692307692307,0.019230769230769232],[0.9615384615384616,0.038461538461538464]],"sample_statistics":{"sample_conditional_counts":[[50.0,0.0],[49.0,1.0]],"sample_size":100.0},"sample_log_likelihood":-13.418794831000639},{"states":{"smoke":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]],"sample_statistics":{"sample_conditional_counts":[[50.0,50.0]],"sample_size":100.0},"sample_log_likelihood":-70.70101241711441},{"states":{"tub":["no","yes"]},"conditioning_states":{"asia":["no","yes"]},"parameters":[[0.99,0.01],[0.75,0.25]],"sample_statistics":{"sample_conditional_counts":[[98.0,0.0],[2.0,0.0]],"sample_size":100.0},"sample_log_likelihood":-7.849494013959967},{"states":{"xray":["no","yes"]},"conditioning_states":{"either":["no","yes"]},"parameters":[[0.900990099009901,0.09900990099009901],[0.3333333333333333,0.6666666666666666]],"sample_statistics":{"sample_conditional_counts":[[90.0,9.0],[0.0,1.0]],"sample_size":100.0},"sample_log_likelihood":-34.52264868287783}]}"#
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
                    let json = true_model.to_json();
                    // Assert the JSON string is correct.
                    assert_eq!(
                        json,
                        r#"{"graph":{"labels":["aceB","asnA","atpD","atpG","b1191","b1583","b1963","cchB","cspA","cspG","dnaG","dnaJ","dnaK","eutG","fixC","flgD","folK","ftsJ","gltA","hupB","ibpB","icdA","lacA","lacY","lacZ","lpdA","mopB","nmpC","nuoM","pspA","pspB","sucA","sucD","tnaA","yaeM","yceP","ycgX","yecO","yedE","yfaD","yfiA","ygbD","ygcE","yhdM","yheI","yjbO"],"edges":[["asnA","icdA"],["asnA","lacA"],["asnA","lacY"],["asnA","lacZ"],["atpD","yheI"],["b1191","fixC"],["b1191","tnaA"],["b1191","ygcE"],["cspA","hupB"],["cspA","yfiA"],["cspG","cspA"],["cspG","lacA"],["cspG","lacY"],["cspG","pspA"],["cspG","pspB"],["cspG","yaeM"],["cspG","yecO"],["cspG","yedE"],["dnaK","mopB"],["eutG","ibpB"],["eutG","lacY"],["eutG","sucA"],["eutG","yceP"],["eutG","yfaD"],["fixC","cchB"],["fixC","tnaA"],["fixC","yceP"],["fixC","ycgX"],["fixC","ygbD"],["fixC","yjbO"],["icdA","aceB"],["lacA","b1583"],["lacA","lacY"],["lacA","lacZ"],["lacA","yaeM"],["lacY","lacZ"],["lacY","nuoM"],["lacZ","b1583"],["lacZ","mopB"],["lacZ","yaeM"],["mopB","ftsJ"],["pspA","nmpC"],["pspB","pspA"],["sucA","atpD"],["sucA","atpG"],["sucA","dnaJ"],["sucA","flgD"],["sucA","gltA"],["sucA","sucD"],["sucA","tnaA"],["sucA","yfaD"],["sucA","ygcE"],["sucA","yhdM"],["yceP","b1583"],["yceP","ibpB"],["yceP","yfaD"],["ycgX","dnaG"],["yedE","lpdA"],["yedE","pspA"],["yedE","pspB"],["yedE","yheI"],["yfiA","hupB"],["ygcE","asnA"],["ygcE","atpD"],["ygcE","icdA"],["yheI","b1963"],["yheI","dnaG"],["yheI","dnaK"],["yheI","folK"],["yheI","ycgX"]]},"cpds":[{"labels":["aceB"],"conditioning_labels":["icdA"],"parameters":{"coefficients":[[1.0475490588756187]],"intercept":[0.12639725851639128],"covariance":[[0.06638476459436277]]},"sample_statistics":{"sample_response_mean":[-1.6574787713497268],"sample_design_mean":[-1.7029045224677422],"sample_response_covariance":[[488.9498246775694]],"sample_cross_covariance":[[480.418001647654]],"sample_design_covariance":[[479.1587026023581]],"sample_size":100.0},"sample_log_likelihood":-6.279468435104807},{"labels":["asnA"],"conditioning_labels":["ygcE"],"parameters":{"coefficients":[[0.8059089956395135]],"intercept":[0.3525760466069605],"covariance":[[0.11294485408364267]]},"sample_statistics":{"sample_response_mean":[2.162847289590218],"sample_design_mean":[2.2462477187597987],"sample_response_covariance":[[713.743151856857]],"sample_cross_covariance":[[777.0006975462043]],"sample_design_covariance":[[865.8587846565139]],"sample_size":100.0},"sample_log_likelihood":-32.851073503080094},{"labels":["atpD"],"conditioning_labels":["sucA","ygcE"],"parameters":{"coefficients":[[0.28643266595623845,-0.735565517817577]],"intercept":[-0.015322489040600074],"covariance":[[0.39395260332430665]]},"sample_statistics":{"sample_response_mean":[-2.0144327019390293],"sample_design_mean":[-1.2109228022034293,2.2462477187597987],"sample_response_covariance":[[612.6915056718626]],"sample_cross_covariance":[[215.0360652771003,-691.4629183469452]],"sample_design_covariance":[[285.8956721206191,-178.4895667475048],[-178.4895667475048,865.8587846565139]],"sample_size":100.0},"sample_log_likelihood":-95.31761966788905},{"labels":["atpG"],"conditioning_labels":["sucA"],"parameters":{"coefficients":[[0.5515556960136763]],"intercept":[-0.9034169351332905],"covariance":[[0.4476571785073016]]},"sample_statistics":{"sample_response_mean":[-1.5713083041214342],"sample_design_mean":[-1.2109228022034293],"sample_response_covariance":[[334.03218460549095]],"sample_cross_covariance":[[267.08420308874963]],"sample_design_covariance":[[285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-101.70747501803537},{"labels":["b1191"],"conditioning_labels":[],"parameters":{"coefficients":[[]],"intercept":[1.3122437605052948],"covariance":[[0.7134261073397125]]},"sample_statistics":{"sample_response_mean":[1.3122437605052948],"sample_design_mean":[],"sample_response_covariance":[[243.540979432479]],"sample_cross_covariance":[[]],"sample_design_covariance":[],"sample_size":100.0},"sample_log_likelihood":-125.01003276683016},{"labels":["b1583"],"conditioning_labels":["lacA","lacZ","yceP"],"parameters":{"coefficients":[[-0.45063565074889883,0.5563044090843663,0.19710031961454424]],"intercept":[1.472943687428247],"covariance":[[1.0369497946671025]]},"sample_statistics":{"sample_response_mean":[1.9116823715022206],"sample_design_mean":[1.7278187360929023,1.9535452087463296,0.6625539312848499],"sample_response_covariance":[[483.02238489958074]],"sample_cross_covariance":[[341.4686418905447,403.93117052377534,136.56299420521944]],"sample_design_covariance":[[565.2962396164389,588.6439738853928,72.28842974812163],[588.6439738853928,660.9465581043719,69.81946049854588],[72.28842974812163,69.81946049854588,165.94291205834824]],"sample_size":100.0},"sample_log_likelihood":-143.70802902351866},{"labels":["b1963"],"conditioning_labels":["yheI"],"parameters":{"coefficients":[[1.0141374970741914]],"intercept":[1.0164053253433016],"covariance":[[0.2898679387276002]]},"sample_statistics":{"sample_response_mean":[2.2721679968445785],"sample_design_mean":[1.2382568193407495],"sample_response_covariance":[[811.4556076830273]],"sample_cross_covariance":[[543.8359691803943]],"sample_design_covariance":[[412.1520877141539]],"sample_size":100.0},"sample_log_likelihood":-79.97736114955762},{"labels":["cchB"],"conditioning_labels":["fixC"],"parameters":{"coefficients":[[0.6792525499365439]],"intercept":[0.8411843347069197],"covariance":[[0.5805562274424564]]},"sample_statistics":{"sample_response_mean":[1.9765180923357217],"sample_design_mean":[1.6714457056287317],"sample_response_covariance":[[524.4330481897946]],"sample_cross_covariance":[[441.8324521856039]],"sample_design_covariance":[[443.47725720711827]],"sample_size":100.0},"sample_log_likelihood":-114.70542221202405},{"labels":["cspA"],"conditioning_labels":["cspG"],"parameters":{"coefficients":[[0.03319492093772271]],"intercept":[0.15317826888089794],"covariance":[[1.4948173143698142]]},"sample_statistics":{"sample_response_mean":[0.2214498321817563],"sample_design_mean":[2.0566870283843497],"sample_response_covariance":[[154.50965702863564]],"sample_cross_covariance":[[49.27848452980071]],"sample_design_covariance":[[535.4586835551238]],"sample_size":100.0},"sample_log_likelihood":-161.99405340196492},{"labels":["cspG"],"conditioning_labels":[],"parameters":{"coefficients":[[]],"intercept":[2.0566870283843497],"covariance":[[1.124625302826791]]},"sample_statistics":{"sample_response_mean":[2.0566870283843497],"sample_design_mean":[],"sample_response_covariance":[[535.4586835551238]],"sample_cross_covariance":[[]],"sample_design_covariance":[],"sample_size":100.0},"sample_log_likelihood":-147.76634912167904},{"labels":["dnaG"],"conditioning_labels":["ycgX","yheI"],"parameters":{"coefficients":[[0.5204635421863666,0.30719423922112343]],"intercept":[0.05645977566458571],"covariance":[[0.12270799847537575]]},"sample_statistics":{"sample_response_mean":[1.1030932185425857],"sample_design_mean":[1.2801051895037088,1.2382568193407495],"sample_response_covariance":[[347.8414016049302]],"sample_cross_covariance":[[416.0792719339714,367.156854167185]],"sample_design_covariance":[[520.6901280438634,448.744086730467],[448.744086730467,412.1520877141539]],"sample_size":100.0},"sample_log_likelihood":-36.9964662133927},{"labels":["dnaJ"],"conditioning_labels":["sucA"],"parameters":{"coefficients":[[-0.8359781178101945]],"intercept":[0.2257263073541791],"covariance":[[0.6353063605639829]]},"sample_statistics":{"sample_response_mean":[1.2380312723536484],"sample_design_mean":[-1.2109228022034293],"sample_response_covariance":[[314.12752670094653]],"sample_cross_covariance":[[-266.33623913271117]],"sample_design_covariance":[[285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-119.2114563777868},{"labels":["dnaK"],"conditioning_labels":["yheI"],"parameters":{"coefficients":[[1.0877465035358587]],"intercept":[-0.2887874593966264],"covariance":[[0.20886033246118188]]},"sample_statistics":{"sample_response_mean":[1.0581220663207072],"sample_design_mean":[1.2382568193407495],"sample_response_covariance":[[439.08697781206763]],"sample_cross_covariance":[[412.5576882422792]],"sample_design_covariance":[[412.1520877141539]],"sample_size":100.0},"sample_log_likelihood":-63.589377513348644},{"labels":["eutG"],"conditioning_labels":[],"parameters":{"coefficients":[[]],"intercept":[1.1718273863624524],"covariance":[[0.7122958210686167]]},"sample_statistics":{"sample_response_mean":[1.1718273863624524],"sample_design_mean":[],"sample_response_covariance":[[208.5475244497673]],"sample_cross_covariance":[[]],"sample_design_covariance":[],"sample_size":100.0},"sample_log_likelihood":-124.9307545790314},{"labels":["fixC"],"conditioning_labels":["b1191"],"parameters":{"coefficients":[[0.8822513036480213]],"intercept":[0.5137169372189536],"covariance":[[1.0857342274918818]]},"sample_statistics":{"sample_response_mean":[1.6714457056287317],"sample_design_mean":[1.3122437605052948],"sample_response_covariance":[[443.47725720711827]],"sample_cross_covariance":[[282.2765311491667]],"sample_design_covariance":[[243.540979432479]],"sample_size":100.0},"sample_log_likelihood":-146.00667659521858},{"labels":["flgD"],"conditioning_labels":["sucA"],"parameters":{"coefficients":[[0.5816448392004009]],"intercept":[-0.5278194457508172],"covariance":[[0.4020365800037357]]},"sample_statistics":{"sample_response_mean":[-1.2321464443225296],"sample_design_mean":[-1.2109228022034293],"sample_response_covariance":[[239.13606231943294]],"sample_cross_covariance":[[230.20460246929207]],"sample_design_covariance":[[285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-96.33324334703003},{"labels":["folK"],"conditioning_labels":["yheI"],"parameters":{"coefficients":[[0.8119782026137856]],"intercept":[0.5791185328420407],"covariance":[[0.12397304773370223]]},"sample_statistics":{"sample_response_mean":[1.5845560793846056],"sample_design_mean":[1.2382568193407495],"sample_response_covariance":[[434.12405220942]],"sample_cross_covariance":[[406.36825863548466]],"sample_design_covariance":[[412.1520877141539]],"sample_size":100.0},"sample_log_likelihood":-37.50929862102599},{"labels":["ftsJ"],"conditioning_labels":["mopB"],"parameters":{"coefficients":[[0.9276519981172759]],"intercept":[0.6502761668669291],"covariance":[[0.16365302880449917]]},"sample_statistics":{"sample_response_mean":[1.5035698631381225],"sample_design_mean":[0.9198424603224085],"sample_response_covariance":[[514.1115336149899]],"sample_cross_covariance":[[431.16671642436893]],"sample_design_covariance":[[400.31343031229227]],"sample_size":100.0},"sample_log_likelihood":-51.39351480108202},{"labels":["gltA"],"conditioning_labels":["sucA"],"parameters":{"coefficients":[[0.3509304181067779]],"intercept":[-1.07348137374885],"covariance":[[0.56140498045523]]},"sample_statistics":{"sample_response_mean":[-1.4984310190211305],"sample_design_mean":[-1.2109228022034293],"sample_response_covariance":[[297.8204989056106]],"sample_cross_covariance":[[230.31979507352156]],"sample_design_covariance":[[285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-113.02821614034822},{"labels":["hupB"],"conditioning_labels":["cspA","yfiA"],"parameters":{"coefficients":[[-0.2693933845193372,1.352667814742886]],"intercept":[-0.23179920868673176],"covariance":[[0.12725149564801796]]},"sample_statistics":{"sample_response_mean":[-1.6269336871398357],"sample_design_mean":[0.2214498321817563,-0.9872914429580538],"sample_response_covariance":[[459.058293035415]],"sample_cross_covariance":[[96.91455803820838,321.38643753113524]],"sample_design_covariance":[[154.50965702863564,106.21353262533091],[106.21353262533091,241.8290133612146]],"sample_size":100.0},"sample_log_likelihood":-38.81435982038843},{"labels":["ibpB"],"conditioning_labels":["eutG","yceP"],"parameters":{"coefficients":[[1.6084065072559937,0.06937306605095948]],"intercept":[-0.6703802035960995],"covariance":[[0.4134689052320351]]},"sample_statistics":{"sample_response_mean":[1.2603579876473991],"sample_design_mean":[1.1718273863624524,0.6625539312848499],"sample_response_covariance":[[401.80578083662436]],"sample_cross_covariance":[[267.4660899188806,212.7136090978042]],"sample_design_covariance":[[208.5475244497673,152.7088726174948],[152.7088726174948,165.94291205834824]],"sample_size":100.0},"sample_log_likelihood":-97.7352050054098},{"labels":["icdA"],"conditioning_labels":["asnA","ygcE"],"parameters":{"coefficients":[[0.41190378005541434,-0.9806238799393189]],"intercept":[-0.3910653434574827],"covariance":[[0.35232012467775975]]},"sample_statistics":{"sample_response_mean":[-1.7029045224677422],"sample_design_mean":[2.162847289590218,2.2462477187597987],"sample_response_covariance":[[479.1587026023581]],"sample_cross_covariance":[[-552.5333983197781,-616.8752400271759]],"sample_design_covariance":[[713.743151856857,777.0006975462043],[777.0006975462043,865.8587846565139]],"sample_size":100.0},"sample_log_likelihood":-89.73309974189681},{"labels":["lacA"],"conditioning_labels":["asnA","cspG"],"parameters":{"coefficients":[[0.22029621423856463,-0.0611337933087201]],"intercept":[1.3770847459140114],"covariance":[[2.545204546686597]]},"sample_statistics":{"sample_response_mean":[1.7278187360929023],"sample_design_mean":[2.162847289590218,2.0566870283843497],"sample_response_covariance":[[565.2962396164389]],"sample_cross_covariance":[[427.6188583602234,349.43542756971533]],"sample_design_covariance":[[713.743151856857,449.15349547337644],[449.15349547337644,535.4586835551238]],"sample_size":100.0},"sample_log_likelihood":-188.60440425805967},{"labels":["lacY"],"conditioning_labels":["asnA","cspG","eutG","lacA"],"parameters":{"coefficients":[[-0.22506267264184268,-0.28093694179241663,0.301656524081352,1.0474593625609612]],"intercept":[0.05661261133823059],"covariance":[[0.06666340893669258]]},"sample_statistics":{"sample_response_mean":[1.1553463439706482],"sample_design_mean":[2.162847289590218,2.0566870283843497,1.1718273863624524,1.7278187360929023],"sample_response_covariance":[[425.4649920936817]],"sample_cross_covariance":[[239.7745226222894,198.6897407454274,134.01193622469856,459.79439319312564]],"sample_design_covariance":[[713.743151856857,449.15349547337644,220.242056686817,427.6188583602234],[449.15349547337644,535.4586835551238,240.48775189452138,349.43542756971533],[220.242056686817,240.48775189452138,208.5475244497673,173.37030915767906],[427.6188583602234,349.43542756971533,173.37030915767906,565.2962396164389]],"sample_size":100.0},"sample_log_likelihood":-6.488899908177226},{"labels":["lacZ"],"conditioning_labels":["asnA","lacA","lacY"],"parameters":{"coefficients":[[-0.0016215875594956762,1.357207108924581,-0.42608441527287005]],"intercept":[0.10431965485422823],"covariance":[[0.3608516263465003]]},"sample_statistics":{"sample_response_mean":[1.9535452087463296],"sample_design_mean":[2.162847289590218,1.7278187360929023,1.1553463439706482],"sample_response_covariance":[[660.9465581043719]],"sample_cross_covariance":[[499.608518467378,588.6439738853928,454.41593451110464]],"sample_design_covariance":[[713.743151856857,427.6188583602234,239.77452262228948],[427.6188583602234,565.2962396164389,459.7943931931257],[239.77452262228948,459.7943931931257,425.4649920936817]],"sample_size":100.0},"sample_log_likelihood":-90.92943269615014},{"labels":["lpdA"],"conditioning_labels":["yedE"],"parameters":{"coefficients":[[0.9359620466131243]],"intercept":[-0.21231730635069557],"covariance":[[0.12394242915764025]]},"sample_statistics":{"sample_response_mean":[-1.5152287232265693],"sample_design_mean":[-1.392055822766098],"sample_response_covariance":[[318.0856653065768]],"sample_cross_covariance":[[292.23459998031774]],"sample_design_covariance":[[280.65117241975616]],"sample_size":100.0},"sample_log_likelihood":-37.49694821168421},{"labels":["mopB"],"conditioning_labels":["dnaK","lacZ"],"parameters":{"coefficients":[[0.9490783290091981,-0.07638263369729559]],"intercept":[0.06481866582176954],"covariance":[[0.2744134355878339]]},"sample_statistics":{"sample_response_mean":[0.9198424603224085],"sample_design_mean":[1.0581220663207072,1.9535452087463296],"sample_response_covariance":[[400.31343031229227]],"sample_cross_covariance":[[403.5692383152964,210.89908675522796]],"sample_design_covariance":[[439.08697781206763,262.06615259112846],[262.06615259112846,660.9465581043719]],"sample_size":100.0},"sample_log_likelihood":-77.23788227963895},{"labels":["nmpC"],"conditioning_labels":["pspA"],"parameters":{"coefficients":[[-0.7647672528051565]],"intercept":[0.21541720797735286],"covariance":[[0.36749509406156206]]},"sample_statistics":{"sample_response_mean":[-0.7530250508855534],"sample_design_mean":[1.2663228653040157],"sample_response_covariance":[[159.7624814450624]],"sample_cross_covariance":[[-182.06117865020124]],"sample_design_covariance":[[273.73027739683187]],"sample_size":100.0},"sample_log_likelihood":-91.84158782493232},{"labels":["nuoM"],"conditioning_labels":["lacY"],"parameters":{"coefficients":[[0.3987809571622565]],"intercept":[-2.0142680234514367],"covariance":[[0.7595458338525302]]},"sample_statistics":{"sample_response_mean":[-1.553537902548908],"sample_design_mean":[1.1553463439706482],"sample_response_covariance":[[363.7354633829251]],"sample_cross_covariance":[[-63.050382881009845]],"sample_design_covariance":[[425.4649920936817]],"sample_size":100.0},"sample_log_likelihood":-128.14212275225705},{"labels":["pspA"],"conditioning_labels":["cspG","pspB","yedE"],"parameters":{"coefficients":[[0.08891935514392953,0.06821969439369431,-0.8903006728512547]],"intercept":[-0.2731200857131635],"covariance":[[0.15315446014966141]]},"sample_statistics":{"sample_response_mean":[1.2663228653040157],"sample_design_mean":[2.0566870283843497,1.7182051621155994,-1.392055822766098],"sample_response_covariance":[[273.73027739683187]],"sample_cross_covariance":[[347.69818773068664,340.38276134369937,-268.2944684009419]],"sample_design_covariance":[[535.4586835551238,465.50009683251915,-364.4853207232856],[465.50009683251915,471.6669392440121,-352.39943329850576],[-364.4853207232856,-352.39943329850576,280.65117241975616]],"sample_size":100.0},"sample_log_likelihood":-48.07843715175956},{"labels":["pspB"],"conditioning_labels":["cspG","yedE"],"parameters":{"coefficients":[[0.24286761613794192,-1.084705849911238]],"intercept":[-0.29126860796732323],"covariance":[[0.2640823618719071]]},"sample_statistics":{"sample_response_mean":[1.7182051621155994],"sample_design_mean":[2.0566870283843497,-1.392055822766098],"sample_response_covariance":[[471.6669392440121]],"sample_cross_covariance":[[465.50009683251915,-352.39943329850576]],"sample_design_covariance":[[535.4586835551238,-364.4853207232856],[-364.4853207232856,280.65117241975616]],"sample_size":100.0},"sample_log_likelihood":-75.31914093532063},{"labels":["sucA"],"conditioning_labels":["eutG"],"parameters":{"coefficients":[[-0.9368572628997918]],"intercept":[-0.11308780442488531],"covariance":[[0.7674395555989045]]},"sample_statistics":{"sample_response_mean":[-1.2109228022034293],"sample_design_mean":[1.1718273863624524],"sample_response_covariance":[[285.8956721206191]],"sample_cross_covariance":[[-208.63120156940457]],"sample_design_covariance":[[208.5475244497673]],"sample_size":100.0},"sample_log_likelihood":-128.65907544336176},{"labels":["sucD"],"conditioning_labels":["sucA"],"parameters":{"coefficients":[[0.7077324162360351]],"intercept":[-0.5039968782266315],"covariance":[[0.38849336986443944]]},"sample_statistics":{"sample_response_mean":[-1.361006198905375],"sample_design_mean":[-1.2109228022034293],"sample_response_covariance":[[293.8375299469048]],"sample_cross_covariance":[[263.36776602974834]],"sample_design_covariance":[[285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-94.6198945551643},{"labels":["tnaA"],"conditioning_labels":["b1191","fixC","sucA"],"parameters":{"coefficients":[[-0.6711719350611629,-0.20780819307222825,0.09795729650674465]],"intercept":[-0.3834518691741118],"covariance":[[0.09943971273540747]]},"sample_statistics":{"sample_response_mean":[-1.7301518890716494],"sample_design_mean":[1.3122437605052948,1.6714457056287317,-1.2109228022034293],"sample_response_covariance":[[367.91694617394484]],"sample_cross_covariance":[[-288.1682252938474,-366.2246029747326,225.76244660376972]],"sample_design_covariance":[[243.540979432479,282.2765311491667,-160.60821664049692],[282.2765311491667,443.47725720711827,-209.46282287510897],[-160.60821664049692,-209.46282287510897,285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-26.48366729010776},{"labels":["yaeM"],"conditioning_labels":["cspG","lacA","lacZ"],"parameters":{"coefficients":[[1.537540236758397,-0.4872006208761242,0.4381803248265501]],"intercept":[0.30724048017886396],"covariance":[[0.5229724689065267]]},"sample_statistics":{"sample_response_mean":[3.483690253884816],"sample_design_mean":[2.0566870283843497,1.7278187360929023,1.9535452087463296],"sample_response_covariance":[[1563.5088305092543]],"sample_cross_covariance":[[896.5567801551991,572.87614461546,695.5857874587421]],"sample_design_covariance":[[535.4586835551238,349.43542756971533,411.52672792674593],[349.43542756971533,565.2962396164389,588.6439738853928],[411.52672792674593,588.6439738853928,660.9465581043719]],"sample_size":100.0},"sample_log_likelihood":-109.4825304696231},{"labels":["yceP"],"conditioning_labels":["eutG","fixC"],"parameters":{"coefficients":[[1.0922854757847726,-0.37463025055150323]],"intercept":[0.008758020517237841],"covariance":[[0.20003072659030097]]},"sample_statistics":{"sample_response_mean":[0.6625539312848499],"sample_design_mean":[1.1718273863624524,1.6714457056287317],"sample_response_covariance":[[165.94291205834824]],"sample_cross_covariance":[[152.7088726174948,57.23539475274568]],"sample_design_covariance":[[208.5475244497673,203.16257981584056],[203.16257981584056,443.47725720711827]],"sample_size":100.0},"sample_log_likelihood":-61.42963875632083},{"labels":["ycgX"],"conditioning_labels":["fixC","yheI"],"parameters":{"coefficients":[[-0.22279771458222708,1.1952212390348245]],"intercept":[0.17250862291029656],"covariance":[[0.24633270678006455]]},"sample_statistics":{"sample_response_mean":[1.2801051895037088],"sample_design_mean":[1.6714457056287317,1.2382568193407495],"sample_response_covariance":[[520.6901280438634]],"sample_cross_covariance":[[279.9603418051363,448.744086730467]],"sample_design_covariance":[[443.47725720711827,292.7760736312909],[292.7760736312909,412.1520877141539]],"sample_size":100.0},"sample_log_likelihood":-71.84024380930299},{"labels":["yecO"],"conditioning_labels":["cspG"],"parameters":{"coefficients":[[0.7270120144420353]],"intercept":[0.4577544532590656],"covariance":[[0.17550816305936862]]},"sample_statistics":{"sample_response_mean":[1.9529906328415751],"sample_design_mean":[2.0566870283843497],"sample_response_covariance":[[458.4097307944357]],"sample_cross_covariance":[[483.43066080219995]],"sample_design_covariance":[[535.4586835551238]],"sample_size":100.0},"sample_log_likelihood":-54.890367120511044},{"labels":["yedE"],"conditioning_labels":["cspG"],"parameters":{"coefficients":[[-0.6951915911884233]],"intercept":[0.03773570507300805],"covariance":[[0.3251706313861892]]},"sample_statistics":{"sample_response_mean":[-1.392055822766098],"sample_design_mean":[2.0566870283843497],"sample_response_covariance":[[280.65117241975616]],"sample_cross_covariance":[[-364.4853207232856]],"sample_design_covariance":[[535.4586835551238]],"sample_size":100.0},"sample_log_likelihood":-85.72359258160816},{"labels":["yfaD"],"conditioning_labels":["eutG","sucA","yceP"],"parameters":{"coefficients":[[0.14074250597522953,-0.23664755327202786,0.4325020200146471]],"intercept":[0.23555557370503588],"covariance":[[0.17960385092380335]]},"sample_statistics":{"sample_response_mean":[0.9735993286241835],"sample_design_mean":[1.1718273863624524,-1.2109228022034293,0.6625539312848499],"sample_response_covariance":[[173.28101314205725]],"sample_cross_covariance":[[172.3735077026075,-191.74032292114708,145.0901789674359]],"sample_design_covariance":[[208.5475244497673,-208.63120156940457,152.7088726174948],[-208.63120156940457,285.8956721206191,-153.05494283805606],[152.7088726174948,-153.05494283805606,165.94291205834824]],"sample_size":100.0},"sample_log_likelihood":-56.0437692367361},{"labels":["yfiA"],"conditioning_labels":["cspA"],"parameters":{"coefficients":[[0.8560978910609188]],"intercept":[-1.1768741772646498],"covariance":[[0.3470805161336987]]},"sample_statistics":{"sample_response_mean":[-0.9872914429580538],"sample_design_mean":[0.2214498321817563],"sample_response_covariance":[[241.8290133612146]],"sample_cross_covariance":[[106.21353262533091]],"sample_design_covariance":[[154.50965702863564]],"sample_size":100.0},"sample_log_likelihood":-88.98392877127748},{"labels":["ygbD"],"conditioning_labels":["fixC"],"parameters":{"coefficients":[[0.649268337497459]],"intercept":[1.332215814419119],"covariance":[[0.508960938183459]]},"sample_statistics":{"sample_response_mean":[2.417432588929953],"sample_design_mean":[1.6714457056287317],"sample_response_covariance":[[704.4721414453525]],"sample_cross_covariance":[[510.60838170295074]],"sample_design_covariance":[[443.47725720711827]],"sample_size":100.0},"sample_log_likelihood":-108.12465293803548},{"labels":["ygcE"],"conditioning_labels":["b1191","sucA"],"parameters":{"coefficients":[[1.9466101799120226,0.6953346665799434]],"intercept":[0.5338172589583299],"covariance":[[0.2824330974022706]]},"sample_statistics":{"sample_response_mean":[2.2462477187597987],"sample_design_mean":[1.3122437605052948,-1.2109228022034293],"sample_response_covariance":[[865.8587846565139]],"sample_cross_covariance":[[432.4527257530996,-178.4895667475048]],"sample_design_covariance":[[243.540979432479,-160.60821664049692],[-160.60821664049692,285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-78.67817433059155},{"labels":["yhdM"],"conditioning_labels":["sucA"],"parameters":{"coefficients":[[-0.839336878438573]],"intercept":[0.16555251246828506],"covariance":[[0.756431405330056]]},"sample_statistics":{"sample_response_mean":[1.1819246772998009],"sample_design_mean":[-1.2109228022034293],"sample_response_covariance":[[313.44610861290755]],"sample_cross_covariance":[[-260.0099122278096]],"sample_design_covariance":[[285.8956721206191]],"sample_size":100.0},"sample_log_likelihood":-127.93668214117493},{"labels":["yheI"],"conditioning_labels":["atpD","yedE"],"parameters":{"coefficients":[[-1.0584110682580954,0.3423499711281579]],"intercept":[-0.41727077791982237],"covariance":[[0.13042092758124368]]},"sample_statistics":{"sample_response_mean":[1.2382568193407495],"sample_design_mean":[-2.0144327019390293,-1.392055822766098],"sample_response_covariance":[[412.1520877141539]],"sample_cross_covariance":[[-470.22906047604977,-137.04341346002244]],"sample_design_covariance":[[612.6915056718626,275.13956021040036],[275.13956021040036,280.65117241975616]],"sample_size":100.0},"sample_log_likelihood":-40.04444558146152},{"labels":["yjbO"],"conditioning_labels":["fixC"],"parameters":{"coefficients":[[-0.06630020652652417]],"intercept":[1.6126819624745836],"covariance":[[1.575703613631242]]},"sample_statistics":{"sample_response_mean":[1.5018647669935268],"sample_design_mean":[1.6714457056287317],"sample_response_covariance":[[383.8514948049089]],"sample_cross_covariance":[[240.14840032965742]],"sample_design_covariance":[[443.47725720711827]],"sample_size":100.0},"sample_log_likelihood":-164.62894888711773}]}"#
                    );
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
                        r#"{"name":"eating","description":"See: U. Nodelman, C.R. Shelton, and D. Koller (2003). \"Learning Continuous Time Bayesian Networks.\" Proc. Nineteenth Conference on Uncertainty in Artificial Intelligence (UAI) (pp. 451-458).","initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[]},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]}]},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]]},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.1,0.1],[10.0,-10.0]],[[-2.0,2.0],[0.1,-0.1]]]}]}"#
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
                        r#"{"initial_distribution":{"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[]},"cpds":[{"states":{"Eating":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]},{"states":{"Hungry":["no","yes"]},"conditioning_states":{},"parameters":[[0.5,0.5]]}]},"graph":{"labels":["Eating","FullStomach","Hungry"],"edges":[["Eating","FullStomach"],["FullStomach","Hungry"],["Hungry","Eating"]]},"cims":[{"states":{"Eating":["no","yes"]},"conditioning_states":{"Hungry":["no","yes"]},"parameters":[[[-0.09761971408804883,0.09761971408804883],[10.01391697001624,-10.01391697001624]],[[-2.0510219056544603,2.0510219056544603],[0.014896945496758228,-0.014896945496758228]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,129.0],[162.0,0.0]],[[0.0,29.0],[0.0,0.0]]],"sample_conditional_times":[[1326.0763089942725,15.727416353316986],[13.883074075743158,33.06392759232465]],"sample_size":320.0},"sample_log_likelihood":-2709.4167395271697},{"states":{"FullStomach":["no","yes"]},"conditioning_states":{"Eating":["no","yes"]},"parameters":[[[-0.10447734377780926,0.10447734377780926],[10.74241980384082,-10.74241980384082]],[[-1.9188634871164392,1.9188634871164392],[0.25871854904000646,-0.25871854904000646]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,138.0],[164.0,0.0]],[[0.0,32.0],[8.0,0.0]]],"sample_conditional_times":[[1325.1462596766082,14.813123393407606],[16.437108980503446,32.35423496513819]],"sample_size":342.0},"sample_log_likelihood":-2882.518082085844},{"states":{"Hungry":["no","yes"]},"conditioning_states":{"FullStomach":["no","yes"]},"parameters":[[[-0.10067226843575305,0.10067226843575305],[9.849547083862912,-9.849547083862912]],[[-1.7046905133498722,1.7046905133498722],[0.17488754102896195,-0.17488754102896195]]],"sample_statistics":{"sample_conditional_counts":[[[0.0,133.0],[162.0,0.0]],[[0.0,28.0],[5.0,0.0]]],"sample_conditional_times":[[1325.5851481179936,15.998220539118314],[16.2185772295963,30.9487811289495]],"sample_size":328.0},"sample_log_likelihood":-2777.550074204407}]}"#
                    );
                }
            }
        }
    }
}

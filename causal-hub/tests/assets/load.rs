#[cfg(test)]
mod tests {
    use causal_hub::{assets::*, distributions::CPD, graphs::Graph, models::BN};
    use dry::macro_for;
    use ndarray::prelude::*;
    use paste::paste;

    macro_for!(
        $bn in [
            alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
            hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
            pigs, sachs, survey, water, win95pts
        ] {
        paste! {
            #[test]
            fn [<test_load_ $bn>]() {
                let _ = [<load_ $bn>]();
            }
        }
    });

    #[test]
    fn test_load_asia_full() {
        // Load BN.
        let bn = load_asia();

        // Check labels.
        assert!(bn.labels().into_iter().eq([
            "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray"
        ]));

        // Check graph structure.
        assert_eq!(bn.graph().vertices().count(), 8);
        assert!(bn.graph().has_edge(0, 6));
        assert!(bn.graph().has_edge(1, 2));
        assert!(bn.graph().has_edge(3, 2));
        assert!(bn.graph().has_edge(3, 7));
        assert!(bn.graph().has_edge(4, 3));
        assert!(bn.graph().has_edge(5, 1));
        assert!(bn.graph().has_edge(5, 4));
        assert!(bn.graph().has_edge(6, 3));

        // Check CPDs.
        assert_eq!(bn.cpds()[0].label(), "asia");
        assert_eq!(bn.cpds()[1].label(), "bronc");
        assert_eq!(bn.cpds()[2].label(), "dysp");
        assert_eq!(bn.cpds()[3].label(), "either");
        assert_eq!(bn.cpds()[4].label(), "lung");
        assert_eq!(bn.cpds()[5].label(), "smoke");
        assert_eq!(bn.cpds()[6].label(), "tub");
        assert_eq!(bn.cpds()[7].label(), "xray");

        assert!(
            bn.cpds()[0]
                .conditioning_labels()
                .iter()
                .eq(Vec::<&str>::new())
        );
        assert!(bn.cpds()[1].conditioning_labels().iter().eq(["smoke"]));
        assert!(
            bn.cpds()[2]
                .conditioning_labels()
                .iter()
                .eq(["bronc", "either"])
        );
        assert!(
            bn.cpds()[3]
                .conditioning_labels()
                .iter()
                .eq(["lung", "tub"])
        );
        assert!(bn.cpds()[4].conditioning_labels().iter().eq(["smoke"]));
        assert!(
            bn.cpds()[5]
                .conditioning_labels()
                .iter()
                .eq(Vec::<&str>::new())
        );
        assert!(bn.cpds()[6].conditioning_labels().iter().eq(["asia"]));
        assert!(bn.cpds()[7].conditioning_labels().iter().eq(["either"]));

        // Check CPDs states.
        assert!(bn.cpds()[0].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[1].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[2].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[3].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[4].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[5].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[6].states().iter().eq(["no", "yes"]));
        assert!(bn.cpds()[7].states().iter().eq(["no", "yes"]));

        // Check CPDs parameters.
        assert_eq!(
            bn.cpds()[0].parameters(),
            array![
                [0.01, 0.99], //
            ]
        );
        assert_eq!(
            bn.cpds()[1].parameters(),
            array![
                [0.30, 0.70], //
                [0.60, 0.40],
            ]
        );
        assert_eq!(
            bn.cpds()[2].parameters(),
            array![
                [0.10, 0.90], //
                [0.70, 0.30], //
                [0.80, 0.20], //
                [0.90, 0.10],
            ]
        );
        assert_eq!(
            bn.cpds()[3].parameters(),
            array![
                [0.00, 1.00], //
                [1.00, 0.00], //
                [1.00, 0.00], //
                [1.00, 0.00],
            ]
        );
        assert_eq!(
            bn.cpds()[4].parameters(),
            array![
                [0.01, 0.99], //
                [0.10, 0.90],
            ]
        );
        assert_eq!(
            bn.cpds()[5].parameters(),
            array![
                [0.50, 0.50], //
            ]
        );
        assert_eq!(
            bn.cpds()[6].parameters(),
            array![
                [0.01, 0.99], //
                [0.05, 0.95],
            ]
        );
        assert_eq!(
            bn.cpds()[7].parameters(),
            array![
                [0.05, 0.95], //
                [0.98, 0.02],
            ]
        );
    }

    #[test]
    fn test_load_child_full() {
        // Load BN.
        let bn = load_child();

        // Get CPD.
        let cpd = bn.cpds().get("HypDistrib").unwrap();

        // Check cardinality.
        assert_eq!(cpd.cardinality(), 2);
        assert_eq!(cpd.conditioning_cardinality(), array![4, 3]);

        // Check probability values with "." in it.
        assert_eq!(
            cpd.to_string(),
            concat!(
                "-----------------------------------------------------------------\n",
                "|               |               | HypDistrib    |               |\n",
                "| ------------- | ------------- | ------------- | ------------- |\n",
                "| CardiacMixing | DuctFlow      | Equal         | Unequal       |\n",
                "| ------------- | ------------- | ------------- | ------------- |\n",
                "| Complete      | Lt_to_Rt      |      0.950000 |      0.050000 |\n",
                "| Complete      | None          |      0.950000 |      0.050000 |\n",
                "| Complete      | Rt_to_Lt      |      0.050000 |      0.950000 |\n",
                "| Mild          | Lt_to_Rt      |      0.950000 |      0.050000 |\n",
                "| Mild          | None          |      0.950000 |      0.050000 |\n",
                "| Mild          | Rt_to_Lt      |      0.950000 |      0.050000 |\n",
                "| None          | Lt_to_Rt      |      0.950000 |      0.050000 |\n",
                "| None          | None          |      0.950000 |      0.050000 |\n",
                "| None          | Rt_to_Lt      |      0.950000 |      0.050000 |\n",
                "| Transp.       | Lt_to_Rt      |      0.500000 |      0.500000 |\n",
                "| Transp.       | None          |      0.950000 |      0.050000 |\n",
                "| Transp.       | Rt_to_Lt      |      0.500000 |      0.500000 |\n",
                "-----------------------------------------------------------------\n",
            )
        );
    }

    #[test]
    fn test_load_sachs_full() {
        // Load BN.
        let bn = load_sachs();

        // Check probability values with exponential notation.
        assert_eq!(
            bn.cpds()[5].to_string(),
            concat!(
                "--------------------------------------------------------\n",
                "|          |          | PIP2     |          |          |\n",
                "| -------- | -------- | -------- | -------- | -------- |\n",
                "| PIP3     | Plcg     | AVG      | HIGH     | LOW      |\n",
                "| -------- | -------- | -------- | -------- | -------- |\n",
                "| AVG      | AVG      | 0.957165 | 0.042445 | 0.000389 |\n",
                "| AVG      | HIGH     | 0.076728 | 0.391103 | 0.532169 |\n",
                "| AVG      | LOW      | 0.986711 | 0.013270 | 0.000019 |\n",
                "| HIGH     | AVG      | 0.521810 | 0.462455 | 0.015735 |\n",
                "| HIGH     | HIGH     | 0.026417 | 0.052354 | 0.921230 |\n",
                "| HIGH     | LOW      | 0.872401 | 0.120071 | 0.007528 |\n",
                "| LOW      | AVG      | 0.997890 | 0.001055 | 0.001055 |\n",
                "| LOW      | HIGH     | 0.221809 | 0.493649 | 0.284542 |\n",
                "| LOW      | LOW      | 0.996792 | 0.003170 | 0.000039 |\n",
                "--------------------------------------------------------\n",
            )
        );
    }

    #[test]
    fn test_load_eating_json() {
        // Load CTBN.
        let ctbn = load_eating();

        // Serialize CTBN to JSON.
        let json = serde_json::to_string(&ctbn).unwrap();

        // Assert the JSON string is correct.
        assert_eq!(
            json.as_str(),
            r#"{"initial_distribution":{"states":{"Eating":["no","yes"],"FullStomach":["no","yes"],"Hungry":["no","yes"]},"graph":{"labels":["Eating","FullStomach","Hungry"],"adjacency_matrix":{"v":1,"dim":[3,3],"data":[false,false,false,false,false,false,false,false,false]}},"cpds":{"Eating":{"label":"Eating","states":["no","yes"],"cardinality":2,"conditioning_labels":[],"conditioning_states":{},"conditioning_cardinality":{"v":1,"dim":[0],"data":[]},"ravel_multi_index":{"cardinality":{"v":1,"dim":[0],"data":[]},"strides":{"v":1,"dim":[0],"data":[]}},"parameters":{"v":1,"dim":[1,2],"data":[0.5,0.5]},"parameters_size":1,"sample_size":null,"sample_log_likelihood":null},"FullStomach":{"label":"FullStomach","states":["no","yes"],"cardinality":2,"conditioning_labels":[],"conditioning_states":{},"conditioning_cardinality":{"v":1,"dim":[0],"data":[]},"ravel_multi_index":{"cardinality":{"v":1,"dim":[0],"data":[]},"strides":{"v":1,"dim":[0],"data":[]}},"parameters":{"v":1,"dim":[1,2],"data":[0.5,0.5]},"parameters_size":1,"sample_size":null,"sample_log_likelihood":null},"Hungry":{"label":"Hungry","states":["no","yes"],"cardinality":2,"conditioning_labels":[],"conditioning_states":{},"conditioning_cardinality":{"v":1,"dim":[0],"data":[]},"ravel_multi_index":{"cardinality":{"v":1,"dim":[0],"data":[]},"strides":{"v":1,"dim":[0],"data":[]}},"parameters":{"v":1,"dim":[1,2],"data":[0.5,0.5]},"parameters_size":1,"sample_size":null,"sample_log_likelihood":null}},"topological_order":[0,1,2]},"graph":{"labels":["Eating","FullStomach","Hungry"],"adjacency_matrix":{"v":1,"dim":[3,3],"data":[false,true,false,false,false,true,true,false,false]}},"cims":{"Eating":{"label":"Eating","states":["no","yes"],"cardinality":2,"conditioning_labels":["Hungry"],"conditioning_states":{"Hungry":["no","yes"]},"conditioning_cardinality":{"v":1,"dim":[1],"data":[2]},"ravel_multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"parameters":{"v":1,"dim":[2,2,2],"data":[-0.1,0.1,10.0,-10.0,-2.0,2.0,0.1,-0.1]},"parameters_size":4,"sample_size":null,"sample_log_likelihood":null},"FullStomach":{"label":"FullStomach","states":["no","yes"],"cardinality":2,"conditioning_labels":["Eating"],"conditioning_states":{"Eating":["no","yes"]},"conditioning_cardinality":{"v":1,"dim":[1],"data":[2]},"ravel_multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"parameters":{"v":1,"dim":[2,2,2],"data":[-0.1,0.1,10.0,-10.0,-2.0,2.0,0.1,-0.1]},"parameters_size":4,"sample_size":null,"sample_log_likelihood":null},"Hungry":{"label":"Hungry","states":["no","yes"],"cardinality":2,"conditioning_labels":["FullStomach"],"conditioning_states":{"FullStomach":["no","yes"]},"conditioning_cardinality":{"v":1,"dim":[1],"data":[2]},"ravel_multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"parameters":{"v":1,"dim":[2,2,2],"data":[-0.1,0.1,10.0,-10.0,-2.0,2.0,0.1,-0.1]},"parameters_size":4,"sample_size":null,"sample_log_likelihood":null}}}"#
        );
    }
}

#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::*,
        labels,
        models::{BN, CPD, Graph, Labelled},
        states,
    };
    use dry::macro_for;
    use ndarray::prelude::*;
    use paste::paste;

    mod bayesian_network {
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
                    fn [<_load_ $bn>]() {
                        let _ = [<load_ $bn>]();
                    }
                }
            });

            #[test]
            fn load_asia_full() {
                // Load BN.
                let bn = load_asia();

                // Check labels.
                assert_eq!(
                    &labels![
                        "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray"
                    ],
                    bn.labels()
                );

                // Check graph structure.
                assert_eq!(bn.graph().vertices().len(), 8);
                assert!(bn.graph().has_edge(0, 6));
                assert!(bn.graph().has_edge(1, 2));
                assert!(bn.graph().has_edge(3, 2));
                assert!(bn.graph().has_edge(3, 7));
                assert!(bn.graph().has_edge(4, 3));
                assert!(bn.graph().has_edge(5, 1));
                assert!(bn.graph().has_edge(5, 4));
                assert!(bn.graph().has_edge(6, 3));

                // Check CPDs.
                assert_eq!(bn.cpds()[0].labels()[0], "asia");
                assert_eq!(bn.cpds()[1].labels()[0], "bronc");
                assert_eq!(bn.cpds()[2].labels()[0], "dysp");
                assert_eq!(bn.cpds()[3].labels()[0], "either");
                assert_eq!(bn.cpds()[4].labels()[0], "lung");
                assert_eq!(bn.cpds()[5].labels()[0], "smoke");
                assert_eq!(bn.cpds()[6].labels()[0], "tub");
                assert_eq!(bn.cpds()[7].labels()[0], "xray");

                assert_eq!(&labels![], bn.cpds()[0].conditioning_labels());
                assert_eq!(&labels!["smoke"], bn.cpds()[1].conditioning_labels());
                assert_eq!(
                    &labels!["bronc", "either"],
                    bn.cpds()[2].conditioning_labels()
                );
                assert_eq!(&labels!["lung", "tub"], bn.cpds()[3].conditioning_labels());
                assert_eq!(&labels!["smoke"], bn.cpds()[4].conditioning_labels());
                assert_eq!(&labels![], bn.cpds()[5].conditioning_labels());
                assert_eq!(&labels!["asia"], bn.cpds()[6].conditioning_labels());
                assert_eq!(&labels!["either"], bn.cpds()[7].conditioning_labels());

                // Check CPDs states.
                assert_eq!(&states![("asia", ["no", "yes"])], bn.cpds()[0].states());
                assert_eq!(&states![("bronc", ["no", "yes"])], bn.cpds()[1].states());
                assert_eq!(&states![("dysp", ["no", "yes"])], bn.cpds()[2].states());
                assert_eq!(&states![("either", ["no", "yes"])], bn.cpds()[3].states());
                assert_eq!(&states![("lung", ["no", "yes"])], bn.cpds()[4].states());
                assert_eq!(&states![("smoke", ["no", "yes"])], bn.cpds()[5].states());
                assert_eq!(&states![("tub", ["no", "yes"])], bn.cpds()[6].states());
                assert_eq!(&states![("xray", ["no", "yes"])], bn.cpds()[7].states());

                // Check CPDs parameters.
                assert_eq!(
                    bn.cpds()[0].parameters(),
                    array![
                        [0.99, 0.01], //
                    ]
                );
                assert_eq!(
                    bn.cpds()[1].parameters(),
                    array![
                        [0.70, 0.30], //
                        [0.40, 0.60],
                    ]
                );
                assert_eq!(
                    bn.cpds()[2].parameters(),
                    array![
                        [0.90, 0.10], //
                        [0.30, 0.70], //
                        [0.20, 0.80], //
                        [0.10, 0.90],
                    ]
                );
                assert_eq!(
                    bn.cpds()[3].parameters(),
                    array![
                        [1.00, 0.00], //
                        [0.00, 1.00], //
                        [0.00, 1.00], //
                        [0.00, 1.00],
                    ]
                );
                assert_eq!(
                    bn.cpds()[4].parameters(),
                    array![
                        [0.99, 0.01], //
                        [0.90, 0.10],
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
                        [0.99, 0.01], //
                        [0.95, 0.05],
                    ]
                );
                assert_eq!(
                    bn.cpds()[7].parameters(),
                    array![
                        [0.95, 0.05], //
                        [0.02, 0.98],
                    ]
                );
            }

            #[test]
            fn load_child_full() {
                // Load BN.
                let bn = load_child();

                // Get CPD.
                let cpd = bn.cpds().get("HypDistrib").unwrap();

                // Check shape.
                assert_eq!(cpd.shape(), array![2]);
                assert_eq!(cpd.conditioning_shape(), array![4, 3]);

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
                        "| Complete      | Rt_to_Lt      |      0.950000 |      0.050000 |\n",
                        "| Mild          | Lt_to_Rt      |      0.950000 |      0.050000 |\n",
                        "| Mild          | None          |      0.950000 |      0.050000 |\n",
                        "| Mild          | Rt_to_Lt      |      0.500000 |      0.500000 |\n",
                        "| None          | Lt_to_Rt      |      0.950000 |      0.050000 |\n",
                        "| None          | None          |      0.950000 |      0.050000 |\n",
                        "| None          | Rt_to_Lt      |      0.050000 |      0.950000 |\n",
                        "| Transp.       | Lt_to_Rt      |      0.950000 |      0.050000 |\n",
                        "| Transp.       | None          |      0.950000 |      0.050000 |\n",
                        "| Transp.       | Rt_to_Lt      |      0.500000 |      0.500000 |\n",
                        "-----------------------------------------------------------------\n",
                    )
                );
            }

            #[test]
            fn load_sachs_full() {
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
                        "| AVG      | AVG      | 0.042445 | 0.000389 | 0.957165 |\n",
                        "| AVG      | HIGH     | 0.391103 | 0.532169 | 0.076728 |\n",
                        "| AVG      | LOW      | 0.013270 | 0.000019 | 0.986711 |\n",
                        "| HIGH     | AVG      | 0.462455 | 0.015735 | 0.521810 |\n",
                        "| HIGH     | HIGH     | 0.052354 | 0.921230 | 0.026417 |\n",
                        "| HIGH     | LOW      | 0.120071 | 0.007528 | 0.872401 |\n",
                        "| LOW      | AVG      | 0.001055 | 0.001055 | 0.997890 |\n",
                        "| LOW      | HIGH     | 0.493649 | 0.284542 | 0.221809 |\n",
                        "| LOW      | LOW      | 0.003170 | 0.000039 | 0.996792 |\n",
                        "--------------------------------------------------------\n",
                    )
                );
            }
        }
    }
}

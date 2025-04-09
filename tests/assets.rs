#[cfg(test)]
mod tests {
    use causal_hub_next::{
        assets::*, distribution::Distribution, graph::Graph, model::BayesianNetwork,
    };
    use dry::macro_for;
    use ndarray::prelude::*;
    use paste::paste;

    macro_for!(
        $bn in [
            alarm,
            andes,
            asia,
            barley,
            cancer,
            child,
            diabetes,
            earthquake,
            hailfinder,
            hepar2,
            insurance,
            link,
            mildew,
            munin1,
            pathfinder,
            pigs,
            sachs,
            survey,
            water,
            win95pts
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
            "asia", "tub", "smoke", "lung", "bronc", "either", "xray", "dysp"
        ]));

        // Check graph structure.
        assert_eq!(bn.graph().vertices().count(), 8);
        assert!(bn.graph().has_edge(0, 1));
        assert!(bn.graph().has_edge(1, 5));
        assert!(bn.graph().has_edge(2, 3));
        assert!(bn.graph().has_edge(2, 4));
        assert!(bn.graph().has_edge(3, 5));
        assert!(bn.graph().has_edge(5, 6));
        assert!(bn.graph().has_edge(4, 7));
        assert!(bn.graph().has_edge(5, 7));

        // Check CPDs.
        assert!(bn.cpds()[0].labels().iter().eq(["asia"]));
        assert!(bn.cpds()[1].labels().iter().eq(["tub", "asia"]));
        assert!(bn.cpds()[2].labels().iter().eq(["smoke"]));
        assert!(bn.cpds()[3].labels().iter().eq(["lung", "smoke"]));
        assert!(bn.cpds()[4].labels().iter().eq(["bronc", "smoke"]));
        assert!(bn.cpds()[5].labels().iter().eq(["either", "lung", "tub"]));
        assert!(bn.cpds()[6].labels().iter().eq(["xray", "either"]));
        assert!(bn.cpds()[7].labels().iter().eq(["dysp", "bronc", "either"]));

        // Check CPDs states.
        assert!(bn.cpds()[0].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[1].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[2].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[3].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[4].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[5].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[6].states()[0].iter().eq(["yes", "no"]));
        assert!(bn.cpds()[7].states()[0].iter().eq(["yes", "no"]));

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
                [0.05, 0.95], //
                [0.01, 0.99]
            ]
        );
        assert_eq!(
            bn.cpds()[2].parameters(),
            array![
                [0.50, 0.50], //
            ]
        );
        assert_eq!(
            bn.cpds()[3].parameters(),
            array![
                [0.10, 0.90], //
                [0.01, 0.99]
            ]
        );
        assert_eq!(
            bn.cpds()[4].parameters(),
            array![
                [0.60, 0.40], //
                [0.30, 0.70]
            ]
        );
        assert_eq!(
            bn.cpds()[5].parameters(),
            array![
                [1.00, 0.00], //
                [1.00, 0.00], //
                [1.00, 0.00], //
                [0.00, 1.00]
            ]
        );
        assert_eq!(
            bn.cpds()[6].parameters(),
            array![
                [0.98, 0.02], //
                [0.05, 0.95]
            ]
        );
        assert_eq!(
            bn.cpds()[7].parameters(),
            array![
                [0.90, 0.10], //
                [0.80, 0.20], //
                [0.70, 0.30], //
                [0.10, 0.90]
            ]
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
                "| PIP3     | Plcg     | LOW      | AVG      | HIGH     |\n",
                "| -------- | -------- | -------- | -------- | -------- |\n",
                "| LOW      | LOW      | 0.996792 | 0.003170 | 0.000039 |\n",
                "| LOW      | AVG      | 0.997890 | 0.001055 | 0.001055 |\n",
                "| LOW      | HIGH     | 0.221809 | 0.493649 | 0.284542 |\n",
                "| AVG      | LOW      | 0.986711 | 0.013270 | 0.000019 |\n",
                "| AVG      | AVG      | 0.957165 | 0.042445 | 0.000389 |\n",
                "| AVG      | HIGH     | 0.076728 | 0.391103 | 0.532169 |\n",
                "| HIGH     | LOW      | 0.872401 | 0.120071 | 0.007528 |\n",
                "| HIGH     | AVG      | 0.521810 | 0.462455 | 0.015735 |\n",
                "| HIGH     | HIGH     | 0.026417 | 0.052354 | 0.921230 |\n",
                "--------------------------------------------------------\n",
            )
        );
    }
}

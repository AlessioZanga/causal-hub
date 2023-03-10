#[cfg(test)]
mod parser {
    use causal_hub::{
        io::{File, BIF},
        prelude::*,
    };

    #[test]
    fn read() {
        // Test for each scenario.
        std::fs::read_dir("tests/assets/bif")
            .expect("No such file or directory")
            .map(|x| x.unwrap().path())
            .filter(|x| x.extension().unwrap().eq("bif"))
            .map(|x| {
                let bif = BIF::read(&x);
                assert!(bif.is_ok(), "{}: {:?}", x.display(), bif.err());
                bif.unwrap()
            })
            .for_each(|bif| {
                let _: DiscreteBayesianNetwork = bif.into();
            });
    }

    #[test]
    fn into_string() {
        // Define reference.
        let true_bif = concat!(
            "network unknown {\n",
            "}\n",
            "variable asia {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable tub {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable smoke {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable lung {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable bronc {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable either {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable xray {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "variable dysp {\n",
            "  type discrete [ 2 ] { yes, no };\n",
            "}\n",
            "probability ( asia ) {\n",
            "  table 0.01, 0.99;\n",
            "}\n",
            "probability ( tub | asia ) {\n",
            "  (yes) 0.05, 0.95;\n",
            "  (no) 0.01, 0.99;\n",
            "}\n",
            "probability ( smoke ) {\n",
            "  table 0.5, 0.5;\n",
            "}\n",
            "probability ( lung | smoke ) {\n",
            "  (yes) 0.1, 0.9;\n",
            "  (no) 0.01, 0.99;\n",
            "}\n",
            "probability ( bronc | smoke ) {\n",
            "  (yes) 0.6, 0.4;\n",
            "  (no) 0.3, 0.7;\n",
            "}\n",
            "probability ( either | lung, tub ) {\n",
            "  (yes, yes) 1, 0;\n",
            "  (yes, no) 1, 0;\n",
            "  (no, yes) 1, 0;\n",
            "  (no, no) 0, 1;\n",
            "}\n",
            "probability ( xray | either ) {\n",
            "  (yes) 0.98, 0.02;\n",
            "  (no) 0.05, 0.95;\n",
            "}\n",
            "probability ( dysp | bronc, either ) {\n",
            "  (yes, yes) 0.9, 0.1;\n",
            "  (yes, no) 0.8, 0.2;\n",
            "  (no, yes) 0.7, 0.3;\n",
            "  (no, no) 0.1, 0.9;\n",
            "}\n"
        );
        // Test for each scenario.
        let pred_bif = BIF::read("tests/assets/bif/asia.bif").unwrap();
        // Cast to string.
        let pred_bif: String = pred_bif.into();

        assert_eq!(true_bif, pred_bif, "{true_bif}\n{pred_bif}");
    }
}

#[cfg(test)]
mod discrete {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call() {
        // Set true graph.
        let true_g = DiGraph::new(
            [
                "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray",
            ],
            [
                ("bronc", "dysp"),
                ("either", "dysp"),
                ("either", "lung"),
                ("either", "tub"),
                ("either", "xray"),
                ("lung", "smoke"),
                ("lung", "tub"),
                ("smoke", "bronc"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(L!(d), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = HC::new(&s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }

    #[test]
    fn parallel_call() {
        // Set true graph.
        let true_g = DiGraph::new(
            [
                "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray",
            ],
            [
                ("bronc", "dysp"),
                ("either", "dysp"),
                ("either", "lung"),
                ("either", "tub"),
                ("either", "xray"),
                ("lung", "smoke"),
                ("lung", "tub"),
                ("smoke", "bronc"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(L!(d), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = ParallelHC::new(&s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }

    #[test]
    fn with_shuffle() {
        // Set true graph.
        let true_g = DiGraph::new(
            [
                "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray",
            ],
            [
                ("bronc", "smoke"),
                ("dysp", "bronc"),
                ("either", "bronc"),
                ("either", "dysp"),
                ("either", "lung"),
                ("either", "tub"),
                ("lung", "smoke"),
                ("tub", "lung"),
                ("xray", "either"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(L!(d), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = HC::new(&s).with_shuffle(42);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }
}

#[cfg(test)]
mod gaussian {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call() {
        // Set true graph.
        let true_g = DiGraph::new(
            [],
            [
                ("aceB", "icdA"),
                ("asnA", "ygcE"),
                ("asnA", "lacY"),
                ("asnA", "aceB"),
                ("asnA", "lacA"),
                ("asnA", "icdA"),
                ("asnA", "ftsJ"),
                ("atpD", "asnA"),
                ("atpD", "ygcE"),
                ("atpG", "lacY"),
                ("b1191", "tnaA"),
                ("b1191", "fixC"),
                ("cspA", "yfiA"),
                ("cspG", "yecO"),
                ("cspG", "yaeM"),
                ("cspG", "lacY"),
                ("cspG", "lacZ"),
                ("cspG", "hupB"),
                ("cspG", "lacA"),
                ("cspG", "cspA"),
                ("dnaJ", "sucA"),
                ("dnaJ", "cchB"),
                ("dnaK", "ftsJ"),
                ("dnaK", "mopB"),
                ("eutG", "ibpB"),
                ("eutG", "yceP"),
                ("eutG", "yfaD"),
                ("eutG", "lacY"),
                ("fixC", "yceP"),
                ("fixC", "cchB"),
                ("fixC", "ygbD"),
                ("fixC", "ycgX"),
                ("fixC", "yjbO"),
                ("folK", "yheI"),
                ("ftsJ", "mopB"),
                ("hupB", "yfiA"),
                ("hupB", "cspA"),
                ("lacA", "lacY"),
                ("lacA", "lacZ"),
                ("lacA", "yaeM"),
                ("lacA", "b1583"),
                ("lacY", "nuoM"),
                ("lacY", "lacZ"),
                ("lacY", "yaeM"),
                ("lacZ", "ftsJ"),
                ("lacZ", "mopB"),
                ("lacZ", "b1583"),
                ("lpdA", "yedE"),
                ("pspA", "nmpC"),
                ("pspA", "pspB"),
                ("pspA", "cspG"),
                ("pspB", "cspG"),
                ("sucA", "sucD"),
                ("sucA", "flgD"),
                ("sucA", "yhdM"),
                ("sucA", "atpG"),
                ("sucA", "eutG"),
                ("sucA", "b1191"),
                ("sucA", "asnA"),
                ("sucA", "gltA"),
                ("sucA", "yfaD"),
                ("sucA", "tnaA"),
                ("sucA", "ygcE"),
                ("sucA", "fixC"),
                ("sucA", "folK"),
                ("sucA", "yheI"),
                ("sucA", "atpD"),
                ("tnaA", "fixC"),
                ("yaeM", "lacZ"),
                ("yceP", "yfaD"),
                ("yceP", "b1583"),
                ("yceP", "ibpB"),
                ("ycgX", "dnaG"),
                ("yedE", "pspA"),
                ("yedE", "pspB"),
                ("yedE", "atpD"),
                ("yedE", "cspG"),
                ("yedE", "folK"),
                ("yedE", "yheI"),
                ("ygcE", "aceB"),
                ("ygcE", "b1191"),
                ("ygcE", "icdA"),
                ("ygcE", "yaeM"),
                ("yhdM", "folK"),
                ("yheI", "dnaK"),
                ("yheI", "atpD"),
                ("yheI", "dnaG"),
                ("yheI", "b1963"),
                ("yheI", "ycgX"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = ContinuousDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(L!(d), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = HC::new(&s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }

    #[test]
    fn parallel_call() {
        // Set true graph.
        let true_g = DiGraph::new(
            [],
            [
                ("aceB", "icdA"),
                ("asnA", "ygcE"),
                ("asnA", "lacY"),
                ("asnA", "aceB"),
                ("asnA", "lacA"),
                ("asnA", "icdA"),
                ("asnA", "ftsJ"),
                ("atpD", "asnA"),
                ("atpD", "ygcE"),
                ("atpG", "lacY"),
                ("b1191", "tnaA"),
                ("b1191", "fixC"),
                ("cspA", "yfiA"),
                ("cspG", "yecO"),
                ("cspG", "yaeM"),
                ("cspG", "lacY"),
                ("cspG", "lacZ"),
                ("cspG", "hupB"),
                ("cspG", "lacA"),
                ("cspG", "cspA"),
                ("dnaJ", "sucA"),
                ("dnaJ", "cchB"),
                ("dnaK", "ftsJ"),
                ("dnaK", "mopB"),
                ("eutG", "ibpB"),
                ("eutG", "yceP"),
                ("eutG", "yfaD"),
                ("eutG", "lacY"),
                ("fixC", "yceP"),
                ("fixC", "cchB"),
                ("fixC", "ygbD"),
                ("fixC", "ycgX"),
                ("fixC", "yjbO"),
                ("folK", "yheI"),
                ("ftsJ", "mopB"),
                ("hupB", "yfiA"),
                ("hupB", "cspA"),
                ("lacA", "lacY"),
                ("lacA", "lacZ"),
                ("lacA", "yaeM"),
                ("lacA", "b1583"),
                ("lacY", "nuoM"),
                ("lacY", "lacZ"),
                ("lacY", "yaeM"),
                ("lacZ", "ftsJ"),
                ("lacZ", "mopB"),
                ("lacZ", "b1583"),
                ("lpdA", "yedE"),
                ("pspA", "nmpC"),
                ("pspA", "pspB"),
                ("pspA", "cspG"),
                ("pspB", "cspG"),
                ("sucA", "sucD"),
                ("sucA", "flgD"),
                ("sucA", "yhdM"),
                ("sucA", "atpG"),
                ("sucA", "eutG"),
                ("sucA", "b1191"),
                ("sucA", "asnA"),
                ("sucA", "gltA"),
                ("sucA", "yfaD"),
                ("sucA", "tnaA"),
                ("sucA", "ygcE"),
                ("sucA", "fixC"),
                ("sucA", "folK"),
                ("sucA", "yheI"),
                ("sucA", "atpD"),
                ("tnaA", "fixC"),
                ("yaeM", "lacZ"),
                ("yceP", "yfaD"),
                ("yceP", "b1583"),
                ("yceP", "ibpB"),
                ("ycgX", "dnaG"),
                ("yedE", "pspA"),
                ("yedE", "pspB"),
                ("yedE", "atpD"),
                ("yedE", "cspG"),
                ("yedE", "folK"),
                ("yedE", "yheI"),
                ("ygcE", "aceB"),
                ("ygcE", "b1191"),
                ("ygcE", "icdA"),
                ("ygcE", "yaeM"),
                ("yhdM", "folK"),
                ("yheI", "dnaK"),
                ("yheI", "atpD"),
                ("yheI", "dnaG"),
                ("yheI", "b1963"),
                ("yheI", "ycgX"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = ContinuousDataMatrix::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(L!(d), [], []);

        // Initialize score functor.
        let s = BIC::new();

        // Initialize discovery functor.
        let hc = ParallelHC::new(&s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }
}

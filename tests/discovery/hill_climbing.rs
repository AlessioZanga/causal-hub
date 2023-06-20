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
                ("either", "xray"),
                ("lung", "either"),
                ("lung", "smoke"),
                ("smoke", "bronc"),
                ("tub", "either"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataSet::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new(&d);

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
                ("either", "xray"),
                ("lung", "either"),
                ("lung", "smoke"),
                ("smoke", "bronc"),
                ("tub", "either"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataSet::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new(&d);

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
                ("bronc", "dysp"),
                ("bronc", "smoke"),
                ("either", "dysp"),
                ("either", "xray"),
                ("lung", "either"),
                ("smoke", "lung"),
                ("tub", "either"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataSet::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new(&d);

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
                ("asnA", "icdA"),
                ("asnA", "lacA"),
                ("atpD", "ygcE"),
                ("atpG", "lacA"),
                ("b1191", "fixC"),
                ("b1191", "tnaA"),
                ("cspA", "cspG"),
                ("cspA", "yecO"),
                ("cspG", "lacA"),
                ("cspG", "lacZ"),
                ("cspG", "pspA"),
                ("cspG", "pspB"),
                ("cspG", "yaeM"),
                ("cspG", "yedE"),
                ("dnaG", "ycgX"),
                ("dnaG", "yheI"),
                ("dnaJ", "cchB"),
                ("dnaJ", "sucA"),
                ("dnaK", "mopB"),
                ("eutG", "lacA"),
                ("eutG", "lacY"),
                ("eutG", "yceP"),
                ("eutG", "yfaD"),
                ("fixC", "cchB"),
                ("fixC", "eutG"),
                ("fixC", "ibpB"),
                ("fixC", "yceP"),
                ("fixC", "ygbD"),
                ("fixC", "yjbO"),
                ("hupB", "cspA"),
                ("hupB", "yfiA"),
                ("ibpB", "eutG"),
                ("ibpB", "yceP"),
                ("icdA", "aceB"),
                ("lacA", "b1583"),
                ("lacA", "lacZ"),
                ("lacA", "yaeM"),
                ("lacY", "lacA"),
                ("lacY", "lacZ"),
                ("lacY", "nuoM"),
                ("lacY", "yaeM"),
                ("lacZ", "b1583"),
                ("lacZ", "mopB"),
                ("lpdA", "ycgX"),
                ("mopB", "ftsJ"),
                ("pspA", "nmpC"),
                ("pspA", "yedE"),
                ("pspB", "pspA"),
                ("pspB", "yedE"),
                ("sucA", "atpD"),
                ("sucA", "atpG"),
                ("sucA", "b1191"),
                ("sucA", "dnaG"),
                ("sucA", "eutG"),
                ("sucA", "fixC"),
                ("sucA", "flgD"),
                ("sucA", "gltA"),
                ("sucA", "ibpB"),
                ("sucA", "sucD"),
                ("sucA", "tnaA"),
                ("sucA", "yfaD"),
                ("sucA", "ygcE"),
                ("sucA", "yhdM"),
                ("tnaA", "fixC"),
                ("yaeM", "lacZ"),
                ("yceP", "b1583"),
                ("yceP", "yfaD"),
                ("ycgX", "atpD"),
                ("ycgX", "b1191"),
                ("ycgX", "fixC"),
                ("ycgX", "tnaA"),
                ("ycgX", "ygcE"),
                ("ycgX", "yheI"),
                ("yecO", "cspG"),
                ("yedE", "atpD"),
                ("yedE", "dnaG"),
                ("yedE", "lpdA"),
                ("yfiA", "cspA"),
                ("ygcE", "asnA"),
                ("ygcE", "b1191"),
                ("ygcE", "icdA"),
                ("ygcE", "yaeM"),
                ("yhdM", "dnaG"),
                ("yheI", "atpD"),
                ("yheI", "b1191"),
                ("yheI", "b1963"),
                ("yheI", "dnaK"),
                ("yheI", "fixC"),
                ("yheI", "folK"),
                ("yheI", "tnaA"),
                ("yheI", "ygcE"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = ContinuousDataSet::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new(&d);

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
                ("asnA", "icdA"),
                ("asnA", "lacA"),
                ("atpD", "ygcE"),
                ("atpG", "lacA"),
                ("b1191", "fixC"),
                ("b1191", "tnaA"),
                ("cspA", "cspG"),
                ("cspA", "yecO"),
                ("cspG", "lacA"),
                ("cspG", "lacZ"),
                ("cspG", "pspA"),
                ("cspG", "pspB"),
                ("cspG", "yaeM"),
                ("cspG", "yedE"),
                ("dnaG", "ycgX"),
                ("dnaG", "yheI"),
                ("dnaJ", "cchB"),
                ("dnaJ", "sucA"),
                ("dnaK", "mopB"),
                ("eutG", "lacA"),
                ("eutG", "lacY"),
                ("eutG", "yceP"),
                ("eutG", "yfaD"),
                ("fixC", "cchB"),
                ("fixC", "eutG"),
                ("fixC", "ibpB"),
                ("fixC", "yceP"),
                ("fixC", "ygbD"),
                ("fixC", "yjbO"),
                ("hupB", "cspA"),
                ("hupB", "yfiA"),
                ("ibpB", "eutG"),
                ("ibpB", "yceP"),
                ("icdA", "aceB"),
                ("lacA", "b1583"),
                ("lacA", "lacZ"),
                ("lacA", "yaeM"),
                ("lacY", "lacA"),
                ("lacY", "lacZ"),
                ("lacY", "nuoM"),
                ("lacY", "yaeM"),
                ("lacZ", "b1583"),
                ("lacZ", "mopB"),
                ("lpdA", "ycgX"),
                ("mopB", "ftsJ"),
                ("pspA", "nmpC"),
                ("pspA", "yedE"),
                ("pspB", "pspA"),
                ("pspB", "yedE"),
                ("sucA", "atpD"),
                ("sucA", "atpG"),
                ("sucA", "b1191"),
                ("sucA", "dnaG"),
                ("sucA", "eutG"),
                ("sucA", "fixC"),
                ("sucA", "flgD"),
                ("sucA", "gltA"),
                ("sucA", "ibpB"),
                ("sucA", "sucD"),
                ("sucA", "tnaA"),
                ("sucA", "yfaD"),
                ("sucA", "ygcE"),
                ("sucA", "yhdM"),
                ("tnaA", "fixC"),
                ("yaeM", "lacZ"),
                ("yceP", "b1583"),
                ("yceP", "yfaD"),
                ("ycgX", "atpD"),
                ("ycgX", "b1191"),
                ("ycgX", "fixC"),
                ("ycgX", "tnaA"),
                ("ycgX", "ygcE"),
                ("ycgX", "yheI"),
                ("yecO", "cspG"),
                ("yedE", "atpD"),
                ("yedE", "dnaG"),
                ("yedE", "lpdA"),
                ("yfiA", "cspA"),
                ("ygcE", "asnA"),
                ("ygcE", "b1191"),
                ("ygcE", "icdA"),
                ("ygcE", "yaeM"),
                ("yhdM", "dnaG"),
                ("yheI", "atpD"),
                ("yheI", "b1191"),
                ("yheI", "b1963"),
                ("yheI", "dnaK"),
                ("yheI", "fixC"),
                ("yheI", "folK"),
                ("yheI", "tnaA"),
                ("yheI", "ygcE"),
            ],
        );

        // Load data set.
        let d = CsvReader::from_path("./tests/assets/ecoli70.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = ContinuousDataSet::from(d);

        // Initialize empty prior knowledge.
        let k = FR::new(d.labels(), [], []);

        // Initialize score functor.
        let s = BIC::new(&d);

        // Initialize discovery functor.
        let hc = ParallelHC::new(&s);
        // Perform discovery.
        let pred_g: DiGraph = hc.call(&d, &k);

        assert_eq!(pred_g, true_g);
    }
}

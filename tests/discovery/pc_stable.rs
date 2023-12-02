#[cfg(test)]
mod categorical {
    use causal_hub::{polars::prelude::*, prelude::*};

    // Set ChiSquared significance level.
    const ALPHA: f64 = 0.05;

    // Set base path.
    const BASE_PATH: &str = "./tests/assets/pc_stable/";

    #[test]
    fn cancer() {
        // Set dataset name.
        let d: String = "cancer".into();
        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, d))
            .unwrap()
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Set true graph.
        let mut true_g = PGraph::empty(L!(d));
        // Add directed edge.
        true_g.add_directed_edge(3, 0);
        true_g.add_directed_edge(4, 0);
        // Set true skeleton
        let true_skel = true_g.to_undirected();

        // Create ChiSquared conditional independence test.
        let test = ChiSquared::new(&d, ALPHA);

        // Create PC-Stable functor.
        let pc_stable = PCStable::new(&test);

        // Perform skeleton discovery.
        let (skel, _): (UGraph, _) = pc_stable.skeleton();
        let (par_skel, _) = pc_stable.par_skeleton();

        // Perform discovery.
        let g: PGraph = pc_stable.call();
        let par_g = pc_stable.par_call();

        // Perform tests.
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn asia() {
        // Set dataset name.
        let d: String = "asia".into();
        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, d))
            .unwrap()
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Set true graph.
        let mut true_g = PGraph::empty(L!(d));
        // Add undirected edge.
        true_g.add_undirected_edge(1, 2);
        true_g.add_undirected_edge(1, 5);
        true_g.add_undirected_edge(3, 4);
        // Set true skeleton.
        let true_skel = true_g.to_undirected();

        // Create ChiSquared conditional independence test.
        let test = ChiSquared::new(&d, ALPHA);

        // Create PC-Stable functor.
        let pc_stable = PCStable::new(&test);

        // Perform skeleton discovery.
        let (skel, _): (UGraph, _) = pc_stable.skeleton();
        let (par_skel, _) = pc_stable.par_skeleton();

        // Perform discovery.
        let g: PGraph = pc_stable.call();
        let par_g = pc_stable.par_call();

        // Perform tests.
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }

    #[test]
    fn survey() {
        // Set dataset name.
        let d: String = "survey".into();
        // Load data set.
        let d = CsvReader::from_path(format!("{}{}.csv", BASE_PATH, d))
            .unwrap()
            .finish()
            .unwrap();
        let d = CategoricalDataMatrix::from(d);

        // Set true graph.
        let mut true_g = PGraph::empty(L!(d));
        // Add undirected edge.
        true_g.add_undirected_edge(3, 5);
        true_g.add_directed_edge(0, 1);
        true_g.add_directed_edge(4, 1);
        // Set true skeleton
        let true_skel = true_g.to_undirected();

        // Create ChiSquared conditional independence test.
        let test = ChiSquared::new(&d, ALPHA);

        // Create PC-Stable functor.
        let pc_stable = PCStable::new(&test);

        // Perform skeleton discovery.
        let (skel, _): (UGraph, _) = pc_stable.skeleton();
        let (par_skel, _) = pc_stable.par_skeleton();

        // Perform discovery.
        let g: PGraph = pc_stable.call();
        let par_g = pc_stable.par_call();

        // Perform tests.
        assert_eq!(skel, par_skel);
        assert_eq!(g, par_g);

        assert_eq!(skel, true_skel);
        assert_eq!(g, true_g);
    }
}

#[cfg(test)]
mod variable_elimination {
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn project_onto() {
        // Load reference data.
        let d: DiscreteDataMatrix = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap()
            .into();
        // Load reference model.
        let p: DiscreteBN = BIF::read("./tests/assets/bif/asia.bif").unwrap().into();

        // Construct distribution projection estimator.
        let estimator = VE::new(&p);
        // Compute projection of P on itself.
        let q = estimator.project_onto(&p);

        assert_eq!(p, q);

        // Construct modified graph.
        let mut q = p.graph().clone();
        q.del_edge(q.vertex("asia"), q.vertex("tub"));
        // Fit modified graph with maximum likelihood estimator.
        let q = MLE::fit(&d, &q);
        // Project P onto Q using variable elimination as estimator.
        let p = estimator.project_onto(&q);

        assert_eq!(
            p,
            BIF::read("./tests/assets/distribution_projection/asia_projected.bif")
                .unwrap()
                .into()
        );
    }
}

#[cfg(test)]
mod tests {
    use approx::*;
    use causal_hub::prelude::*;
    use polars::prelude::*;

    #[test]
    fn call() {
        // Load reference data.
        let d: CategoricalDataMatrix = CsvReader::from_path("./tests/assets/asia.csv")
            .unwrap()
            .finish()
            .unwrap()
            .into();
        // Load reference model.
        let p: CategoricalBN = BIF::read("./tests/assets/bif/asia.bif").unwrap().into();

        // KL of P given P is zero.
        assert_relative_eq!(KL::new(&p, &p).call(), 0.);

        // Construct modified graph.
        let mut q = p.graph().clone();
        q.del_edge(q.label_to_vertex("bronc"), q.label_to_vertex("dysp"));
        // Fit modified graph with maximum likelihood estimator.
        let q = MLE::call(&d, &q);
        // Project P onto Q using variable elimination as estimator.
        let p = VE::new(&p).project_onto(&q);

        assert_relative_eq!(KL::new(&p, &q).call(), 0.04892162293878239);
    }
}

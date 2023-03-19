#[cfg(test)]
mod variable_elimination {
    use causal_hub::prelude::*;

    #[test]
    fn project_onto() {
        // Load reference model.
        let p: DiscreteBN = BIF::read("./tests/assets/bif/asia.bif").unwrap().into();

        // Construct distribution projection estimator.
        let estimator = VariableElimination::new(&p);

        // Compute projection of P on itself.
        let q = estimator.project_onto(&p);

        assert_eq!(p, q);
    }
}

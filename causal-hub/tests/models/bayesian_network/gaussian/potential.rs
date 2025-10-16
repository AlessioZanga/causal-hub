#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        labels,
        models::{GaussCPD, GaussCPDP, GaussPhi, GaussPhiK, Phi},
        set,
    };
    use ndarray::prelude::*;

    #[test]
    fn from_cpd() {
        // Set the labels.
        let x = labels!("A");
        let z = labels!("B", "C");
        // Set the parameters.
        let a = array![[3., -1.]];
        let b = array![2.];
        let s = array![[4.]];
        let p = GaussCPDP::new(a, b, s);
        // Initialize the CPD.
        let cpd = GaussCPD::new(x, z, p);

        // Convert to potential.
        let pred_phi = GaussPhi::from_cpd(cpd);

        // Set the true potential.
        let true_l = labels!("A", "B", "C");
        let true_k = array![
            [0.25, -0.75, 0.25],  //
            [-0.75, 2.25, -0.75], //
            [0.25, -0.75, 0.25]
        ];
        let true_h = array![0.5, -1.5, 0.5];
        let true_g = -2.112085713764618;
        let true_p = GaussPhiK::new(true_k, true_h, true_g);
        let true_phi = GaussPhi::new(true_l, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn into_cpd() {
        // Set the labels.
        let x = labels!("A");
        let z = labels!("B", "C");
        // Set the parameters.
        let a = array![[3., -1.]];
        let b = array![2.];
        let s = array![[4.]];
        let p = GaussCPDP::new(a, b, s);
        // Initialize the CPD.
        let true_cpd = GaussCPD::new(x, z, p);

        // Convert to potential.
        let phi = GaussPhi::from_cpd(true_cpd.clone());

        // Convert back to CPD.
        let pred_cpd = phi.into_cpd(&set![0], &set![2, 1]);

        // Compare the CPDs.
        assert_relative_eq!(true_cpd, pred_cpd);
    }
}

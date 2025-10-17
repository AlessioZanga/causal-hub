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
    fn multiply() {
        // Set the labels.
        let l_1 = labels!("A", "B");
        let l_2 = labels!("B", "C");
        // Set the parameters.
        let p_1 = GaussPhiK::new(
            array![
                [1., -1.], //
                [-1., 1.]  //
            ],
            array![1., -1.], //
            -3.,
        );
        let p_2 = GaussPhiK::new(
            array![
                [3., -2.], //
                [-2., 4.]  //
            ],
            array![5., -1.], //
            1.,
        );
        // Initialize the potential.
        let phi_1 = GaussPhi::new(l_1, p_1);
        let phi_2 = GaussPhi::new(l_2, p_2);

        // Multiply the potentials.
        let pred_phi = &phi_1 * &phi_2;

        // Set the true potential.
        let true_l = labels!("A", "B", "C");
        let true_p = GaussPhiK::new(
            array![
                [1., -1., 0.],  //
                [-1., 4., -2.], //
                [0., -2., 4.]   //
            ],
            array![1., 4., -1.],
            -2.,
        );
        let true_phi = GaussPhi::new(true_l, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        // Test other variant.
        let mut pred_phi = phi_1.clone();
        pred_phi *= &phi_2;
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn divide() {
        // Set the labels.
        let l_1 = labels!("A", "B");
        let l_2 = labels!("B", "C");
        // Set the parameters.
        let p_1 = GaussPhiK::new(
            array![
                [1., -1.], //
                [-1., 1.]  //
            ],
            array![1., -1.], //
            -3.,
        );
        let p_2 = GaussPhiK::new(
            array![
                [3., -2.], //
                [-2., 4.]  //
            ],
            array![5., -1.], //
            1.,
        );
        // Initialize the potential.
        let phi_1 = GaussPhi::new(l_1, p_1);
        let phi_2 = GaussPhi::new(l_2, p_2);

        // Multiply the potentials.
        let pred_phi = &phi_1 / &phi_2;

        // Set the true potential.
        let true_l = labels!("A", "B", "C");
        let true_p = GaussPhiK::new(
            array![
                [1., -1., 0.],  //
                [-1., -2., 2.], //
                [0., 2., -4.]   //
            ],
            array![1., -6., 1.],
            -4.,
        );
        let true_phi = GaussPhi::new(true_l, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        // Test other variant.
        let mut pred_phi = phi_1.clone();
        pred_phi /= &phi_2;
        assert_relative_eq!(true_phi, pred_phi);
    }

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

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{CatEv, CatEvT},
        labels,
        models::{CatCPD, CatPhi, Labelled, Phi},
        set, states,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() {
        // Set the states.
        let s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let p = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        // Initialize the potential.
        let phi = CatPhi::new(s.clone(), p.clone());

        // Assert the labels.
        assert_eq!(&labels!["A", "B", "C"], phi.labels(),);
        // Assert the states.
        assert_eq!(phi.states(), &s);
        // Assert the shape.
        assert_eq!(phi.shape(), &array![3, 2, 2]);
        // Assert the parameters.
        assert_relative_eq!(phi.parameters(), &p);
    }

    #[test]
    fn condition() {
        // Set the states.
        let s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let p = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        // Initialize the potential.
        let phi = CatPhi::new(s.clone(), p);

        // Condition the potential.
        let e = CatEv::new(s, [CatEvT::CertainPositive { event: 2, state: 0 }]);
        let pred_phi = phi.condition(&e);

        // Set the true potential.
        let true_s = states![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let true_p = array![0.25, 0.08, 0.05, 0., 0.15, 0.09]
            .into_shape_with_order((3, 2))
            .unwrap()
            .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn marginalize() {
        // Set the states.
        let s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let p = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        // Initialize the potential.
        let phi = CatPhi::new(s, p);

        // Marginalize the potential.
        let pred_phi = phi.marginalize(&set![1]);

        // Set the true potential.
        let true_s = states![("A", ["a1", "a2", "a3"]), ("C", ["c1", "c2"]),];
        let true_p = array![0.33, 0.51, 0.05, 0.07, 0.24, 0.39]
            .into_shape_with_order((3, 2))
            .unwrap()
            .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn normalize() {
        // Set the states.
        let s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let p = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        // Initialize the potential.
        let phi = CatPhi::new(s.clone(), p.clone());

        // Marginalize the potential.
        let pred_phi = phi.normalize();

        // Set the true potential.
        let true_s = s;
        let true_p = &p / p.sum();
        let true_phi = CatPhi::new(true_s, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn multiply() {
        // Set the states.
        let s_1 = states![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let s_2 = states![("B", ["b1", "b2"]), ("C", ["c1", "c2"]),];
        // Set the parameters.
        let p_1 = array![0.5, 0.8, 0.1, 0., 0.3, 0.9]
            .into_shape_with_order((3, 2))
            .unwrap()
            .into_dyn();
        let p_2 = array![0.5, 0.7, 0.1, 0.2]
            .into_shape_with_order((2, 2))
            .unwrap()
            .into_dyn();
        // Initialize the potential.
        let phi_1 = CatPhi::new(s_1, p_1);
        let phi_2 = CatPhi::new(s_2, p_2);

        // Multiply the potentials.
        let pred_phi = &phi_1 * &phi_2;

        // Set the true potential.
        let true_s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let true_p = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        // Test other variant.
        let mut pred_phi = phi_1.clone();
        pred_phi *= &phi_2;
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn divide() {
        // Set the states.
        let s_1 = states![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let s_2 = states![("A", ["a1", "a2", "a3"]),];
        // Set the parameters.
        let p_1 = array![0.5, 0.2, 0., 0., 0.3, 0.45]
            .into_shape_with_order((3, 2))
            .unwrap()
            .into_dyn();
        let p_2 = array![0.8, 0., 0.6]
            .into_shape_with_order((3,))
            .unwrap()
            .into_dyn();
        // Initialize the potential.
        let phi_1 = CatPhi::new(s_1, p_1);
        let phi_2 = CatPhi::new(s_2, p_2);

        // Divide the potentials.
        let pred_phi = &phi_1 / &phi_2;

        // Set the true potential.
        let true_s = states![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let true_p = array![0.625, 0.25, 0., 0., 0.5, 0.75]
            .into_shape_with_order((3, 2))
            .unwrap()
            .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        // Test other variant.
        let mut pred_phi = phi_1;
        pred_phi /= &phi_2;
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn from_cpd() {
        // Set the states.
        let x = states![("A", ["a1", "a2", "a3"]),];
        let z = states![("B", ["b1", "b2"]), ("C", ["c1", "c2"]),];
        // Set the parameters.
        let p = array![
            [0.25, 0.35, 0.40],
            [0.05, 0.15, 0.80],
            [0.30, 0.70, 0.00],
            [0.10, 0.90, 0.00]
        ];
        // Initialize the CPD.
        let cpd = CatCPD::new(x, z, p);

        // Convert the CPD into a potential.
        let pred_phi = CatPhi::from_cpd(cpd);

        // Set the true potential.
        let true_s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let true_p = array![
            0.25, 0.05, 0.30, 0.10, 0.35, 0.15, 0.70, 0.90, 0.40, 0.80, 0.00, 0.00
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p);

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);
    }

    #[test]
    fn into_cpd() {
        // Set the true potential.
        let s = states![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let p = array![
            0.25, 0.05, 0.30, 0.10, 0.35, 0.15, 0.70, 0.90, 0.40, 0.80, 0.00, 0.00
        ]
        .into_shape_with_order((3, 2, 2))
        .unwrap()
        .into_dyn();
        let phi = CatPhi::new(s, p);

        // Convert the potential into a CPD.
        let pred_cpd = phi.into_cpd(&set![0], &set![2, 1]);

        // Set the true CPD.
        let true_x = states![("A", ["a1", "a2", "a3"])];
        let true_z = states![("B", ["b1", "b2"]), ("C", ["c1", "c2"])];
        let true_p = array![
            [0.25, 0.35, 0.40],
            [0.05, 0.15, 0.80],
            [0.30, 0.70, 0.00],
            [0.10, 0.90, 0.00]
        ];
        let true_cpd = CatCPD::new(true_x, true_z, true_p);

        // Compare the CPDs.
        assert_relative_eq!(true_cpd, pred_cpd);
    }
}

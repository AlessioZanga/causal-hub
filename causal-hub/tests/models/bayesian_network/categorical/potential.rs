#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{CatEv, CatEvT},
        labels,
        models::{CatCPD, CatPhi, Labelled, Phi},
        set, support,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() -> Result<()> {
        // Set the support.
        let stats = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let probability = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        // Initialize the potential.
        let potential = CatPhi::new(stats.clone(), probability.clone())?;

        // Assert the labels.
        assert_eq!(potential.labels(), &labels!["A", "B", "C"]);
        // Assert the support.
        assert_eq!(potential.support(), &stats);
        // Assert the shape.
        assert_eq!(potential.shape(), &array![3, 2, 2]);
        // Assert the parameters.
        assert_relative_eq!(potential.parameters(), &probability);

        Ok(())
    }

    #[test]
    fn condition() -> Result<()> {
        // Set the support.
        let stats = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let probability = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        // Initialize the potential.
        let potential = CatPhi::new(stats.clone(), probability)?;

        // Condition the potential.
        let evidence = CatEv::new(stats, [CatEvT::CertainPositive { event: 2, state: 0 }])?;
        let pred_phi = potential.condition(&evidence)?;

        // Set the true potential.
        let true_s = support![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let true_p = array![0.25, 0.08, 0.05, 0., 0.15, 0.09]
            .into_shape_with_order((3, 2))?
            .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p)?;

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        Ok(())
    }

    #[test]
    fn marginalize() -> Result<()> {
        // Set the support.
        let stats = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let probability = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        // Initialize the potential.
        let potential = CatPhi::new(stats, probability)?;

        // Marginalize the potential.
        let pred_phi = potential.marginalize(&set![1])?;

        // Set the true potential.
        let true_s = support![("A", ["a1", "a2", "a3"]), ("C", ["c1", "c2"]),];
        let true_p = array![0.33, 0.51, 0.05, 0.07, 0.24, 0.39]
            .into_shape_with_order((3, 2))?
            .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p)?;

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        Ok(())
    }

    #[test]
    fn normalize() -> Result<()> {
        // Set the support.
        let stats = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        // Set the parameters.
        let probability = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        // Initialize the potential.
        let potential = CatPhi::new(stats.clone(), probability.clone())?;

        // Marginalize the potential.
        let pred_phi = potential.normalize()?;

        // Set the true potential.
        let true_s = stats;
        let true_p = &probability / probability.sum();
        let true_phi = CatPhi::new(true_s, true_p)?;

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        Ok(())
    }

    #[test]
    fn multiply() -> Result<()> {
        // Set the support.
        let s_1 = support![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let s_2 = support![("B", ["b1", "b2"]), ("C", ["c1", "c2"]),];
        // Set the parameters.
        let p_1 = array![0.5, 0.8, 0.1, 0., 0.3, 0.9]
            .into_shape_with_order((3, 2))?
            .into_dyn();
        let p_2 = array![0.5, 0.7, 0.1, 0.2]
            .into_shape_with_order((2, 2))?
            .into_dyn();
        // Initialize the potential.
        let phi_1 = CatPhi::new(s_1, p_1)?;
        let phi_2 = CatPhi::new(s_2, p_2)?;

        // Multiply the potentials.
        let pred_phi = &phi_1 * &phi_2;

        // Set the true potential.
        let true_s = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let true_p = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p)?;

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        // Test other variant.
        let mut pred_phi = phi_1.clone();
        pred_phi *= &phi_2;
        assert_relative_eq!(true_phi, pred_phi);

        Ok(())
    }

    #[test]
    fn divide() -> Result<()> {
        // Set the support.
        let s_1 = support![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let s_2 = support![("A", ["a1", "a2", "a3"]),];
        // Set the parameters.
        let p_1 = array![0.5, 0.2, 0., 0., 0.3, 0.45]
            .into_shape_with_order((3, 2))?
            .into_dyn();
        let p_2 = array![0.8, 0., 0.6].into_shape_with_order((3,))?.into_dyn();
        // Initialize the potential.
        let phi_1 = CatPhi::new(s_1, p_1)?;
        let phi_2 = CatPhi::new(s_2, p_2)?;

        // Divide the potentials.
        let pred_phi = phi_1.div(&phi_2)?;

        // Set the true potential.
        let true_s = support![("A", ["a1", "a2", "a3"]), ("B", ["b1", "b2"]),];
        let true_p = array![0.625, 0.25, 0., 0., 0.5, 0.75]
            .into_shape_with_order((3, 2))?
            .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p)?;

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        // Test other variant.
        let mut pred_phi = phi_1;
        pred_phi.div_assign(&phi_2)?;
        assert_relative_eq!(true_phi, pred_phi);

        Ok(())
    }

    #[test]
    fn from_cpd() -> Result<()> {
        // Set the support.
        let x = support![("A", ["a1", "a2", "a3"]),];
        let z = support![("B", ["b1", "b2"]), ("C", ["c1", "c2"]),];
        // Set the parameters.
        let probability = array![
            [0.25, 0.35, 0.40],
            [0.05, 0.15, 0.80],
            [0.30, 0.70, 0.00],
            [0.10, 0.90, 0.00]
        ];
        // Initialize the CPD.
        let distribution = CatCPD::new(x, z, probability)?;

        // Convert the CPD into a potential.
        let pred_phi = CatPhi::from_cpd(distribution)?;

        // Set the true potential.
        let true_s = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let true_p = array![
            0.25, 0.05, 0.30, 0.10, 0.35, 0.15, 0.70, 0.90, 0.40, 0.80, 0.00, 0.00
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        let true_phi = CatPhi::new(true_s, true_p)?;

        // Compare the potentials.
        assert_relative_eq!(true_phi, pred_phi);

        Ok(())
    }

    #[test]
    fn into_cpd() -> Result<()> {
        // Set the true potential.
        let stats = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let probability = array![
            0.25, 0.05, 0.30, 0.10, 0.35, 0.15, 0.70, 0.90, 0.40, 0.80, 0.00, 0.00
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        let potential = CatPhi::new(stats, probability)?;

        // Convert the potential into a CPD.
        let pred_cpd = potential.into_cpd(&set![0], &set![2, 1])?;

        // Set the true CPD.
        let true_x = support![("A", ["a1", "a2", "a3"])];
        let true_z = support![("B", ["b1", "b2"]), ("C", ["c1", "c2"])];
        let true_p = array![
            [0.25, 0.35, 0.40],
            [0.05, 0.15, 0.80],
            [0.30, 0.70, 0.00],
            [0.10, 0.90, 0.00]
        ];
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        // Compare the CPDs.
        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn support() -> Result<()> {
        let stats = support![
            ("A", ["a1", "a2", "a3"]),
            ("B", ["b1", "b2"]),
            ("C", ["c1", "c2"]),
        ];
        let probability = array![
            0.25, 0.35, 0.08, 0.16, 0.05, 0.07, 0., 0., 0.15, 0.21, 0.09, 0.18
        ]
        .into_shape_with_order((3, 2, 2))?
        .into_dyn();
        let potential = CatPhi::new(stats.clone(), probability)?;

        let support = Phi::support(&potential);
        assert_eq!(*support, stats);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        labels,
        models::{CPD, CatCPD, Labelled},
        set, support,
        types::Result,
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"]), ("C", ["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let cpd = CatCPD::new(x, z, p.clone())?;

        assert_eq!(cpd.labels(), &labels!["A"]);
        assert_eq!(cpd.support(), &support![("A", ["no", "yes"])]);
        assert_eq!(cpd.conditioning_labels(), &labels!["B", "C"]);
        assert!(
            cpd.conditioning_support()
                .values()
                .all(|x| x.iter().eq(["no", "yes"]))
        );
        assert_eq!(cpd.parameters(), &p);

        Ok(())
    }

    #[test]
    fn unique_labels() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("A", ["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        assert!(CatCPD::new(x, z, p).is_err());

        Ok(())
    }

    #[test]
    fn empty_labels() -> Result<()> {
        let x = support![];
        let z = support![];
        let p = array![[]];
        assert!(CatCPD::new(x, z, p).is_err());

        Ok(())
    }

    #[test]
    fn display() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8]];
        let cpd = CatCPD::new(x, z, p)?;

        assert_eq!(
            cpd.to_string(),
            concat!(
                "----------------------------------\n",
                "|          | A        |          |\n",
                "| -------- | -------- | -------- |\n",
                "| B        | no       | yes      |\n",
                "| -------- | -------- | -------- |\n",
                "| no       | 0.100000 | 0.900000 |\n",
                "| yes      | 0.200000 | 0.800000 |\n",
                "----------------------------------\n",
            )
        );

        Ok(())
    }

    #[test]
    fn marginalize_single_x() -> Result<()> {
        let x = support![("A", ["no", "yes"]), ("B", ["no", "yes"])];
        let z = support![("C", ["no", "yes"]), ("D", ["no", "yes"])];
        let p = array![
            // A0,    0,    1,    1
            // B0     1     0     1
            [0.10, 0.40, 0.30, 0.20],
            [0.20, 0.30, 0.25, 0.25],
            [0.30, 0.20, 0.35, 0.15],
            [0.40, 0.10, 0.45, 0.05],
        ];
        let cpd = CatCPD::new(x, z, p.clone())?;

        let pred_cpd = cpd.marginalize(&set![0], &set![])?;

        let true_x = support![("B", ["no", "yes"])];
        let true_z = support![("C", ["no", "yes"]), ("D", ["no", "yes"])];
        let true_p = array![
            // B                 0,                      1,     (C, D)
            [p[[0, 0]] + p[[0, 2]], p[[0, 1]] + p[[0, 3]]], //  (0, 0)
            [p[[1, 0]] + p[[1, 2]], p[[1, 1]] + p[[1, 3]]], //  (0, 1)
            [p[[2, 0]] + p[[2, 2]], p[[2, 1]] + p[[2, 3]]], //  (1, 0)
            [p[[3, 0]] + p[[3, 2]], p[[3, 1]] + p[[3, 3]]], //  (1, 1)
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn marginalize_multiple_x() -> Result<()> {
        let x = support![
            ("A", ["no", "yes"]),
            ("B", ["no", "yes"]),
            ("C", ["no", "yes"])
        ];
        let z = support![
            ("D", ["no", "yes"]),
            ("E", ["no", "yes"]),
            ("F", ["no", "yes"])
        ];
        let p = array![
            // A0,    0,    0,    0,    1,    1,    1,    1
            // B0,    0,    1,    1,    0,    0,    1,    1
            // C0,    1,    0,    1,    0,    1,    0,    1
            [0.24, 0.10, 0.02, 0.09, 0.12, 0.17, 0.24, 0.02],
            [0.04, 0.22, 0.14, 0.21, 0.09, 0.12, 0.11, 0.07],
            [0.19, 0.10, 0.07, 0.13, 0.05, 0.20, 0.10, 0.16],
            [0.17, 0.03, 0.12, 0.06, 0.20, 0.10, 0.15, 0.17],
            [0.21, 0.07, 0.08, 0.04, 0.13, 0.25, 0.18, 0.04],
            [0.13, 0.11, 0.02, 0.21, 0.17, 0.03, 0.24, 0.09],
            [0.04, 0.10, 0.06, 0.16, 0.21, 0.17, 0.12, 0.14],
            [0.09, 0.16, 0.13, 0.11, 0.05, 0.21, 0.04, 0.21]
        ];
        let cpd = CatCPD::new(x, z, p.clone())?;

        let pred_cpd = cpd.marginalize(&set![0, 2], &set![])?;

        let true_x = support![("B", ["no", "yes"])];
        let true_z = support![
            ("D", ["no", "yes"]),
            ("E", ["no", "yes"]),
            ("F", ["no", "yes"])
        ];
        let true_p = array![
            // B                                                    (D, E, F)
            [
                // B0
                p[[0, 0]] + p[[0, 1]] + p[[0, 4]] + p[[0, 5]],
                // B1
                p[[0, 2]] + p[[0, 3]] + p[[0, 6]] + p[[0, 7]]
            ], //                                                   (0, 0, 0)
            [
                // B0
                p[[1, 0]] + p[[1, 1]] + p[[1, 4]] + p[[1, 5]],
                // B1
                p[[1, 2]] + p[[1, 3]] + p[[1, 6]] + p[[1, 7]]
            ], //                                                   (0, 0, 1)
            [
                // B0
                p[[2, 0]] + p[[2, 1]] + p[[2, 4]] + p[[2, 5]],
                // B1
                p[[2, 2]] + p[[2, 3]] + p[[2, 6]] + p[[2, 7]]
            ], //                                                   (0, 1, 0)
            [
                // B0
                p[[3, 0]] + p[[3, 1]] + p[[3, 4]] + p[[3, 5]],
                // B1
                p[[3, 2]] + p[[3, 3]] + p[[3, 6]] + p[[3, 7]]
            ], //                                                   (0, 1, 1)
            [
                // B0
                p[[4, 0]] + p[[4, 1]] + p[[4, 4]] + p[[4, 5]],
                // B1
                p[[4, 2]] + p[[4, 3]] + p[[4, 6]] + p[[4, 7]]
            ], //                                                   (1, 0, 0)
            [
                // B0
                p[[5, 0]] + p[[5, 1]] + p[[5, 4]] + p[[5, 5]],
                // B1
                p[[5, 2]] + p[[5, 3]] + p[[5, 6]] + p[[5, 7]]
            ], //                                                   (1, 0, 1)
            [
                // B0
                p[[6, 0]] + p[[6, 1]] + p[[6, 4]] + p[[6, 5]],
                // B1
                p[[6, 2]] + p[[6, 3]] + p[[6, 6]] + p[[6, 7]]
            ], //                                                   (1, 1, 0)
            [
                // B0
                p[[7, 0]] + p[[7, 1]] + p[[7, 4]] + p[[7, 5]],
                // B1
                p[[7, 2]] + p[[7, 3]] + p[[7, 6]] + p[[7, 7]]
            ] //                                                   (1, 1, 1)
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn marginalize_single_z() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"]), ("C", ["no", "yes"])];
        let p = array![
            //                  (B, C)
            [0.10, 0.90], //    (0, 0)
            [0.20, 0.80], //    (0, 1)
            [0.30, 0.70], //    (1, 0)
            [0.40, 0.60]  //    (1, 1)
        ];
        let cpd = CatCPD::new(x, z, p.clone())?;

        let pred_cpd = cpd.marginalize(&set![], &set![0])?;

        let true_x = support![("A", ["no", "yes"])];
        let true_z = support![("C", ["no", "yes"])];
        let true_p = array![
            //                                                      (C)
            [(p[[0, 0]] + p[[2, 0]]), (p[[0, 1]] + p[[2, 1]])], //  (0)
            [(p[[1, 0]] + p[[3, 0]]), (p[[1, 1]] + p[[3, 1]])]  //  (1)
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        let pred_cpd = cpd.marginalize(&set![], &set![1])?;

        let true_x = support![("A", ["no", "yes"])];
        let true_z = support![("B", ["no", "yes"])];
        let true_p = array![
            //                                                      (B)
            [(p[[0, 0]] + p[[1, 0]]), (p[[0, 1]] + p[[1, 1]])], //  (0)
            [(p[[2, 0]] + p[[3, 0]]), (p[[2, 1]] + p[[3, 1]])]  //  (1)
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn marginalize_multiple_z() -> Result<()> {
        let x = support![
            ("A", ["no", "yes"]),
            ("B", ["no", "yes"]),
            ("C", ["no", "yes"])
        ];
        let z = support![
            ("D", ["no", "yes"]),
            ("E", ["no", "yes"]),
            ("F", ["no", "yes"])
        ];
        let p = array![
            //                                                   (D, E, F)
            [0.24, 0.10, 0.02, 0.09, 0.12, 0.17, 0.24, 0.02], // (0, 0, 0)
            [0.04, 0.22, 0.14, 0.21, 0.09, 0.12, 0.11, 0.07], // (0, 0, 1)
            [0.19, 0.10, 0.07, 0.13, 0.05, 0.20, 0.10, 0.16], // (0, 1, 0)
            [0.17, 0.03, 0.12, 0.06, 0.20, 0.10, 0.15, 0.17], // (0, 1, 1)
            [0.21, 0.07, 0.08, 0.04, 0.13, 0.25, 0.18, 0.04], // (1, 0, 0)
            [0.13, 0.11, 0.02, 0.21, 0.17, 0.03, 0.24, 0.09], // (1, 0, 1)
            [0.04, 0.10, 0.06, 0.16, 0.21, 0.17, 0.12, 0.14], // (1, 1, 0)
            [0.09, 0.16, 0.13, 0.11, 0.05, 0.21, 0.04, 0.21]  // (1, 1, 1)
        ];
        let cpd = CatCPD::new(x, z, p.clone())?;

        let pred_cpd = cpd.marginalize(&set![], &set![0, 2])?;

        let true_x = support![
            ("A", ["no", "yes"]),
            ("B", ["no", "yes"]),
            ("C", ["no", "yes"])
        ];
        let true_z = support![("E", ["no", "yes"])];
        let true_p = array![
            //                                                  (E)
            [
                p[[0, 0]] + p[[1, 0]] + p[[4, 0]] + p[[5, 0]],
                p[[0, 1]] + p[[1, 1]] + p[[4, 1]] + p[[5, 1]],
                p[[0, 2]] + p[[1, 2]] + p[[4, 2]] + p[[5, 2]],
                p[[0, 3]] + p[[1, 3]] + p[[4, 3]] + p[[5, 3]],
                p[[0, 4]] + p[[1, 4]] + p[[4, 4]] + p[[5, 4]],
                p[[0, 5]] + p[[1, 5]] + p[[4, 5]] + p[[5, 5]],
                p[[0, 6]] + p[[1, 6]] + p[[4, 6]] + p[[5, 6]],
                p[[0, 7]] + p[[1, 7]] + p[[4, 7]] + p[[5, 7]]
            ], //                                               (0)
            [
                p[[2, 0]] + p[[3, 0]] + p[[6, 0]] + p[[7, 0]],
                p[[2, 1]] + p[[3, 1]] + p[[6, 1]] + p[[7, 1]],
                p[[2, 2]] + p[[3, 2]] + p[[6, 2]] + p[[7, 2]],
                p[[2, 3]] + p[[3, 3]] + p[[6, 3]] + p[[7, 3]],
                p[[2, 4]] + p[[3, 4]] + p[[6, 4]] + p[[7, 4]],
                p[[2, 5]] + p[[3, 5]] + p[[6, 5]] + p[[7, 5]],
                p[[2, 6]] + p[[3, 6]] + p[[6, 6]] + p[[7, 6]],
                p[[2, 7]] + p[[3, 7]] + p[[6, 7]] + p[[7, 7]]
            ] //                                                (1)
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn marginalize_single_x_z() -> Result<()> {
        let x = support![
            ("A", ["no", "yes"]),
            ("B", ["no", "yes"]),
            ("C", ["no", "yes"])
        ];
        let z = support![
            ("D", ["no", "yes"]),
            ("E", ["no", "yes"]),
            ("F", ["no", "yes"])
        ];
        let p = array![
            // A0,    0,    0,    0,    1,    1,    1,    1
            // B0,    0,    1,    1,    0,    0,    1,    1
            // C0,    1,    0,    1,    0,    1,    0,    1      (D, E, F)
            [0.24, 0.10, 0.02, 0.09, 0.12, 0.17, 0.24, 0.02], // (0, 0, 0)
            [0.04, 0.22, 0.14, 0.21, 0.09, 0.12, 0.11, 0.07], // (0, 0, 1)
            [0.19, 0.10, 0.07, 0.13, 0.05, 0.20, 0.10, 0.16], // (0, 1, 0)
            [0.17, 0.03, 0.12, 0.06, 0.20, 0.10, 0.15, 0.17], // (0, 1, 1)
            [0.21, 0.07, 0.08, 0.04, 0.13, 0.25, 0.18, 0.04], // (1, 0, 0)
            [0.13, 0.11, 0.02, 0.21, 0.17, 0.03, 0.24, 0.09], // (1, 0, 1)
            [0.04, 0.10, 0.06, 0.16, 0.21, 0.17, 0.12, 0.14], // (1, 1, 0)
            [0.09, 0.16, 0.13, 0.11, 0.05, 0.21, 0.04, 0.21]  // (1, 1, 1)
        ];
        let cpd = CatCPD::new(x, z, p.clone())?;

        let pred_cpd = cpd.marginalize(&set![1], &set![2])?;

        let true_x = support![("A", ["no", "yes"]), ("C", ["no", "yes"])];
        let true_z = support![("D", ["no", "yes"]), ("E", ["no", "yes"])];
        let true_p = array![
            [
                p[[0, 0]] + p[[0, 2]] + p[[1, 0]] + p[[1, 2]],
                p[[0, 1]] + p[[0, 3]] + p[[1, 1]] + p[[1, 3]],
                p[[0, 4]] + p[[0, 6]] + p[[1, 4]] + p[[1, 6]],
                p[[0, 5]] + p[[0, 7]] + p[[1, 5]] + p[[1, 7]]
            ],
            [
                p[[2, 0]] + p[[2, 2]] + p[[3, 0]] + p[[3, 2]],
                p[[2, 1]] + p[[2, 3]] + p[[3, 1]] + p[[3, 3]],
                p[[2, 4]] + p[[2, 6]] + p[[3, 4]] + p[[3, 6]],
                p[[2, 5]] + p[[2, 7]] + p[[3, 5]] + p[[3, 7]]
            ],
            [
                p[[4, 0]] + p[[4, 2]] + p[[5, 0]] + p[[5, 2]],
                p[[4, 1]] + p[[4, 3]] + p[[5, 1]] + p[[5, 3]],
                p[[4, 4]] + p[[4, 6]] + p[[5, 4]] + p[[5, 6]],
                p[[4, 5]] + p[[4, 7]] + p[[5, 5]] + p[[5, 7]]
            ],
            [
                p[[6, 0]] + p[[6, 2]] + p[[7, 0]] + p[[7, 2]],
                p[[6, 1]] + p[[6, 3]] + p[[7, 1]] + p[[7, 3]],
                p[[6, 4]] + p[[6, 6]] + p[[7, 4]] + p[[7, 6]],
                p[[6, 5]] + p[[6, 7]] + p[[7, 5]] + p[[7, 7]]
            ]
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn marginalize_multiple_x_z() -> Result<()> {
        let x = support![
            ("A", ["no", "yes"]),
            ("B", ["no", "yes"]),
            ("C", ["no", "yes"])
        ];
        let z = support![
            ("D", ["no", "yes"]),
            ("E", ["no", "yes"]),
            ("F", ["no", "yes"])
        ];
        let p = array![
            // A0,    0,    0,    0,    1,    1,    1,    1
            // B0,    0,    1,    1,    0,    0,    1,    1
            // C0,    1,    0,    1,    0,    1,    0,    1      (D, E, F)
            [0.24, 0.10, 0.02, 0.09, 0.12, 0.17, 0.24, 0.02], // (0, 0, 0)
            [0.04, 0.22, 0.14, 0.21, 0.09, 0.12, 0.11, 0.07], // (0, 0, 1)
            [0.19, 0.10, 0.07, 0.13, 0.05, 0.20, 0.10, 0.16], // (0, 1, 0)
            [0.17, 0.03, 0.12, 0.06, 0.20, 0.10, 0.15, 0.17], // (0, 1, 1)
            [0.21, 0.07, 0.08, 0.04, 0.13, 0.25, 0.18, 0.04], // (1, 0, 0)
            [0.13, 0.11, 0.02, 0.21, 0.17, 0.03, 0.24, 0.09], // (1, 0, 1)
            [0.04, 0.10, 0.06, 0.16, 0.21, 0.17, 0.12, 0.14], // (1, 1, 0)
            [0.09, 0.16, 0.13, 0.11, 0.05, 0.21, 0.04, 0.21]  // (1, 1, 1)
        ];
        let cpd = CatCPD::new(x, z, p.clone())?;

        let pred_cpd = cpd.marginalize(&set![0, 1], &set![0, 2])?;
        let true_x = support![("C", ["no", "yes"])];
        let true_z = support![("E", ["no", "yes"])];
        let true_p = array![
            [
                p[[0, 0]] + p[[0, 2]] + p[[0, 4]] + p[[0, 6]] + // (C0, E0)
                p[[1, 0]] + p[[1, 2]] + p[[1, 4]] + p[[1, 6]] + // (C0, E0)
                p[[4, 0]] + p[[4, 2]] + p[[4, 4]] + p[[4, 6]] + // (C0, E0)
                p[[5, 0]] + p[[5, 2]] + p[[5, 4]] + p[[5, 6]], //  (C0, E0)
                p[[0, 1]] + p[[0, 3]] + p[[0, 5]] + p[[0, 7]] + // (C0, E1)
                p[[1, 1]] + p[[1, 3]] + p[[1, 5]] + p[[1, 7]] + // (C0, E1)
                p[[4, 1]] + p[[4, 3]] + p[[4, 5]] + p[[4, 7]] + // (C0, E1)
                p[[5, 1]] + p[[5, 3]] + p[[5, 5]] + p[[5, 7]] //   (C0, E1)
            ],
            [
                p[[2, 0]] + p[[2, 2]] + p[[2, 4]] + p[[2, 6]] + // (C1, E0)
                p[[3, 0]] + p[[3, 2]] + p[[3, 4]] + p[[3, 6]] + // (C1, E0)
                p[[6, 0]] + p[[6, 2]] + p[[6, 4]] + p[[6, 6]] + // (C1, E0)
                p[[7, 0]] + p[[7, 2]] + p[[7, 4]] + p[[7, 6]], //  (C1, E0)
                p[[2, 1]] + p[[2, 3]] + p[[2, 5]] + p[[2, 7]] + // (C1, E1)
                p[[3, 1]] + p[[3, 3]] + p[[3, 5]] + p[[3, 7]] + // (C1, E1)
                p[[6, 1]] + p[[6, 3]] + p[[6, 5]] + p[[6, 7]] + // (C1, E1)
                p[[7, 1]] + p[[7, 3]] + p[[7, 5]] + p[[7, 7]] //   (C1, E1)
            ]
        ];
        let true_p = &true_p / &true_p.sum_axis(Axis(1)).insert_axis(Axis(1));
        let true_cpd = CatCPD::new(true_x, true_z, true_p)?;

        assert_relative_eq!(true_cpd, pred_cpd);

        Ok(())
    }

    #[test]
    fn support() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"]), ("C", ["no", "yes"])];
        let p = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let cpd = CatCPD::new(x, z, p)?;

        let s = CPD::support(&cpd);
        assert_eq!(*s, support![("A", ["no", "yes"])]);
        let cs = CPD::conditioning_support(&cpd);
        assert_eq!(
            cs.values()
                .map(|v| v.iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![vec!["no", "yes"], vec!["no", "yes"]]
        );

        Ok(())
    }

    #[test]
    fn parameters_size() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"]), ("C", ["no", "yes"])];
        // 4 rows (B,C combinations) x 2 columns (A states) = 4 independent params (1 per row)
        let p = array![[0.1, 0.9], [0.2, 0.8], [0.3, 0.7], [0.4, 0.6]];
        let cpd = CatCPD::new(x, z, p)?;
        assert_eq!(cpd.parameters_size(), 4);
        Ok(())
    }

    #[test]
    fn pf() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![];
        let p = array![[0.25, 0.75]];
        let cpd = CatCPD::new(x, z, p)?;

        let prob = cpd.pf(&array![0], &array![])?;
        assert_relative_eq!(prob, 0.25);
        let prob = cpd.pf(&array![1], &array![])?;
        assert_relative_eq!(prob, 0.75);

        Ok(())
    }

    #[test]
    fn pf_conditional() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"])];
        let p = array![[0.1, 0.9], [0.4, 0.6]];
        let cpd = CatCPD::new(x, z, p)?;

        let prob = cpd.pf(&array![0], &array![0])?;
        assert_relative_eq!(prob, 0.1);
        let prob = cpd.pf(&array![1], &array![0])?;
        assert_relative_eq!(prob, 0.9);
        let prob = cpd.pf(&array![0], &array![1])?;
        assert_relative_eq!(prob, 0.4);
        let prob = cpd.pf(&array![1], &array![1])?;
        assert_relative_eq!(prob, 0.6);

        Ok(())
    }

    #[test]
    fn pf_wrong_size() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![];
        let p = array![[0.25, 0.75]];
        let cpd = CatCPD::new(x, z, p)?;

        // Wrong number of x values
        assert!(cpd.pf(&array![0, 1], &array![]).is_err());
        // Wrong number of z values
        assert!(cpd.pf(&array![0], &array![0, 1]).is_err());
        Ok(())
    }

    #[test]
    fn sample() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let x = support![("A", ["no", "yes"])];
        let z = support![];
        let p = array![[0.25, 0.75]];
        let cpd = CatCPD::new(x, z, p)?;

        let sample = cpd.sample(&mut rng, &array![])?;
        assert_eq!(sample.len(), 1);
        assert!(sample[0] == 0 || sample[0] == 1);

        Ok(())
    }

    #[test]
    fn sample_conditional() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let x = support![("A", ["no", "yes"])];
        let z = support![("B", ["no", "yes"])];
        let p = array![[0.1, 0.9], [0.4, 0.6]];
        let cpd = CatCPD::new(x, z, p)?;

        let sample = cpd.sample(&mut rng, &array![0])?;
        assert_eq!(sample.len(), 1);

        Ok(())
    }

    #[test]
    fn fitted_statistics_none() -> Result<()> {
        let x = support![("A", ["no", "yes"])];
        let z = support![];
        let p = array![[0.25, 0.75]];
        let cpd = CatCPD::new(x, z, p)?;

        assert!(cpd.fitted_statistics().is_none());
        assert!(cpd.fitted_log_likelihood().is_none());

        Ok(())
    }
}

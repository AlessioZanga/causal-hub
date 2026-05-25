#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        labels,
        models::{CPD, CatCPD, GaussCPD, GaussCPDP, Labelled, MixedCPD, MixedSample},
        states,
        types::Result,
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;

    #[test]
    fn from_categorical() -> Result<()> {
        let cat = CatCPD::new(
            // P(A | B, C)
            states![("A", ["no", "yes"])],                       //
            states![("B", ["no", "yes"]), ("C", ["no", "yes"])], //
            array![
                [0.1, 0.9], // (B=0, C=0)
                [0.2, 0.8], // (B=0, C=1)
                [0.3, 0.7], // (B=1, C=0)
                [0.4, 0.6], // (B=1, C=1)
            ],
        )?;
        let mixed = MixedCPD::from(cat);

        assert_eq!(mixed.labels(), &labels!["A"]);
        assert_eq!(mixed.conditioning_labels(), &labels!["B", "C"]);
        assert_eq!(mixed.parameters_size(), 4);

        Ok(())
    }

    #[test]
    fn pf_categorical() -> Result<()> {
        let mixed = MixedCPD::from(CatCPD::new(
            // P(A | B)
            states![("A", ["no", "yes"])], //
            states![("B", ["no", "yes"])], //
            array![
                [0.1, 0.9], // B=0
                [0.2, 0.8], // B=1
            ],
        )?);

        // P(A=0 | B=0) = 0.1
        let p = mixed.pf(
            &MixedSample::Categorical(array![0]),
            &MixedSample::Categorical(array![0]),
        )?;
        assert!((p - 0.1).abs() < 1e-10);

        // P(A=1 | B=0) = 0.9
        let p = mixed.pf(
            &MixedSample::Categorical(array![1]),
            &MixedSample::Categorical(array![0]),
        )?;
        assert!((p - 0.9).abs() < 1e-10);

        Ok(())
    }

    #[test]
    fn sample_categorical() -> Result<()> {
        let mixed = MixedCPD::from(CatCPD::new(
            // P(A), deterministic A=1
            states![("A", ["no", "yes"])], //
            states![],                     //
            array![[0.0, 1.0]],            //
        )?);

        let mut rng = rand::rngs::Xoshiro256PlusPlus::seed_from_u64(42);
        let sample = mixed.sample(&mut rng, &MixedSample::Categorical(array![]))?;

        match sample {
            MixedSample::Categorical(s) => {
                assert_eq!(s.len(), 1);
                assert_eq!(s[0], 1);
            }
            _ => panic!("Expected categorical sample"),
        }

        Ok(())
    }

    #[test]
    fn from_gaussian() -> Result<()> {
        // A = 1.0 + 0.5 * B + eps, eps ~ N(0, 0.1)
        let params = GaussCPDP::new(array![[0.5]], array![1.0], array![[0.1]])?;
        let gauss = GaussCPD::new(labels!["A"], labels!["B"], params)?;
        let mixed = MixedCPD::from(gauss);

        // Check the labels.
        assert_eq!(mixed.labels(), &labels!["A"]);
        assert_eq!(mixed.conditioning_labels(), &labels!["B"]);

        Ok(())
    }

    #[test]
    fn pf_gaussian() -> Result<()> {
        // A = 1.0 + 0.5 * B + eps, eps ~ N(0, 0.1)
        let params = GaussCPDP::new(array![[0.5]], array![1.0], array![[0.1]])?;
        let mixed = MixedCPD::from(GaussCPD::new(labels!["A"], labels!["B"], params)?);

        // P(A=1.5 | B=1.0) should be finite and positive
        let p = mixed.pf(
            &MixedSample::Gaussian(array![1.5]),
            &MixedSample::Gaussian(array![1.0]),
        )?;
        assert!(p.is_finite() && p > 0.);

        Ok(())
    }

    #[test]
    fn sample_gaussian() -> Result<()> {
        // A ~ N(0, 1), no conditioning
        let params = GaussCPDP::new(Array2::<f64>::zeros((1, 0)), array![0.0], array![[1.0]])?;
        let mixed = MixedCPD::from(GaussCPD::new(labels!["A"], labels![], params)?);

        let mut rng = rand::rngs::Xoshiro256PlusPlus::seed_from_u64(42);
        let sample = mixed.sample(&mut rng, &MixedSample::Gaussian(array![]))?;

        match sample {
            MixedSample::Gaussian(s) => {
                assert_eq!(s.len(), 1);
                assert!(s[0].is_finite());
            }
            _ => panic!("Expected Gaussian sample"),
        }

        Ok(())
    }

    #[test]
    fn type_mismatch_error() -> Result<()> {
        let mixed = MixedCPD::from(CatCPD::new(
            // P(A)
            states![("A", ["no", "yes"])], //
            states![],                     //
            array![[0.1, 0.9]],            //
        )?);

        // Passing Gaussian sample to a categorical CPD should fail.
        let result = mixed.pf(
            &MixedSample::Gaussian(array![0.0]),
            &MixedSample::Gaussian(array![]),
        );
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn equality() -> Result<()> {
        let a = MixedCPD::from(CatCPD::new(
            // P(A)
            states![("A", ["no", "yes"])], //
            states![],                     //
            array![[0.1, 0.9]],            //
        )?);
        let b = MixedCPD::from(CatCPD::new(
            // P(A)
            states![("A", ["no", "yes"])], //
            states![],                     //
            array![[0.1, 0.9]],            //
        )?);

        assert_eq!(a, b);
        assert_relative_eq!(a, b);

        Ok(())
    }

    #[test]
    fn from_samples() -> Result<()> {
        use causal_hub::datasets::{CatSample, GaussSample};

        let cat_sample: MixedSample = CatSample::from_vec(vec![0, 1]).into();
        match cat_sample {
            MixedSample::Categorical(s) => assert_eq!(s, array![0, 1]),
            _ => panic!("Expected categorical"),
        }

        let gauss_sample: MixedSample = GaussSample::from_vec(vec![1.5, 2.5]).into();
        match gauss_sample {
            MixedSample::Gaussian(s) => assert_eq!(s, array![1.5, 2.5]),
            _ => panic!("Expected gaussian"),
        }

        Ok(())
    }
}

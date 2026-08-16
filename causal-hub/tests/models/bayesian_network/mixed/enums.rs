#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        datasets::{
            CatEv, CatEvT, CatIncTable, CatTable, CatWtdTable, GaussEv, GaussEvT, GaussIncTable,
            GaussTable, GaussWtdTable, IncDataset,
        },
        labels,
        models::{
            CatCPDS, GaussCPDS, Labelled, MixedCPDS, MixedEv, MixedIncTable, MixedSample,
            MixedTable, MixedWtdTable,
        },
        support,
        types::Result,
    };
    use ndarray::prelude::*;

    const M_CAT: u8 = CatIncTable::MISSING;

    #[test]
    fn from_categorical_evidence() -> Result<()> {
        let support = support![("X", ["a", "b"]), ("Y", ["0", "1"]),];
        let values = vec![CatEvT::CertainPositive { event: 0, state: 1 }];
        let ev = CatEv::new(support, values)?;
        let mixed = MixedEv::from(ev);

        match mixed {
            MixedEv::Categorical(inner) => assert_eq!(inner.labels(), &labels!["X", "Y"]),
            _ => panic!("expected categorical"),
        }

        Ok(())
    }

    #[test]
    fn from_gaussian_evidence() -> Result<()> {
        let labels = labels!["X", "Y", "Z"];
        let values = vec![GaussEvT::CertainPositive {
            event: 1,
            value: 2.5,
        }];
        let ev = GaussEv::new(labels, values)?;
        let mixed = MixedEv::from(ev);

        match mixed {
            MixedEv::Gaussian(inner) => assert_eq!(inner.labels(), &labels!["X", "Y", "Z"]),
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }

    #[test]
    fn from_categorical_table() -> Result<()> {
        let table = CatTable::new(support![("A", ["no", "yes"])], array![[0], [1], [0]])?;
        let mixed = MixedTable::from(table);

        match mixed {
            MixedTable::Categorical(inner) => assert_eq!(inner.labels(), &labels!["A"]),
            _ => panic!("expected categorical"),
        }

        Ok(())
    }

    #[test]
    fn from_gaussian_table() -> Result<()> {
        let table = GaussTable::new(
            labels!["X", "Y"],
            array![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
        )?;
        let mixed = MixedTable::from(table);

        match mixed {
            MixedTable::Gaussian(inner) => assert_eq!(inner.labels(), &labels!["X", "Y"]),
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }

    #[test]
    fn from_categorical_incomplete_table() -> Result<()> {
        let inc_table =
            CatIncTable::new(support![("A", ["no", "yes"])], array![[0], [M_CAT], [1]])?;
        let mixed = MixedIncTable::from(inc_table);

        match mixed {
            MixedIncTable::Categorical(inner) => assert_eq!(inner.labels(), &labels!["A"]),
            _ => panic!("expected categorical"),
        }

        Ok(())
    }

    #[test]
    fn from_gaussian_incomplete_table() -> Result<()> {
        let inc_table = GaussIncTable::new(labels!["X", "Y"], array![[0.0, f64::NAN], [1.0, 2.0]])?;
        let mixed = MixedIncTable::from(inc_table);

        match mixed {
            MixedIncTable::Gaussian(inner) => assert_eq!(inner.labels(), &labels!["X", "Y"]),
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }

    #[test]
    fn from_categorical_weighted_table() -> Result<()> {
        let table = CatTable::new(support![("A", ["no", "yes"])], array![[0], [1], [0]])?;
        let weights = array![1.0, 2.0, 1.0];
        let wtd = CatWtdTable::new(table, weights)?;
        let mixed = MixedWtdTable::from(wtd);

        match mixed {
            MixedWtdTable::Categorical(inner) => assert_eq!(inner.labels(), &labels!["A"]),
            _ => panic!("expected categorical"),
        }

        Ok(())
    }

    #[test]
    fn from_gaussian_weighted_table() -> Result<()> {
        let table = GaussTable::new(labels!["X"], array![[0.0], [1.0], [2.0]])?;
        let weights = array![1.0, 2.0, 1.0];
        let wtd = GaussWtdTable::new(table, weights)?;
        let mixed = MixedWtdTable::from(wtd);

        match mixed {
            MixedWtdTable::Gaussian(inner) => assert_eq!(inner.labels(), &labels!["X"]),
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }

    #[test]
    fn from_categorical_cpds() -> Result<()> {
        let n_xz = array![[1.6, 0.8], [0.0, 1.6]];
        let cpds = CatCPDS::new(n_xz, 4.0)?;
        let mixed = MixedCPDS::from(cpds);

        match mixed {
            MixedCPDS::Categorical(inner) => {
                assert_relative_eq!(inner.fitted_size(), 4.0);
                assert_eq!(inner.fitted_conditional_counts().shape(), &[2, 2]);
            }
            _ => panic!("expected categorical"),
        }

        Ok(())
    }

    #[test]
    fn from_gaussian_cpds() -> Result<()> {
        let mu_x = array![0.5];
        let mu_z = array![0.5];
        let s_xx = array![[1.0]];
        let s_xz = array![[0.5]];
        let s_zz = array![[1.0]];
        let cpds = GaussCPDS::new(mu_x, mu_z, s_xx, s_xz, s_zz, 4.0)?;
        let mixed = MixedCPDS::from(cpds);

        match mixed {
            MixedCPDS::Gaussian(inner) => {
                assert_relative_eq!(inner.fitted_size(), 4.0);
                assert_eq!(inner.fitted_response_mean().len(), 1);
            }
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }

    #[test]
    fn from_cat_sample() -> Result<()> {
        use causal_hub::datasets::CatSample;

        let sample: MixedSample = CatSample::from_vec(vec![0, 1, 0]).into();
        match sample {
            MixedSample::Categorical(stats) => assert_eq!(stats, array![0, 1, 0]),
            _ => panic!("expected categorical"),
        }

        Ok(())
    }

    #[test]
    fn from_gauss_sample() -> Result<()> {
        use causal_hub::datasets::GaussSample;

        let sample: MixedSample = GaussSample::from_vec(vec![1.5, 2.5]).into();
        match sample {
            MixedSample::Gaussian(stats) => assert_eq!(stats, array![1.5, 2.5]),
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }

    #[test]
    fn mixed_sample_clone_and_debug() -> Result<()> {
        use causal_hub::datasets::CatSample;

        let sample: MixedSample = CatSample::from_vec(vec![0, 1]).into();
        let cloned = sample.clone();
        assert_eq!(format!("{:?}", cloned), format!("{:?}", sample));

        Ok(())
    }

    #[test]
    fn cross_variant_rejection() -> Result<()> {
        // Construct a categorical CPD and verify it doesn't accidentally match Gaussian
        let table = CatTable::new(support![("A", ["no", "yes"])], array![[0], [1]])?;
        let mixed = MixedTable::from(table);

        match mixed {
            MixedTable::Categorical(_) => {} // expected
            _ => panic!("expected categorical"),
        }

        // Also verify the reverse: Gaussian → Gaussian
        let table = GaussTable::new(labels!["X"], array![[0.0], [1.0]])?;
        let mixed = MixedTable::from(table);

        match mixed {
            MixedTable::Gaussian(_) => {} // expected
            _ => panic!("expected gaussian"),
        }

        Ok(())
    }
}

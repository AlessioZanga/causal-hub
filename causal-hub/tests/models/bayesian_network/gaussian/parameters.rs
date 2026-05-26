#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use causal_hub::{
        labels,
        models::{CPD, GaussCPD, GaussCPDP},
        types::Result,
    };
    use ndarray::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn support() -> Result<()> {
        // P(A | B, C)
        let l = labels!("A");
        let z = labels!("B", "C");
        let a = array![[3., -1.]];
        let b = array![2.];
        let s = array![[4.]];
        let p = GaussCPDP::new(a, b, s)?;
        let cpd = GaussCPD::new(l.clone(), z.clone(), p)?;

        let support = CPD::support(&cpd);
        for (_, &(lo, hi)) in &*support {
            assert!(lo.is_infinite() && lo.is_sign_negative());
            assert!(hi.is_infinite() && hi.is_sign_positive());
        }

        let conditioning_support = CPD::conditioning_support(&cpd);
        for (_, &(lo, hi)) in &*conditioning_support {
            assert!(lo.is_infinite() && lo.is_sign_negative());
            assert!(hi.is_infinite() && hi.is_sign_positive());
        }

        Ok(())
    }

    #[test]
    fn parameters_size() -> Result<()> {
        let l = labels!("A");
        let z = labels!("B", "C");
        let a = array![[3., -1.]];
        let b = array![2.];
        let s = array![[4.]];
        let p = GaussCPDP::new(a, b, s)?;
        let cpd = GaussCPD::new(l, z, p)?;
        assert_eq!(cpd.parameters_size(), 4);
        Ok(())
    }

    #[test]
    fn pf_unconditional() -> Result<()> {
        let l = labels!("A");
        let z = labels![];
        // For unconditional Gaussian, coefficient matrix has 0 columns
        let a: Array2<f64> = Array2::from_shape_vec((1, 0), vec![])?;
        let b = array![0.];
        let s = array![[1.]];
        let cpd = GaussCPD::new(l, z, GaussCPDP::new(a, b, s)?)?;

        // Mean 0, var 1, density at 0 = 1/sqrt(2*pi) ≈ 0.3989
        let density = cpd.pf(&array![0.0], &array![])?;
        assert_relative_eq!(density, 0.3989422804014327, epsilon = 1e-8);

        Ok(())
    }

    #[test]
    fn pf_conditional() -> Result<()> {
        let l = labels!("A");
        let z = labels!("B");
        let a = array![[2.]]; // A = 2 * B + 0
        let b = array![0.];
        let s = array![[1.]];
        let cpd = GaussCPD::new(l, z, GaussCPDP::new(a, b, s)?)?;

        // When B=1, mean = 2, var = 1
        let density = cpd.pf(&array![2.0], &array![1.0])?;
        assert_relative_eq!(density, 0.3989422804014327, epsilon = 1e-8);

        Ok(())
    }

    #[test]
    fn sample() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let l = labels!("A");
        let z = labels!["B"];
        let a = array![[2.]];
        let b = array![0.];
        let s = array![[1.]];
        let cpd = GaussCPD::new(l, z, GaussCPDP::new(a, b, s)?)?;

        let smp = cpd.sample(&mut rng, &array![1.0])?;
        assert_eq!(smp.len(), 1);
        assert!(smp[0].is_finite());

        Ok(())
    }

    #[test]
    fn fitted_statistics_none() -> Result<()> {
        let l = labels!("A");
        let z = labels!["B"];
        let a = array![[2.]];
        let b = array![0.];
        let s = array![[1.]];
        let cpd = GaussCPD::new(l, z, GaussCPDP::new(a, b, s)?)?;

        assert!(cpd.fitted_statistics().is_none());
        assert!(cpd.fitted_log_likelihood().is_none());

        Ok(())
    }
}

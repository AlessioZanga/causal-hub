#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{CPD, GaussCPD, GaussCPDP},
        types::Result,
    };
    use ndarray::prelude::*;

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
}

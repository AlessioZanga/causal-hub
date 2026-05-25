#[cfg(test)]
mod tests {
    use causal_hub::{
        models::{CIM, CatCIM},
        support,
        types::Result,
    };
    use ndarray::prelude::*;

    #[test]
    fn support() -> Result<()> {
        let x = support![("A", ["a1", "a2"])];
        let z = support![("B", ["b1", "b2"])];
        let p = array![[[-1.0, 1.0], [2.0, -2.0]], [[-3.0, 3.0], [4.0, -4.0]]];
        let cim = CatCIM::new(x.clone(), z.clone(), p)?;

        let s = CIM::support(&cim);
        assert_eq!(*s, x);

        let cs = CIM::conditioning_support(&cim);
        assert_eq!(*cs, z);

        Ok(())
    }
}

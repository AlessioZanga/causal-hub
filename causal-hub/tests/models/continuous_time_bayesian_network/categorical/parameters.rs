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
        let probability = array![[[-1.0, 1.0], [2.0, -2.0]], [[-3.0, 3.0], [4.0, -4.0]]];
        let intensity = CatCIM::new(x.clone(), z.clone(), probability)?;

        let stats = CIM::support(&intensity);
        assert_eq!(*stats, x);

        let cs = CIM::conditioning_support(&intensity);
        assert_eq!(*cs, z);

        Ok(())
    }
}

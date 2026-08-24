#[cfg(test)]
mod tests {
    use causal_hub::{
        labels,
        models::{Graph, HasLabels},
        random::{Random, RngUnGraph},
        types::Result,
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn new() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B", "C"];
        let _ = RngUnGraph::new(&mut rng, &labels, 0.5)?;

        Ok(())
    }

    #[test]
    fn random() -> Result<()> {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let labels = labels!["A", "B", "C"];
        let mut rng_graph = RngUnGraph::new(&mut rng, &labels, 0.5)?;

        let graph = rng_graph.random()?;

        assert_eq!(graph.vertices().len(), 3);
        assert_eq!(graph.labels(), &labels);

        Ok(())
    }
}

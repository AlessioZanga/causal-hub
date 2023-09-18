#[cfg(test)]
mod categorical {
    use causal_hub::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn sample() {
        // Initialize random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Read BN from BIF.
        let true_b: CategoricalBN = BIF::read("./tests/assets/bif/asia.bif").unwrap().into();
        // Sample using forward sampling.
        true_b.sample(&mut rng, 1e3 as usize);
    }
}

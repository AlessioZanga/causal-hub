#[cfg(test)]
mod discrete {
    use causal_hub::prelude::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    #[test]
    fn sample() {
        // Initialize random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Read BN from BIF.
        let bn: DiscreteBN = BIF::read("./tests/assets/bif/asia.bif").unwrap().into();
        // Sample using forward sampling.
        bn.sample(&mut rng, 1000);
    }
}

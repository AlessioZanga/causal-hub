pub mod sample {

    use causal_hub::prelude::*;
    use criterion::{BenchmarkId, Criterion, Throughput};
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    fn driver(c: &mut Criterion, model: &str) {
        // Initialize benchmark group.
        let mut group =
            c.benchmark_group(format!("models::bayesian_network::sample::{model}").as_str());

        // Initialize random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Load reference model.
        let model: DiscreteBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
            .unwrap()
            .into();

        // Repeat for different sample sizes.
        for sample_size in [100, 1_000, 10_000].iter() {
            // Set input dimension.
            group.throughput(Throughput::Elements(*sample_size as u64));
            // Benchmark function.
            group.bench_with_input(
                BenchmarkId::from_parameter(sample_size),
                sample_size,
                |b, sample_size| {
                    b.iter(|| {
                        // Sample data set from reference model.
                        let _ = model.sample(&mut rng, *sample_size);
                    })
                },
            );
        }
    }

    pub fn asia(c: &mut Criterion) {
        driver(c, "asia");
    }

    pub fn alarm(c: &mut Criterion) {
        driver(c, "alarm");
    }
}

pub mod par_sample {

    use causal_hub::prelude::*;
    use criterion::{BenchmarkId, Criterion, Throughput};
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    fn driver(c: &mut Criterion, model: &str) {
        // Initialize benchmark group.
        let mut group =
            c.benchmark_group(format!("models::bayesian_network::par_sample::{model}").as_str());

        // Initialize random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Load reference model.
        let model: DiscreteBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
            .unwrap()
            .into();

        // Repeat for different sample sizes.
        for sample_size in [100, 1_000, 10_000].iter() {
            // Set input dimension.
            group.throughput(Throughput::Elements(*sample_size as u64));
            // Benchmark function.
            group.bench_with_input(
                BenchmarkId::from_parameter(sample_size),
                sample_size,
                |b, sample_size| {
                    b.iter(|| {
                        // Sample data set from reference model.
                        let _ = model.par_sample(&mut rng, *sample_size);
                    })
                },
            );
        }
    }

    pub fn asia(c: &mut Criterion) {
        driver(c, "asia");
    }

    pub fn alarm(c: &mut Criterion) {
        driver(c, "alarm");
    }
}

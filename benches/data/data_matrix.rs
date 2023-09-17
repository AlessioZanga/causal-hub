pub mod sample {
    use causal_hub::prelude::*;
    use criterion::{black_box, BenchmarkId, Criterion, Throughput};
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    fn driver(c: &mut Criterion, model: &str) {
        // Initialize benchmark group.
        let mut group = c.benchmark_group(format!("data::data_matrix::sample::{model}").as_str());

        // Initialize random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Load reference model.
        let model: DiscreteBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
            .unwrap()
            .into();
        // Load reference model.
        let data_set: DiscreteDataMatrix = model.sample(&mut rng, 10000);

        // Repeat for different sample sizes.
        for sample_size in [100, 1_000, 10_000].iter() {
            // Set input dimension.
            group.throughput(Throughput::Elements(*sample_size as u64));
            // Benchmark function.
            group.bench_with_input(
                BenchmarkId::from_parameter(sample_size),
                sample_size,
                |b, &sample_size| {
                    b.iter(|| data_set.sample(black_box(&mut rng), black_box(sample_size)))
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

pub mod sample_with_replacement {
    use causal_hub::prelude::*;
    use criterion::{black_box, BenchmarkId, Criterion, Throughput};
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    fn driver(c: &mut Criterion, model: &str) {
        // Initialize benchmark group.
        let mut group = c.benchmark_group(
            format!("data::data_matrix::sample_with_replacement::{model}").as_str(),
        );

        // Initialize random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
        // Load reference model.
        let model: DiscreteBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
            .unwrap()
            .into();
        // Load reference model.
        let data_set: DiscreteDataMatrix = model.sample(&mut rng, 10000);

        // Repeat for different sample sizes.
        for sample_size in [100, 1_000, 10_000].iter() {
            // Set input dimension.
            group.throughput(Throughput::Elements(*sample_size as u64));
            // Benchmark function.
            group.bench_with_input(
                BenchmarkId::from_parameter(sample_size),
                sample_size,
                |b, &sample_size| {
                    b.iter(|| {
                        data_set
                            .sample_with_replacement(black_box(&mut rng), black_box(sample_size))
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

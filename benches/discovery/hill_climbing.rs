pub mod categorical {

    pub mod call {

        use causal_hub::prelude::*;
        use criterion::{black_box, BenchmarkId, Criterion, Throughput};
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        fn call<D, G, K, S>(data_set: &D, prior_knowledge: &K, scoring_criterion: &S) -> G
        where
            D: DataSet,
            G: DirectedGraph<Direction = Directed> + PathGraph,
            K: PriorKnowledge,
            S: DecomposableScoringCriterion<D, G>,
        {
            // Initialize functor.
            let hill_climbing = HC::new(scoring_criterion);
            // Call functor.
            hill_climbing.call(data_set, prior_knowledge)
        }

        fn driver(c: &mut Criterion, model: &str) {
            // Initialize benchmark group.
            let mut group = c.benchmark_group(
                format!("discovery::hill_climbing::categorical::call::{model}").as_str(),
            );

            // Initialize random number generator.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Load reference model.
            let model: CategoricalBN =
                BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
                    .unwrap()
                    .into();

            // Repeat for different sample sizes.
            for sample_size in [100, 1_000, 10_000].iter() {
                // Sample data set from reference model.
                let data_set = model.sample(&mut rng, *sample_size);
                // Initialize empty prior knowledge.
                let prior_knowledge = FR::new(L!(data_set), [], []);
                // Initialize scoring criterion functor.
                let scoring_criterion = BIC::new(&data_set);

                // Set input dimension.
                group.throughput(Throughput::Elements(*sample_size as u64));
                // Benchmark function.
                group.bench_with_input(
                    BenchmarkId::from_parameter(sample_size),
                    sample_size,
                    |b, _| {
                        b.iter(|| {
                            let _: DGraph = call(
                                black_box(&data_set),
                                black_box(&prior_knowledge),
                                black_box(&scoring_criterion),
                            );
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

    pub mod par_call {

        use causal_hub::prelude::*;
        use criterion::{black_box, BenchmarkId, Criterion, Throughput};
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        fn par_call<D, G, K, S>(data_set: &D, prior_knowledge: &K, scoring_criterion: &S) -> G
        where
            D: DataSet + Sync,
            G: DirectedGraph<Direction = Directed> + PathGraph + Sync,
            K: PriorKnowledge + Sync,
            S: DecomposableScoringCriterion<D, G> + Sync,
        {
            // Initialize functor.
            let hill_climbing = HC::new(scoring_criterion);
            // Call functor.
            hill_climbing.par_call(data_set, prior_knowledge)
        }

        fn driver(c: &mut Criterion, model: &str) {
            // Initialize benchmark group.
            let mut group = c.benchmark_group(
                format!("discovery::hill_climbing::categorical::par_call::{model}").as_str(),
            );

            // Initialize random number generator.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
            // Load reference model.
            let model: CategoricalBN =
                BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
                    .unwrap()
                    .into();

            // Repeat for different sample sizes.
            for sample_size in [100, 1_000, 10_000].iter() {
                // Sample data set from reference model.
                let data_set = model.sample(&mut rng, *sample_size);
                // Initialize empty prior knowledge.
                let prior_knowledge = FR::new(L!(data_set), [], []);
                // Initialize scoring criterion functor.
                let scoring_criterion = BIC::new(&data_set);

                // Set input dimension.
                group.throughput(Throughput::Elements(*sample_size as u64));
                // Benchmark function.
                group.bench_with_input(
                    BenchmarkId::from_parameter(sample_size),
                    sample_size,
                    |b, _| {
                        b.iter(|| {
                            let _: DGraph = par_call(
                                black_box(&data_set),
                                black_box(&prior_knowledge),
                                black_box(&scoring_criterion),
                            );
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
}

pub mod zinb {

    pub mod call {

        use causal_hub::{polars::prelude::*, prelude::*};
        use criterion::{black_box, BenchmarkId, Criterion, Throughput};
        use rand::SeedableRng;
        use rand_xoshiro::Xoshiro256PlusPlus;

        fn call<D, G, K, S>(data_set: &D, prior_knowledge: &K, scoring_criterion: &S) -> G
        where
            D: DataSet,
            G: DirectedGraph<Direction = Directed> + PathGraph,
            K: PriorKnowledge,
            S: DecomposableScoringCriterion<D, G>,
        {
            // Initialize functor.
            let hill_climbing = HC::new(scoring_criterion);
            // Call functor.
            hill_climbing.call(data_set, prior_knowledge)
        }

        pub fn dummy(c: &mut Criterion) {
            // Initialize benchmark group.
            let mut group = c.benchmark_group("discovery::hill_climbing::zinb::call");

            // Initialize random number generator.
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

            // Set dtypes.
            let dtypes = vec![DataType::Float64; 5];
            // Load reference data set.
            let data_matrix: ZINBDataSet = CsvReader::from_path("./tests/assets/zinb.csv")
                .unwrap()
                .has_header(true)
                .with_dtypes_slice(Some(&dtypes))
                .finish()
                .unwrap()
                .into();

            // Repeat for different sample sizes.
            for sample_size in [100, 250, 500, 1_000].iter() {
                // Sample data set from reference data set.
                let data_set = data_matrix.sample_with_replacement(&mut rng, *sample_size);
                // Initialize empty prior knowledge.
                let prior_knowledge = FR::new(L!(data_set), [], []);
                // Initialize scoring criterion functor.
                let scoring_criterion = BIC::new(&data_set);

                // Set input dimension.
                group.throughput(Throughput::Elements(*sample_size as u64));
                // Benchmark function.
                group.bench_with_input(
                    BenchmarkId::from_parameter(sample_size),
                    sample_size,
                    |b, _| {
                        b.iter(|| {
                            let _: DGraph = call(
                                black_box(&data_set),
                                black_box(&prior_knowledge),
                                black_box(&scoring_criterion),
                            );
                        })
                    },
                );
            }
        }
    }
}

use causal_hub::prelude::*;
use criterion::{black_box, BenchmarkId, Criterion, Throughput};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

fn hill_climbing<D, G, K, S>(data_set: &D, prior_knowledge: &K, scoring_criterion: &S) -> G
where
    D: DataSet,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    K: PriorKnowledge,
    S: DecomposableScoringCriterion<D, G>,
{
    // Initialize functor.
    let hill_climbing = HC::new(scoring_criterion);
    // Call functor.
    hill_climbing.call(data_set, prior_knowledge)
}

fn hill_climbing_driver(c: &mut Criterion, model: &str) {
    // Initialize benchmark group.
    let mut group = c.benchmark_group(format!("discovery::hill_climbing::{model}").as_str());

    // Initialize random number generator.
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    // Load reference model.
    let model: DiscreteBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
        .unwrap()
        .into();

    // Repeat for different sample sizes.
    for sample_size in [10, 100, 1_000, 10_000, 100_000].iter() {
        // Sample data set from reference model.
        let data_set = model.sample(&mut rng, *sample_size);
        // Initialize empty prior knowledge.
        let prior_knowledge = FR::new(data_set.labels(), [], []);
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
                    let _: DiGraph = hill_climbing(
                        black_box(&data_set),
                        black_box(&prior_knowledge),
                        black_box(&scoring_criterion),
                    );
                })
            },
        );
    }
}

fn par_hill_climbing<D, G, K, S>(data_set: &D, prior_knowledge: &K, scoring_criterion: &S) -> G
where
    D: DataSet,
    G: DirectedGraph<Direction = directions::Directed> + PathGraph,
    K: PriorKnowledge,
    S: DecomposableScoringCriterion<D, G>,
{
    // Initialize functor.
    let hill_climbing = ParallelHC::new(scoring_criterion);
    // Call functor.
    hill_climbing.call(data_set, prior_knowledge)
}

fn par_hill_climbing_driver(c: &mut Criterion, model: &str) {
    // Initialize benchmark group.
    let mut group = c.benchmark_group(format!("discovery::hill_climbing::par_{model}").as_str());

    // Initialize random number generator.
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    // Load reference model.
    let model: DiscreteBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
        .unwrap()
        .into();

    // Repeat for different sample sizes.
    for sample_size in [10, 100, 1_000, 10_000, 100_000].iter() {
        // Sample data set from reference model.
        let data_set = model.sample(&mut rng, *sample_size);
        // Initialize empty prior knowledge.
        let prior_knowledge = FR::new(data_set.labels(), [], []);
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
                    let _: DiGraph = par_hill_climbing(
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
    hill_climbing_driver(c, "asia");
}

pub fn alarm(c: &mut Criterion) {
    hill_climbing_driver(c, "alarm");
}

pub fn par_asia(c: &mut Criterion) {
    par_hill_climbing_driver(c, "asia");
}

pub fn par_alarm(c: &mut Criterion) {
    par_hill_climbing_driver(c, "alarm");
}

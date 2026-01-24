use std::hint::black_box as _b;

use causal_hub::{
    assets::{load_asia, load_child, load_hailfinder},
    datasets::{CatEv, CatEvT, CatTable},
    estimators::{BE, BNEstimator, CSSEstimator, MLE, SSE},
    inference::{ApproximateInference, BNCausalInference, BNInference, CausalInference},
    models::{BN, CatBN, Labelled},
    samplers::{BNSampler, ForwardSampler, ImportanceSampler},
    set,
    types::{Result, Set},
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::{SeedableRng, seq::IteratorRandom};
use rand_xoshiro::Xoshiro256PlusPlus;

fn bench_catbn(c: &mut Criterion, model: CatBN) -> Result<()> {
    // -----------------------------------------------------------------------------------------
    // Setup
    // -----------------------------------------------------------------------------------------
    // Get the model name.
    let name = model.name().unwrap_or("unknown");

    // Setup random generator for variable selection and sampling.
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    // Compute the number of variables to usage.
    let n = std::cmp::max(3, model.labels().len() / 5);
    // Select the variables randomly.
    let v: Vec<usize> = (0..model.labels().len()).choose_multiple(&mut rng, n);
    // Split the variables into X, Y, Z.
    let x: Set<usize> = set![v[0]];
    let y: Set<usize> = set![v[1]];
    let z: Set<usize> = v[2..].iter().cloned().collect();
    // Create a forward sampler to generate data.
    let sampler = ForwardSampler::new(&mut rng, &model)?;
    // Generate a dataset of 5000 samples.
    let data: CatTable = sampler.sample_n(5000)?;
    // Get the graph structure.
    let graph = model.graph().clone();

    // -----------------------------------------------------------------------------------------
    // Estimators
    // -----------------------------------------------------------------------------------------
    let mut group = c.benchmark_group(format!("{name}/estimators"));

    // SSE (Sufficient Statistics Estimator)
    let sse = SSE::new(&data);

    group.bench_function("sse", |b| {
        b.iter(|| -> Result<()> {
            sse.fit(_b(&x), _b(&z))?;
            Ok(())
        })
    });

    // MLE (Maximum Likelihood Estimator)
    let mle = MLE::new(&data);
    group.bench_function("mle", |b| {
        b.iter(|| -> Result<()> {
            BNEstimator::<CatBN>::fit(&mle, _b(graph.clone()))?;
            Ok(())
        })
    });

    // BE (Bayesian Estimator)
    let be = BE::new(&data);
    group.bench_function("be", |b| {
        b.iter(|| -> Result<()> {
            BNEstimator::<CatBN>::fit(&be, _b(graph.clone()))?;
            Ok(())
        })
    });

    group.finish();

    // -----------------------------------------------------------------------------------------
    // Samplers
    // -----------------------------------------------------------------------------------------
    let mut group = c.benchmark_group(format!("{name}/samplers"));

    // Set the sample sizes.
    let sample_sizes = [1, 10, 100, 1000, 10000];

    // Forward Sampling
    for n in sample_sizes {
        group.bench_with_input(BenchmarkId::new("forward_sampling", n), &n, |b, &n| {
            b.iter(|| -> Result<_> { sampler.sample_n(_b(n)) })
        });
    }

    // Importance Sampling
    // Create trivial evidence: "0" = 0.
    let e = CatEvT::CertainPositive { event: 0, state: 0 };
    // Construct events vector.
    let e = CatEv::new(model.states().clone(), vec![e])?;

    // Setup Importance Sampler
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(200);
    let sampler = ImportanceSampler::new(&mut rng, &model, &e)?;

    for n in sample_sizes {
        group.bench_with_input(BenchmarkId::new("importance_sampling", n), &n, |b, &n| {
            b.iter(|| -> Result<_> { sampler.sample_n(_b(n)) })
        });
    }

    group.finish();

    // -----------------------------------------------------------------------------------------
    // Inference
    // -----------------------------------------------------------------------------------------
    let mut group = c.benchmark_group(format!("{name}/inference"));

    // Approximate Inference
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(300);
    let engine = ApproximateInference::new(&mut rng, &model).with_sample_size(100)?;

    group.bench_function("approximate_inference", |b| {
        b.iter(|| -> Result<_> { engine.estimate(_b(&x), _b(&z)) })
    });

    // Causal Inference
    let engine = CausalInference::new(&engine);
    // ACE(X -> Y).
    group.bench_function("causal_inference (ace)", |b| {
        b.iter(|| -> Result<_> { engine.ace_estimate(_b(&x), _b(&y)) })
    });

    // CACE(X -> Y | Z).
    group.bench_function("causal_inference (cace)", |b| {
        b.iter(|| -> Result<_> { engine.cace_estimate(_b(&x), _b(&y), _b(&z)) })
    });

    group.finish();

    Ok(())
}

fn bench_main(c: &mut Criterion) {
    let _ = || -> Result<()> {
        bench_catbn(c, load_asia()?)?;
        bench_catbn(c, load_child()?)?;
        bench_catbn(c, load_hailfinder()?)?;
        Ok(())
    }();
}

criterion_group!(benches, bench_main);
criterion_main!(benches);

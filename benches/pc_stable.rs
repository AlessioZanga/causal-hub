#[allow(unused)] // FIXME: remove this line
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use causal_hub::prelude::*;
use polars::prelude::*;

// Set ChiSquared significance level
const ALPHA: f64 = 0.05;

// PC-Stable skeleton `asia` benchmark
fn pcstable_skeleton_asia(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/asia.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = DiscreteDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("pcstable_skeleton_asia", |b| b.iter(|| pcs.call_skeleton()));
}

// PC-Stable cpdag `asia` benchmark
fn pcstable_cpdag_asia(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/asia.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = DiscreteDataMatrix::from(d);

    // Create ChiSquared conditional independence testpwd
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("pcstable_cpdag_asia", |b| b.iter(|| pcs.call()));
}

// PC-Stable skeleton `alarm` benchmark
fn pcstable_skeleton_alarm(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/PC-Stable/alarm/alarm.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = DiscreteDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("pcstable_skeleton_alarm", |b| {
        b.iter(|| pcs.call_skeleton())
    });
}

// PC-Stable cpdag `alarm` benchmark
fn pcstable_cpdag_alarm(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/PC-Stable/alarm/alarm.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = DiscreteDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("pcstable_cpdag_alarm", |b| {
        b.iter(|| pcs.call())
    });
}

criterion_group!(benches, pcstable_skeleton_asia, pcstable_cpdag_asia, pcstable_skeleton_alarm, pcstable_cpdag_alarm);
criterion_main!(benches);

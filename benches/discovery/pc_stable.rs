use causal_hub::prelude::*;
use criterion::Criterion;
use polars::prelude::*;

// Set ChiSquared significance level
const ALPHA: f64 = 0.05;

// PC-Stable `cancer` benchmark
pub fn cancer(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/pc_stable/cancer.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = CategoricalDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("discovery::pc_stable::cancer", |b| {
        b.iter(|| pcs.call().meek_procedure_until_3())
    });
}

// PC-Stable parallel `cancer` benchmark
pub fn par_cancer(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/pc_stable/cancer.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = CategoricalDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("discovery::pc_stable::par_cancer", |b| {
        b.iter(|| pcs.par_call().meek_procedure_until_3())
    });
}

// PC-Stable `asia` benchmark
pub fn asia(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/pc_stable/asia.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = CategoricalDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("discovery::pc_stable::asia", |b| {
        b.iter(|| pcs.call().meek_procedure_until_3())
    });
}

// PC-Stable parallel `asia` benchmark
pub fn par_asia(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/pc_stable/asia.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = CategoricalDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("discovery::pc_stable::par_asia", |b| {
        b.iter(|| pcs.par_call().meek_procedure_until_3())
    });
}

// PC-Stable `survey` benchmark
pub fn survey(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/pc_stable/survey.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = CategoricalDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("discovery::pc_stable::survey", |b| {
        b.iter(|| pcs.call().meek_procedure_until_3())
    });
}

// PC-Stable parallel `survey` benchmark
pub fn par_survey(c: &mut Criterion) {
    // Load data set.
    let d = CsvReader::from_path("./tests/assets/pc_stable/survey.csv")
        .unwrap()
        .finish()
        .unwrap();
    let d = CategoricalDataMatrix::from(d);

    // Create ChiSquared conditional independence test
    let test = ChiSquared::new(&d).with_significance_level(ALPHA);

    // Create PC-Stable functor
    let pcs = PCStable::new(&test);

    // Benchmark
    c.bench_function("discovery::pc_stable::par_survey", |b| {
        b.iter(|| pcs.par_call().meek_procedure_until_3())
    });
}

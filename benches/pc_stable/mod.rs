pub mod discrete {
    use causal_hub::prelude::*;
    use criterion::{criterion_group, Criterion};
    use polars::prelude::*;

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    // PC-Stable cpdag `cancer` benchmark
    fn cpdag_cancer(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/cancer.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_cancer", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `earthquake` benchmark
    fn cpdag_earthquake(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/earthquake.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_earthquake", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `asia` benchmark
    fn cpdag_asia(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_asia", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `survey` benchmark
    fn cpdag_survey(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/survey.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_survey", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `sachs` benchmark
    fn cpdag_sachs(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/sachs.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_sachs", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `child` benchmark
    fn cpdag_child(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/child.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_child", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `alarm` benchmark
    fn cpdag_alarm(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/alarm.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_alarm", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `win95pts` benchmark
    fn cpdag_win95pts(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/win95pts.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_win95pts", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable cpdag `insurance` benchmark
    fn cpdag_insurance(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/insurance.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("cpdag_insurance", |b| b.iter(|| pcs.call()));
    }

    criterion_group!(
        discrete,
        cpdag_cancer,
        cpdag_earthquake,
        cpdag_asia,
        cpdag_survey,
        cpdag_sachs,
        cpdag_child,
        cpdag_alarm,
        cpdag_win95pts,
        cpdag_insurance,
    );
}

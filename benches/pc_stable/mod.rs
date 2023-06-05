pub mod discrete {
    use causal_hub::prelude::*;
    use criterion::{criterion_group, Criterion};
    use polars::prelude::*;

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    // PC-Stable skeleton `asia` benchmark
    fn skeleton_asia(c: &mut Criterion) {
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
        c.bench_function("skeleton_asia", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `asia` benchmark
    fn cpdag_asia(c: &mut Criterion) {
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
        c.bench_function("cpdag_asia", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable skeleton `alarm` benchmark
    fn skeleton_alarm(c: &mut Criterion) {
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
        c.bench_function("skeleton_alarm", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `alarm` benchmark
    fn cpdag_alarm(c: &mut Criterion) {
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
        c.bench_function("cpdag_alarm", |b| b.iter(|| pcs.call()));
    }

    // PC-Stable skeleton `cancer` benchmark
    fn skeleton_cancer(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/cancer/cancer.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("skeleton_cancer", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `cancer` benchmark
    fn cpdag_cancer(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/cancer/cancer.csv")
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

    // PC-Stable skeleton `child` benchmark
    fn skeleton_child(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/child/child.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("skeleton_child", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `child` benchmark
    fn cpdag_child(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/child/child.csv")
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

    // PC-Stable skeleton `earthquake` benchmark
    fn skeleton_earthquake(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/earthquake/earthquake.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("skeleton_earthquake", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `earthquake` benchmark
    fn cpdag_earthquake(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/earthquake/earthquake.csv")
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

    // PC-Stable skeleton `sachs` benchmark
    fn skeleton_sachs(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/sachs/sachs.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("skeleton_sachs", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `sachs` benchmark
    fn cpdag_sachs(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/sachs/sachs.csv")
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

    // PC-Stable skeleton `survey` benchmark
    fn skeleton_survey(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/survey/survey.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("skeleton_survey", |b| b.iter(|| pcs.call_skeleton()));
    }

    // PC-Stable cpdag `survey` benchmark
    fn cpdag_survey(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/survey/survey.csv")
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

    criterion_group!(
        discrete,
        skeleton_asia,
        cpdag_asia,
        skeleton_alarm,
        cpdag_alarm,
        skeleton_cancer,
        cpdag_cancer,
        skeleton_child,
        cpdag_child,
        skeleton_earthquake,
        cpdag_earthquake,
        skeleton_sachs,
        cpdag_sachs,
        skeleton_survey,
        cpdag_survey,
    );
}

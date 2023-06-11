pub mod discrete {
    use causal_hub::prelude::*;
    use criterion::{criterion_group, Criterion};
    use polars::prelude::*;

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    // PC-Stable `cancer` benchmark
    fn cancer(c: &mut Criterion) {
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
        c.bench_function("cancer", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `cancer` benchmark
    fn par_cancer(c: &mut Criterion) {
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
        c.bench_function("par_cancer", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `earthquake` benchmark
    fn earthquake(c: &mut Criterion) {
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
        c.bench_function("earthquake", |b| {
            b.iter(|| pcs.call().meek_procedure_until_3())
        });
    }

    // PC-Stable parallel `earthquake` benchmark
    fn par_earthquake(c: &mut Criterion) {
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
        c.bench_function("par_earthquake", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `asia` benchmark
    fn asia(c: &mut Criterion) {
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
        c.bench_function("asia", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `asia` benchmark
    fn par_asia(c: &mut Criterion) {
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
        c.bench_function("par_asia", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `survey` benchmark
    fn survey(c: &mut Criterion) {
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
        c.bench_function("survey", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `survey` benchmark
    fn par_survey(c: &mut Criterion) {
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
        c.bench_function("par_survey", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `sachs` benchmark
    fn sachs(c: &mut Criterion) {
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
        c.bench_function("sachs", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `sachs` benchmark
    fn par_sachs(c: &mut Criterion) {
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
        c.bench_function("par_sachs", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `child` benchmark
    fn child(c: &mut Criterion) {
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
        c.bench_function("child", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `child` benchmark
    fn par_child(c: &mut Criterion) {
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
        c.bench_function("par_child", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `alarm` benchmark
    fn alarm(c: &mut Criterion) {
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
        c.bench_function("alarm", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `alarm` benchmark
    fn par_alarm(c: &mut Criterion) {
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
        c.bench_function("par_alarm", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `win95pts` benchmark
    fn win95pts(c: &mut Criterion) {
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
        c.bench_function("win95pts", |b| {
            b.iter(|| pcs.call().meek_procedure_until_3())
        });
    }

    // PC-Stable parallel `win95pts` benchmark
    fn par_win95pts(c: &mut Criterion) {
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
        c.bench_function("par_win95pts", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `insurance` benchmark
    fn insurance(c: &mut Criterion) {
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
        c.bench_function("insurance", |b| {
            b.iter(|| pcs.call().meek_procedure_until_3())
        });
    }

    // PC-Stable parallel `insurance` benchmark
    fn par_insurance(c: &mut Criterion) {
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
        c.bench_function("par_insurance", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    criterion_group!(
        discrete,
        cancer,
        par_cancer,
        earthquake,
        par_earthquake,
        asia,
        par_asia,
        survey,
        par_survey,
        sachs,
        par_sachs,
        child,
        par_child,
        alarm,
        par_alarm,
        win95pts,
        par_win95pts,
        insurance,
        par_insurance
    );
}

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
            b.iter(|| pcs.par_call().meek_procedure_until_3())
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
            b.iter(|| pcs.par_call().meek_procedure_until_3())
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
            b.iter(|| pcs.par_call().meek_procedure_until_3())
        });
    }

    criterion_group!(discrete, cancer, par_cancer, asia, par_asia, survey, par_survey,);
}

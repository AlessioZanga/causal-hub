pub mod categorical {

    pub mod call {
        use causal_hub::{polars::prelude::*, prelude::*};
        use criterion::Criterion;

        // Set ChiSquared significance level
        const ALPHA: f64 = 0.05;

        // PC-Stable `asia` benchmark
        pub fn asia(c: &mut Criterion) {
            // Load data set.
            let d = CsvReader::from_path("./tests/assets/pc_stable/asia.csv")
                .unwrap()
                .finish()
                .unwrap();
            let d = CategoricalDataMatrix::from(d);

            // Create ChiSquared conditional independence test
            let test = ChiSquared::new(&d, ALPHA);

            // Create PC-Stable functor
            let pcs = PCStable::new(&test);

            // Benchmark
            c.bench_function("discovery::pc_stable::categorical::call::asia", |b| {
                b.iter(|| {
                    let _: PGraph = pcs.call();
                })
            });
        }

        // PC-Stable `cancer` benchmark
        pub fn cancer(c: &mut Criterion) {
            // Load data set.
            let d = CsvReader::from_path("./tests/assets/pc_stable/cancer.csv")
                .unwrap()
                .finish()
                .unwrap();
            let d = CategoricalDataMatrix::from(d);

            // Create ChiSquared conditional independence test
            let test = ChiSquared::new(&d, ALPHA);

            // Create PC-Stable functor
            let pcs = PCStable::new(&test);

            // Benchmark
            c.bench_function("discovery::pc_stable::categorical::call::cancer", |b| {
                b.iter(|| {
                    let _: PGraph = pcs.call();
                })
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
            let test = ChiSquared::new(&d, ALPHA);

            // Create PC-Stable functor
            let pcs = PCStable::new(&test);

            // Benchmark
            c.bench_function("discovery::pc_stable::categorical::call::survey", |b| {
                b.iter(|| {
                    let _: PGraph = pcs.call();
                })
            });
        }
    }

    pub mod par_call {

        use causal_hub::{polars::prelude::*, prelude::*};
        use criterion::Criterion;

        // Set ChiSquared significance level
        const ALPHA: f64 = 0.05;

        // PC-Stable parallel `asia` benchmark
        pub fn asia(c: &mut Criterion) {
            // Load data set.
            let d = CsvReader::from_path("./tests/assets/pc_stable/asia.csv")
                .unwrap()
                .finish()
                .unwrap();
            let d = CategoricalDataMatrix::from(d);

            // Create ChiSquared conditional independence test
            let test = ChiSquared::new(&d, ALPHA);

            // Create PC-Stable functor
            let pcs = PCStable::new(&test);

            // Benchmark
            c.bench_function("discovery::pc_stable::categorical::par_call::asia", |b| {
                b.iter(|| {
                    let _: PGraph = pcs.par_call();
                })
            });
        }

        // PC-Stable parallel `cancer` benchmark
        pub fn cancer(c: &mut Criterion) {
            // Load data set.
            let d = CsvReader::from_path("./tests/assets/pc_stable/cancer.csv")
                .unwrap()
                .finish()
                .unwrap();
            let d = CategoricalDataMatrix::from(d);

            // Create ChiSquared conditional independence test
            let test = ChiSquared::new(&d, ALPHA);

            // Create PC-Stable functor
            let pcs = PCStable::new(&test);

            // Benchmark
            c.bench_function("discovery::pc_stable::categorical::par_call::cancer", |b| {
                b.iter(|| {
                    let _: PGraph = pcs.par_call();
                })
            });
        }

        // PC-Stable parallel `survey` benchmark
        pub fn survey(c: &mut Criterion) {
            // Load data set.
            let d = CsvReader::from_path("./tests/assets/pc_stable/survey.csv")
                .unwrap()
                .finish()
                .unwrap();
            let d = CategoricalDataMatrix::from(d);

            // Create ChiSquared conditional independence test
            let test = ChiSquared::new(&d, ALPHA);

            // Create PC-Stable functor
            let pcs = PCStable::new(&test);

            // Benchmark
            c.bench_function("discovery::pc_stable::categorical::par_call::survey", |b| {
                b.iter(|| {
                    let _: PGraph = pcs.par_call();
                })
            });
        }
    }
}

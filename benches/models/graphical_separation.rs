pub mod call {

    use causal_hub::prelude::*;
    use criterion::{BenchmarkId, Criterion, Throughput};
    use itertools::Itertools;

    fn driver(c: &mut Criterion, model: &str) {
        // Initialize benchmark group.
        let mut group =
            c.benchmark_group(format!("models::graphical_separation::call::{model}").as_str());

        // Load reference model.
        let model: CategoricalBN = BIF::read(format!("./tests/assets/bif/{model}.bif").as_str())
            .unwrap()
            .into();

        // Get underlying graph.
        let g = model.graph();
        // Initialize graphical separation.
        let d_separation = GSeparation::new(g);

        // Repeat for different sample sizes.
        for k in [0, 1, 2, 3, 5].iter() {
            // Set input dimension.
            group.throughput(Throughput::Elements(*k as u64));
            // Benchmark function.
            group.bench_with_input(BenchmarkId::from_parameter(k), k, |b, &k| {
                b.iter(|| {
                    for xy in V!(g).combinations(2) {
                        // Unpack variables.
                        let (x, y) = (xy[0], xy[1]);
                        // Get conditioning set.
                        let z = V!(g)
                            .filter(|&z| z != x && z != y)
                            .combinations(k)
                            .next()
                            .unwrap();
                        // Call d-separation.
                        let _ = ConditionalIndependenceTest::call(&d_separation, x, y, &z);
                    }
                })
            });
        }
    }

    pub fn asia(c: &mut Criterion) {
        driver(c, "asia");
    }

    pub fn alarm(c: &mut Criterion) {
        driver(c, "alarm");
    }
}

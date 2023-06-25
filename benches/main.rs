use criterion::{criterion_group, criterion_main};

include!("mod.rs");

criterion_group!(
    benches,
    // Data set benchmarks.
    crate::data::data_matrix::sample::asia,
    crate::data::data_matrix::sample::alarm,
    crate::data::data_matrix::sample_with_replacement::asia,
    crate::data::data_matrix::sample_with_replacement::alarm,
    // Causal Discovery benchmarks.
    crate::discovery::pc_stable::cancer,
    crate::discovery::pc_stable::par_cancer,
    crate::discovery::pc_stable::asia,
    crate::discovery::pc_stable::par_asia,
    crate::discovery::pc_stable::survey,
    crate::discovery::pc_stable::par_survey,
    crate::discovery::hill_climbing::call::asia,
    crate::discovery::hill_climbing::call::alarm,
    crate::discovery::hill_climbing::par_call::asia,
    crate::discovery::hill_climbing::par_call::alarm,
);

criterion_main!(benches);

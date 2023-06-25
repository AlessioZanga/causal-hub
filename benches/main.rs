use criterion::{criterion_group, criterion_main};

include!("mod.rs");

criterion_group!(
    benches,
    // Causal Discovery benchmarks.
    crate::discovery::hill_climbing::asia,
    crate::discovery::hill_climbing::alarm,
    crate::discovery::hill_climbing::par_asia,
    crate::discovery::hill_climbing::par_alarm,
    crate::discovery::pc_stable::cancer,
    crate::discovery::pc_stable::par_cancer,
    crate::discovery::pc_stable::asia,
    crate::discovery::pc_stable::par_asia,
    crate::discovery::pc_stable::survey,
    crate::discovery::pc_stable::par_survey
);

criterion_main!(benches);

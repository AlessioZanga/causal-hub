use criterion::{criterion_group, criterion_main};

mod data;
mod discovery;
mod stats;

criterion_group!(
    benches,
    // Data set benchmarks.
    data::data_matrix::sample::asia,
    data::data_matrix::sample::alarm,
    data::data_matrix::sample_with_replacement::asia,
    data::data_matrix::sample_with_replacement::alarm,
    // Causal Discovery benchmarks.
    discovery::pc_stable::cancer,
    discovery::pc_stable::par_cancer,
    discovery::pc_stable::asia,
    discovery::pc_stable::par_asia,
    discovery::pc_stable::survey,
    discovery::pc_stable::par_survey,
    discovery::hill_climbing::call::asia,
    discovery::hill_climbing::call::alarm,
    discovery::hill_climbing::par_call::asia,
    discovery::hill_climbing::par_call::alarm,
    // Statistics.
    stats::log_likelihood::marginal::asia,
    stats::log_likelihood::marginal::alarm,
    stats::log_likelihood::par_marginal::asia,
    stats::log_likelihood::par_marginal::alarm,
    stats::akaike_information_criterion::call::asia,
    stats::akaike_information_criterion::call::alarm,
    stats::akaike_information_criterion::par_call::asia,
    stats::akaike_information_criterion::par_call::alarm,
    stats::bayesian_information_criterion::call::asia,
    stats::bayesian_information_criterion::call::alarm,
    stats::bayesian_information_criterion::par_call::asia,
    stats::bayesian_information_criterion::par_call::alarm,
);

criterion_main!(benches);

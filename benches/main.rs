use criterion::{criterion_group, criterion_main};

mod data;
mod discovery;
mod models;
mod stats;

criterion_group!(
    benches,
    // Data set benchmarks.
    data::data_matrix::sample::asia,
    data::data_matrix::sample::alarm,
    data::data_matrix::sample_with_replacement::asia,
    data::data_matrix::sample_with_replacement::alarm,
    // Causal Discovery benchmarks.
    discovery::pc_stable::categorical::call::asia,
    discovery::pc_stable::categorical::call::cancer,
    discovery::pc_stable::categorical::call::survey,
    discovery::pc_stable::categorical::par_call::asia,
    discovery::pc_stable::categorical::par_call::cancer,
    discovery::pc_stable::categorical::par_call::survey,
    discovery::hill_climbing::categorical::call::asia,
    discovery::hill_climbing::categorical::call::alarm,
    discovery::hill_climbing::categorical::par_call::asia,
    discovery::hill_climbing::categorical::par_call::alarm,
    discovery::hill_climbing::zinb::call::dummy,
    // Models benchmarks.
    models::bayesian_network::sample::asia,
    models::bayesian_network::sample::alarm,
    models::bayesian_network::par_sample::asia,
    models::bayesian_network::par_sample::alarm,
    models::graphical_separation::call::asia,
    models::graphical_separation::call::alarm,
    // Statistics benchmarks.
    stats::log_likelihood::marginal::asia,
    stats::log_likelihood::marginal::alarm,
    stats::akaike_information_criterion::call::asia,
    stats::akaike_information_criterion::call::alarm,
    stats::bayesian_information_criterion::call::asia,
    stats::bayesian_information_criterion::call::alarm,
);

criterion_main!(benches);

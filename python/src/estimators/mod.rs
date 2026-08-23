mod parameters;
pub use parameters::*;

mod structures;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
pub use structures::*;

/// estimator methods.
///
#[gen_stub_pyclass_enum]
#[pyclass(
    name = "EstimatorMethod",
    module = "causal_hub.estimators",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PyEstimatorMethod {
    /// Maximum Likelihood Estimator.
    MLE,
    /// Bayesian Estimator.
    BE,
}

/// Scoring criteria for score-based structure learning algorithms.
///
#[gen_stub_pyclass_enum]
#[pyclass(
    name = "ScorerMethod",
    module = "causal_hub.estimators",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PyScorerMethod {
    /// Log Likelihood (`LL`).
    LL,
    /// Akaike Information Criterion (`AIC`).
    AIC,
    /// Akaike Information Criterion Corrected (`AICc`).
    AICC,
    /// Bayesian Information Criterion (`BIC`).
    BIC,
    /// Bayesian Information Criterion Corrected (`BICc`).
    BICC,
    /// Hannan-Quinn Criterion (`HQC`).
    HQC,
}

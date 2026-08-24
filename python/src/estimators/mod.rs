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
    name = "ParametersEstimator",
    module = "causal_hub.estimators",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PyParametersEstimator {
    /// Maximum Likelihood Estimator.
    MLE,
    /// Bayesian Estimator.
    BE,
}

/// Scoring criteria for score-based structure learning algorithms.
///
#[gen_stub_pyclass_enum]
#[pyclass(name = "Scorer", module = "causal_hub.estimators", from_py_object)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PyScorer {
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

/// Fitting methods for models: parameter fitting or structure learning.
///
#[gen_stub_pyclass_enum]
#[pyclass(name = "FitMethod", module = "causal_hub.estimators", from_py_object)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PyFitMethod {
    /// Fit the model parameters given the structure.
    Parameters,
    /// Learn the model structure from data, then fit the parameters.
    Structure,
}

/// Structure estimator methods for score- and constraint-based structure
/// learning algorithms.
///
#[gen_stub_pyclass_enum]
#[pyclass(
    name = "StructureEstimator",
    module = "causal_hub.estimators",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PyStructureEstimator {
    /// Continuous Time Hill Climbing (`CTHC`).
    CTHC,
    /// Continuous Time Peter-Clark (`CTPC`).
    CTPC,
}

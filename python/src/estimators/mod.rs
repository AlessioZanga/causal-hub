mod parameters;
pub use parameters::*;

mod structures;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;
pub use structures::*;

/// estimator methods.
#[gen_stub_pyclass_enum]
#[pyclass(
    name = "EstimatorMethod",
    module = "causal_hub.estimators",
    from_py_object
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PyEstimatorMethod {
    /// Maximum Likelihood Estimator.
    MLE,
    /// Bayesian Estimator.
    BE,
}

mod expectation_maximization;
use backend::estimators::{BNEstimator, CTBNEstimator, ParBNEstimator, ParCTBNEstimator};
pub use expectation_maximization::*;

// Define a trait for the estimator.
pub(crate) trait PyBNEstimator<T>: BNEstimator<T> + ParBNEstimator<T> + Send {}
impl<E, T> PyBNEstimator<T> for E where E: BNEstimator<T> + ParBNEstimator<T> + Send {}
pub(crate) trait PyCTBNEstimator<T>: CTBNEstimator<T> + ParCTBNEstimator<T> + Send {}
impl<E, T> PyCTBNEstimator<T> for E where E: CTBNEstimator<T> + ParCTBNEstimator<T> + Send {}

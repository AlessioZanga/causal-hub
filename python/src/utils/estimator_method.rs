/// A macro to dispatch over `PyEstimatorMethod`.
///
/// It constructs the selected parameter estimator on the given dataset and
/// evaluates the runner with it.
#[macro_export]
macro_rules! dispatch_estimator_method {
    ($method:expr, $dataset:expr, $runner:expr $(,)?) => {
        match $method {
            $crate::estimators::PyEstimatorMethod::MLE => {
                ($runner)(&::backend::estimators::MLE::new($dataset))
            }
            $crate::estimators::PyEstimatorMethod::BE => {
                ($runner)(&::backend::estimators::BE::new($dataset))
            }
        }
    };
}

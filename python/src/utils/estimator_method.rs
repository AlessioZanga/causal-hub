/// A macro to dispatch over `PyEstimatorMethod`.
///
/// It constructs the selected parameter estimator on the given dataset,
/// optionally configuring the missing data handling method and mechanism, and
/// evaluates the runner with it.
///
#[macro_export]
macro_rules! dispatch_estimator_method {
    (
        $dataset:expr,
        $estimator_method:expr,
        $missing_method:expr,
        $missing_mechanism:expr,
        $runner:expr $(,)?
    ) => {
        match $estimator_method {
            $crate::estimators::PyEstimatorMethod::MLE => {
                let estimator = ::backend::estimators::MLE::new($dataset)
                    .with_missing_method($missing_method, $missing_mechanism)
                    .map_err($crate::error::to_pyerr)?;
                ($runner)(&estimator)
            }
            $crate::estimators::PyEstimatorMethod::BE => {
                let estimator = ::backend::estimators::BE::new($dataset)
                    .with_missing_method($missing_method, $missing_mechanism)
                    .map_err($crate::error::to_pyerr)?;
                ($runner)(&estimator)
            }
        }
    };
}

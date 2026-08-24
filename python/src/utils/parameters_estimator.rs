/// A macro to dispatch over `PyParametersEstimator`.
///
/// It constructs the selected parameter estimator on the given dataset,
/// optionally configuring the missing data handling method and mechanism, and
/// evaluates the runner with it.
///
#[macro_export]
macro_rules! dispatch_parameters_estimator {
    (
        $dataset:expr,
        $parameters_estimator:expr,
        $missing_method:expr,
        $missing_mechanism:expr,
        $runner:expr $(,)?
    ) => {
        match $parameters_estimator {
            $crate::estimators::PyParametersEstimator::MLE => {
                let estimator = ::backend::estimators::MLE::new($dataset)
                    .with_missing_method($missing_method, $missing_mechanism)
                    .map_err($crate::error::to_pyerr)?;
                ($runner)(&estimator)
            }
            $crate::estimators::PyParametersEstimator::BE => {
                let estimator = ::backend::estimators::BE::new($dataset)
                    .with_missing_method($missing_method, $missing_mechanism)
                    .map_err($crate::error::to_pyerr)?;
                ($runner)(&estimator)
            }
        }
    };
}

/// A macro to dispatch over `PyScorerMethod`.
///
/// It constructs the corresponding scorer from the given cache and evaluates
/// the runner with it.
#[macro_export]
macro_rules! dispatch_scorer_method {
    ($scorer_method:expr, $cache:expr, $runner:expr $(,)?) => {
        match $scorer_method {
            $crate::estimators::PyScorerMethod::LL => {
                ($runner)(&::backend::estimators::LL::new($cache))
            }
            $crate::estimators::PyScorerMethod::AIC => {
                ($runner)(&::backend::estimators::AIC::new($cache))
            }
            $crate::estimators::PyScorerMethod::AICC => {
                ($runner)(&::backend::estimators::AICC::new($cache))
            }
            $crate::estimators::PyScorerMethod::BIC => {
                ($runner)(&::backend::estimators::BIC::new($cache))
            }
            $crate::estimators::PyScorerMethod::BICC => {
                ($runner)(&::backend::estimators::BICC::new($cache))
            }
            $crate::estimators::PyScorerMethod::HQC => {
                ($runner)(&::backend::estimators::HQC::new($cache))
            }
        }
    };
}

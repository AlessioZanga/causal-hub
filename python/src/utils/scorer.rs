/// A macro to dispatch over `PyScorer`.
///
/// It constructs the corresponding scorer from the given cache and evaluates
/// the runner with it.
///
#[macro_export]
macro_rules! dispatch_scorer {
    ($cache:expr, $scorer:expr, $runner:expr $(,)?) => {
        match $scorer {
            $crate::estimators::PyScorer::LL => ($runner)(&::backend::estimators::LL::new($cache)),
            $crate::estimators::PyScorer::AIC => {
                ($runner)(&::backend::estimators::AIC::new($cache))
            }
            $crate::estimators::PyScorer::AICC => {
                ($runner)(&::backend::estimators::AICC::new($cache))
            }
            $crate::estimators::PyScorer::BIC => {
                ($runner)(&::backend::estimators::BIC::new($cache))
            }
            $crate::estimators::PyScorer::BICC => {
                ($runner)(&::backend::estimators::BICC::new($cache))
            }
            $crate::estimators::PyScorer::HQC => {
                ($runner)(&::backend::estimators::HQC::new($cache))
            }
        }
    };
}

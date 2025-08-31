//! Estimators module for BNs and CTBNs.
//!
//! This module contains the following traits:
//!
//! * CSSEstimator - Estimation of conditional sufficient statistics.
//! * CPDEstimator - Estimation of CPDs.
//! * BNEstimator - Estimation of BNs.
//! * CTBNEstimator - Estimation of CTBNs.
//!
//! and their parallel versions:
//!
//! * ParCSSEstimator - Parallel estimation of conditional sufficient statistics.
//! * ParCPDEstimator - Parallel estimation of CPDs.
//! * ParBNEstimator - Parallel estimation of BNs.
//! * ParCTBNEstimator - Parallel estimation of CTBNs.
//!
//! This module contains the following estimators:
//!
//! * MLE - Maximum Likelihood Estimator.
//! * BE - Bayesian Estimator.
//! * EM - Expectation-Maximization algorithm.
//!
//! This is the table of the implemented traits for each estimator type:
//!
//! | Estimator         | MLE | BE | EM |
//! |-------------------|:---:|:--:|:--:|
//! | CSSEstimator      | ✅  | ✅ |    |
//! | CPDEstimator      | ✅  | ✅ |    |
//! | BNEstimator       | ✅  | ✅ |    |
//! | CTBNEstimator     | ✅  | ✅ |    |
//! | ParCSSEstimator   | ✅  | ✅ |    |
//! | ParCPDEstimator   | ✅  | ✅ |    |
//! | ParBNEstimator    | ✅  | ✅ |    |
//! | ParCTBNEstimator  | ✅  | ✅ |    |
//!

mod parameters;
pub use parameters::*;

mod structures;
pub use structures::*;

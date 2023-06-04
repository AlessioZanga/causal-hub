/// Bayesian Information Criterion (BIC) functor.
///
/// # Generics
///
/// - `RESCALED`: Rescale by -2, allowing for direct comparison with log-likelihood.
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct BayesianInformationCriterion<'a, D, const RESCALED: bool, const PARALLEL: bool> {
    pub(crate) d: &'a D,
    pub(crate) k: f64,
}

impl<'a, D, const RESCALED: bool, const PARALLEL: bool>
    BayesianInformationCriterion<'a, D, RESCALED, PARALLEL>
{
    /// Constructor for BIC functor.
    #[inline]
    pub const fn new(d: &'a D) -> Self {
        Self { d, k: 1. }
    }

    /// Sets penalty coefficient.
    #[inline]
    pub const fn with_penalty_coeff(mut self, k: f64) -> Self {
        // Set penalty coefficient.
        self.k = k;

        self
    }
}

/// Alias for (rescaled) single-thread BIC functor.
pub type BIC<'a, D> = BayesianInformationCriterion<'a, D, true, false>;
/// Alias for (rescaled) multi-thread BIC functor.
pub type ParallelBIC<'a, D> = BayesianInformationCriterion<'a, D, true, true>;

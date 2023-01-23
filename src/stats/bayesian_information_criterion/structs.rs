use std::marker::PhantomData;

/// Bayesian Information Criterion (BIC) functor.
///
/// # Generics
///
/// - `RESCALED`: Rescale by -2, allowing for direct comparison with log-likelihood.
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct BayesianInformationCriterion<D, const RESCALED: bool, const PARALLEL: bool> {
    _d: PhantomData<D>,
    pub(crate) k: f64,
}

impl<D, const RESCALED: bool, const PARALLEL: bool>
    BayesianInformationCriterion<D, RESCALED, PARALLEL>
{
    /// Constructor for BIC functor.
    #[inline]
    pub const fn new() -> Self {
        Self {
            _d: PhantomData,
            k: 1.,
        }
    }

    /// Sets penalty coefficient.
    #[inline]
    pub const fn with_penalty_coeff(mut self, k: f64) -> Self {
        // Set penalty coefficient.
        self.k = k;

        self
    }
}

impl<D, const RESCALED: bool, const PARALLEL: bool> Default
    for BayesianInformationCriterion<D, RESCALED, PARALLEL>
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Alias for (rescaled) single-thread BIC functor.
pub type BIC<D> = BayesianInformationCriterion<D, true, false>;
/// Alias for (rescaled) multi-thread BIC functor.
pub type ParallelBIC<D> = BayesianInformationCriterion<D, true, true>;

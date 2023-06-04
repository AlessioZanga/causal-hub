/// Akaike Information Criterion (AIC) functor.
///
/// # Generics
///
/// - `RESCALED`: Rescale by -2, allowing for direct comparison with log-likelihood.
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct AkaikeInformationCriterion<'a, D, const RESCALED: bool, const PARALLEL: bool> {
    pub(crate) d: &'a D,
    pub(crate) k: f64,
}

impl<'a, D, const RESCALED: bool, const PARALLEL: bool>
    AkaikeInformationCriterion<'a, D, RESCALED, PARALLEL>
{
    /// Constructor for AIC functor.
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

/// Alias for (rescaled) single-thread AIC functor.
pub type AIC<'a, D> = AkaikeInformationCriterion<'a, D, true, false>;
/// Alias for (rescaled) multi-thread AIC functor.
pub type ParallelAIC<'a, D> = AkaikeInformationCriterion<'a, D, true, true>;

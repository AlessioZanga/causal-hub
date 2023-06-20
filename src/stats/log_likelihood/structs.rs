/// Log-Likelihood (LL) functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct LogLikelihood<'a, D, const PARALLEL: bool> {
    pub(crate) data_set: &'a D,
}

impl<'a, D, const PARALLEL: bool> LogLikelihood<'a, D, PARALLEL> {
    /// Constructor for LL functor.
    #[inline]
    pub const fn new(data_set: &'a D) -> Self {
        Self { data_set }
    }
}

/// Alias for single-thread LL functor.
pub type LL<'a, D> = LogLikelihood<'a, D, false>;
/// Alias for multi-thread LL functor.
pub type ParallelLL<'a, D> = LogLikelihood<'a, D, true>;

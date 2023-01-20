use std::marker::PhantomData;

/// Marginal log-likelihood functor.
#[derive(Clone, Debug, Default)]
pub struct MarginalLogLikelihood<D> {
    _d: PhantomData<D>,
}

/// Conditional log-likelihood functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug, Default)]
pub struct ConditionalLogLikelihood<D, const PARALLEL: bool> {
    _d: PhantomData<D>,
}

/// Log-Likelihood (LL) functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug, Default)]
pub struct LogLikelihood<D, const PARALLEL: bool> {
    _d: PhantomData<D>,
}

impl<D, const PARALLEL: bool> LogLikelihood<D, PARALLEL> {
    /// Constructor for LL functor.
    #[inline]
    pub const fn new() -> Self {
        Self { _d: PhantomData }
    }
}

/// Alias for single-thread LL functor.
pub type LL<D> = LogLikelihood<D, false>;

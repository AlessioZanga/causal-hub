use std::marker::PhantomData;

/// Marginal log-likelihood functor.
#[derive(Clone, Debug)]
pub struct MarginalLogLikelihood<D, const PARALLEL: bool> {
    _d: PhantomData<D>,
}

impl<D, const PARALLEL: bool> Default for MarginalLogLikelihood<D, PARALLEL> {
    fn default() -> Self {
        Self {
            _d: Default::default(),
        }
    }
}

/// Conditional log-likelihood functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
pub struct ConditionalLogLikelihood<D, const PARALLEL: bool> {
    _d: PhantomData<D>,
}

impl<D, const PARALLEL: bool> Default for ConditionalLogLikelihood<D, PARALLEL> {
    fn default() -> Self {
        Self {
            _d: Default::default(),
        }
    }
}

/// Log-Likelihood (LL) functor.
///
/// # Generics
///
/// - `PARALLEL`: Enables parallel computation of conditional count matrix and log-likelihood.
///
#[derive(Clone, Debug)]
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

impl<D, const PARALLEL: bool> Default for LogLikelihood<D, PARALLEL> {
    fn default() -> Self {
        Self::new()
    }
}

/// Alias for single-thread LL functor.
pub type LL<D> = LogLikelihood<D, false>;
/// Alias for multi-thread LL functor.
pub type ParallelLL<D> = LogLikelihood<D, true>;

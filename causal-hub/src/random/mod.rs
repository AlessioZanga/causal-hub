mod datasets;
pub use datasets::*;

mod models;
pub use models::*;

/// A trait for random generators.
pub trait Random {
    /// The output type of the random generator.
    type Output;

    /// Returns a random instance of the output type.
    fn random(&mut self) -> Self::Output;
}

/// A trait for parallel random generators.
#[allow(clippy::module_name_repetitions)]
pub trait ParRandom {
    /// The output type of the parallel random generator.
    type Output;

    /// Returns a random instance of the output type in parallel.
    fn par_random(&mut self) -> Self::Output;
}

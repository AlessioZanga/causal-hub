/// A trait for random generators.
pub trait Random<T> {
    /// Returns a random instance of type `T`.
    fn random(&mut self) -> T;
}

mod datasets;
pub use datasets::*;

mod models;
pub use models::*;

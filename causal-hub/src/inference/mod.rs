mod approximate_inference;
pub use approximate_inference::*;

mod causal_inference;
pub use causal_inference::*;

mod backdoor_criterion;
pub use backdoor_criterion::*;

mod graphical_separation;
pub use graphical_separation::*;

mod topological_order;
pub use topological_order::*;

/// A trait to provide access to the underlying model.
pub trait Modelled<T> {
    /// Get the model.
    ///
    /// # Returns
    ///
    /// A reference to the model.
    ///
    fn model(&self) -> &T;
}

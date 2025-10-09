mod table;
mod trajectory;

use crate::{models::Labelled, types::Labels};

/// A struct representing a sufficient statistics estimator.
#[derive(Clone, Copy, Debug)]
pub struct SSE<'a, D> {
    dataset: &'a D,
}

impl<'a, D> SSE<'a, D> {
    /// Constructs a new sufficient statistics estimator.
    ///
    /// # Returns
    ///
    /// A new `SSE` instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self { dataset }
    }
}

impl<D> Labelled for SSE<'_, D>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

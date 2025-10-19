use crate::{models::Labelled, types::Labels};

/// Gaussian evidence structure.
#[derive(Clone, Debug)]
pub struct GaussEv {
    labels: Labels,
}

impl Labelled for GaussEv {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

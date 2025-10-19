use crate::{datasets::GaussType, models::Labelled, types::Labels};

/// Gaussian evidence type.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum GaussEvT {
    /// Certain positive evidence.
    CertainPositive {
        /// The observed event of the evidence.
        event: usize,
        /// The value of the evidence.
        value: GaussType,
    },
}

/// Gaussian evidence structure.
#[derive(Clone, Debug)]
pub struct GaussEv {
    labels: Labels,
    evidences: Vec<Option<GaussEvT>>,
}

impl Labelled for GaussEv {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl GaussEv {
    /// The evidences of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the evidence.
    ///
    #[inline]
    pub const fn evidences(&self) -> &Vec<Option<GaussEvT>> {
        &self.evidences
    }
}

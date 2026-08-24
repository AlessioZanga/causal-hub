//! Sufficient-statistics estimators.

mod table;
mod trajectory;

use crate::{
    datasets::{MissingMechanism, MissingMethod},
    models::HasLabels,
    types::{Error, Labels, Result},
};

/// A struct representing a sufficient statistics estimator.
#[derive(Clone, Debug)]
pub struct SSE<'a, D> {
    dataset: &'a D,
    missing_method: Option<MissingMethod>,
    missing_mechanism: Option<MissingMechanism>,
}

impl<'a, D> SSE<'a, D> {
    /// Constructs a new sufficient statistics estimator.
    ///
    /// # Returns
    ///
    /// A new sufficient statistics estimator instance.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self {
            dataset,
            missing_method: None,
            missing_mechanism: None,
        }
    }

    /// Sets the missing handling method.
    ///
    /// # Arguments
    ///
    /// * `missing_method` - An optional missing handling method to set.
    /// * `missing_mechanism` - An optional missing mechanism to set.
    ///
    /// # Returns
    ///
    /// A new estimator with the specified missing handling method.
    ///
    #[inline]
    pub fn with_missing_method(
        mut self,
        missing_method: Option<MissingMethod>,
        missing_mechanism: Option<MissingMechanism>,
    ) -> Result<Self> {
        // Validate missing method and mechanism.
        match (missing_method, &missing_mechanism) {
            (Some(MissingMethod::LW) | Some(MissingMethod::PW), Some(_)) => {
                return Err(Error::InvalidParameter(
                    "missing_mechanism",
                    "must be None if missing_method is LW or PW",
                ));
            }
            (Some(MissingMethod::IPW) | Some(MissingMethod::AIPW), None) => {
                return Err(Error::InvalidParameter(
                    "missing_mechanism",
                    "must be provided if missing_method is IPW or AIPW",
                ));
            }
            _ => {}
        }

        self.missing_method = missing_method;
        self.missing_mechanism = missing_mechanism;
        Ok(self)
    }
}

impl<D> HasLabels for SSE<'_, D>
where
    D: HasLabels,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

mod table;
mod trajectory;

use crate::{datasets::MissingMethod, models::Labelled, types::Labels};

/// A struct representing a Bayesian estimator.
#[derive(Clone, Debug)]
pub struct BE<'a, D, T> {
    dataset: &'a D,
    missing_method: Option<MissingMethod>,
    prior: T,
}

impl<'a, D> BE<'a, D, ()> {
    /// Creates a new Bayesian estimator.
    ///
    /// # Arguments
    ///
    /// * `dataset` - A reference to the dataset to fit the estimator to.
    ///
    /// # Returns
    ///
    /// A new Bayesian estimator.
    ///
    #[inline]
    pub const fn new(dataset: &'a D) -> Self {
        Self {
            dataset,
            missing_method: None,
            prior: (),
        }
    }
}

impl<'a, D, T> BE<'a, D, T> {
    /// Sets the missing data handling method.
    ///
    /// # Arguments
    ///
    /// * `missing_method` - The missing data handling method to set.
    ///
    /// # Returns
    ///
    /// A new Bayesian estimator with the specified missing data handling method.
    ///
    #[inline]
    pub fn with_missing_method(mut self, missing_method: MissingMethod) -> Self {
        self.missing_method = Some(missing_method);
        self
    }

    /// Sets the prior distribution.
    ///
    /// # Arguments
    ///
    /// * `prior` - The prior distribution to set.
    ///
    /// # Returns
    ///
    /// A new Bayesian estimator with the specified prior.
    ///
    #[inline]
    pub fn with_prior<U>(self, prior: U) -> BE<'a, D, U> {
        BE {
            dataset: self.dataset,
            missing_method: self.missing_method,
            prior,
        }
    }
}

impl<D, T> Labelled for BE<'_, D, T>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.labels()
    }
}

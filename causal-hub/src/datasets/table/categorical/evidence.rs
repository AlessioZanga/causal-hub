use approx::relative_eq;
use ndarray::prelude::*;

use crate::{
    datasets::CatTrjEvT,
    models::Labelled,
    types::{Labels, Set, States},
    utils::sort_states,
};

/// Categorical evidence type.
#[derive(Clone, Debug)]
pub enum CatEvT {
    /// Certain positive evidence.
    CertainPositive {
        /// The observed event of the evidence.
        event: usize,
        /// The state of the evidence.
        state: usize,
    },
    /// Certain negative evidence.
    CertainNegative {
        /// The observed event of the evidence.
        event: usize,
        /// The states of the evidence.
        not_states: Set<usize>,
    },
    /// Uncertain positive evidence.
    UncertainPositive {
        /// The observed event of the evidence.
        event: usize,
        /// The probabilities of the states.
        p_states: Array1<f64>,
    },
    /// Uncertain negative evidence.
    UncertainNegative {
        /// The observed event of the evidence.
        event: usize,
        /// The probabilities of the states.
        p_not_states: Array1<f64>,
    },
}

impl From<CatTrjEvT> for CatEvT {
    fn from(evidence: CatTrjEvT) -> Self {
        // Get shortened variable types.
        use CatEvT as U;
        use CatTrjEvT as T;
        // Match the evidence type discard the temporal information.
        match evidence {
            T::CertainPositiveInterval { event, state, .. } => U::CertainPositive { event, state },
            T::CertainNegativeInterval {
                event, not_states, ..
            } => U::CertainNegative { event, not_states },
            T::UncertainPositiveInterval {
                event, p_states, ..
            } => U::UncertainPositive { event, p_states },
            T::UncertainNegativeInterval {
                event,
                p_not_states,
                ..
            } => U::UncertainNegative {
                event,
                p_not_states,
            },
        }
    }
}

impl CatEvT {
    /// Return the observed event of the evidence.
    ///
    /// # Returns
    ///
    /// The observed event of the evidence.
    ///
    pub const fn event(&self) -> usize {
        match self {
            Self::CertainPositive { event, .. }
            | Self::CertainNegative { event, .. }
            | Self::UncertainPositive { event, .. }
            | Self::UncertainNegative { event, .. } => *event,
        }
    }
}

/// Categorical evidence structure.
#[derive(Clone, Debug)]
pub struct CatEv {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    evidences: Vec<Option<CatEvT>>,
}

impl Labelled for CatEv {
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CatEv {
    /// Creates a new categorical evidence structure.
    ///
    /// # Arguments
    ///
    /// * `states` - A collection of states, where each state is a tuple of a string and an iterator of strings.
    /// * `values` - A collection of values, where each value is a categorical evidence type.
    ///
    /// # Returns
    ///
    /// A new categorical evidence structure.
    ///
    pub fn new<I>(states: States, values: I) -> Self
    where
        I: IntoIterator<Item = CatEvT>,
    {
        // Get the indices to sort the labels and states labels.
        let (states, sorted_idx) = sort_states(states);
        // Get the sorted labels.
        let labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let shape = Array::from_iter(states.values().map(|x| x.len()));

        // Get shortened variable type.
        use CatEvT as E;

        // Allocate evidences.
        let mut evidences: Vec<Option<_>> = vec![None; states.len()];

        // Reverse the indices to get the argsort.
        let mut argsorted_idx = sorted_idx.clone();
        sorted_idx
            .into_iter()
            .enumerate()
            .for_each(|(i, (j, states))| {
                argsorted_idx[j] = (i, states);
            });

        // Iterate over the values and insert them into the events map using sorted indices.
        values.into_iter().for_each(|e| {
            // Get the event of the evidence.
            let event = e.event();
            // Sort event index.
            let (event, states) = &argsorted_idx[event];
            // Get the event index.
            let event = *event;

            // Sort the variable states.
            let e = match e {
                E::CertainPositive { state, .. } => {
                    // Sort the variable states.
                    let state = states[state];
                    // Construct the sorted evidence.
                    E::CertainPositive { event, state }
                }
                E::CertainNegative { not_states, .. } => {
                    // Sort the variable states.
                    let not_states = not_states.iter().map(|state| states[*state]).collect();
                    // Construct the sorted evidence.
                    E::CertainNegative { event, not_states }
                }
                E::UncertainPositive { p_states, .. } => {
                    // Allocate new variable states.
                    let mut new_p_states = Array::zeros(p_states.len());
                    // Sort the variable states.
                    p_states.indexed_iter().for_each(|(i, &p)| {
                        new_p_states[states[i]] = p;
                    });
                    // Substitute the sorted states.
                    let p_states = new_p_states;
                    // Construct the sorted evidence.
                    E::UncertainPositive { event, p_states }
                }
                E::UncertainNegative { p_not_states, .. } => {
                    // Allocate new variable states.
                    let mut new_p_not_states = Array::zeros(p_not_states.len());
                    // Sort the variable states.
                    p_not_states.indexed_iter().for_each(|(i, &p)| {
                        new_p_not_states[states[i]] = p;
                    });
                    // Substitute the sorted states.
                    let p_not_states = new_p_not_states;
                    // Construct the sorted evidence.
                    E::UncertainNegative {
                        event,
                        p_not_states,
                    }
                }
            };

            // Push the value into the variable events.
            evidences[event] = Some(e);
        });

        // For each variable ...
        for (i, evidence) in evidences.iter_mut().enumerate() {
            // Assert states distributions have the correct size.
            assert!(
                evidence.as_ref().is_none_or(|e| match e {
                    E::CertainPositive { .. } => true,
                    E::CertainNegative { .. } => true,
                    E::UncertainPositive { p_states, .. } => {
                        p_states.len() == shape[i]
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        p_not_states.len() == shape[i]
                    }
                }),
                "Evidence states distributions must have the correct size."
            );
            // Assert states distributions are not negative.
            assert!(
                evidence.as_ref().is_none_or(|e| match e {
                    E::CertainPositive { .. } => true,
                    E::CertainNegative { .. } => true,
                    E::UncertainPositive { p_states, .. } => {
                        p_states.iter().all(|&x| x >= 0.0)
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        p_not_states.iter().all(|&x| x >= 0.0)
                    }
                }),
                "Evidence states distributions must be non-negative."
            );
            // Assert states distributions sum to 1.
            assert!(
                evidence.as_ref().is_none_or(|e| match e {
                    E::CertainPositive { .. } => true,
                    E::CertainNegative { .. } => true,
                    E::UncertainPositive { p_states, .. } => {
                        relative_eq!(p_states.sum(), 1.)
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        relative_eq!(p_not_states.sum(), 1.)
                    }
                }),
                "Evidence states distributions must sum to 1."
            );
        }

        Self {
            labels,
            states,
            shape,
            evidences,
        }
    }

    /// The states of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the states of the evidence.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        &self.states
    }

    /// The shape of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the evidence.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        &self.shape
    }

    /// The evidences of the evidence.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the evidence.
    ///
    #[inline]
    pub const fn evidences(&self) -> &Vec<Option<CatEvT>> {
        &self.evidences
    }
}

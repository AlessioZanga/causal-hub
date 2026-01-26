use approx::relative_eq;
use ndarray::prelude::*;

use crate::{
    datasets::CatTrjEvT,
    models::Labelled,
    types::{Error, Labels, Result, Set, States},
};

/// Categorical evidence type.
#[non_exhaustive]
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

    /// Set the observed event of the evidence.
    ///
    /// # Arguments
    ///
    /// * `event` - The new observed event of the evidence.
    ///
    pub const fn set_event(&mut self, event: usize) {
        match self {
            Self::CertainPositive { event: e, .. }
            | Self::CertainNegative { event: e, .. }
            | Self::UncertainPositive { event: e, .. }
            | Self::UncertainNegative { event: e, .. } => *e = event,
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
    pub fn new<I>(mut states: States, values: I) -> Result<Self>
    where
        I: IntoIterator<Item = CatEvT>,
    {
        // Get shortened variable type.
        use CatEvT as E;

        // Get the sorted labels.
        let mut labels: Labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let mut shape = Array::from_iter(states.values().map(Set::len));
        // Fill the evidences.
        let mut evidences = values.into_iter().try_fold(
            vec![None; states.len()],
            |mut evidences, e| -> Result<_> {
                // Get the event of the evidence.
                let event = e.event();
                // Check if event is in bounds.
                if event >= evidences.len() {
                    return Err(Error::VertexOutOfBounds(event));
                }
                // Push the value into the variable events.
                evidences[event] = Some(e);

                Ok(evidences)
            },
        )?;

        // Sort states, if necessary.
        if !states.keys().is_sorted() || !states.values().all(|x| x.iter().is_sorted()) {
            // Clone the states.
            let mut new_states = states.clone();
            // Sort the states.
            new_states.sort_keys();
            new_states.values_mut().for_each(Set::sort);

            // Allocate new evidences.
            let mut new_evidences = vec![None; states.len()];

            // Iterate over the values and insert them into the events map using sorted indices.
            for e in evidences.into_iter().flatten() {
                // Get the event and states of the evidence.
                let (event, states) = states
                    .get_index(e.event())
                    .ok_or_else(|| Error::VertexOutOfBounds(e.event()))?;
                // Sort the event index.
                let (event, _, new_states) = new_states
                    .get_full(event)
                    .ok_or_else(|| Error::MissingLabel(event.clone()))?;

                // Sort the variable states.
                let e = match e {
                    E::CertainPositive { state, .. } => {
                        // Sort the variable states.
                        let state = new_states
                            .get_index_of(&states[state])
                            .ok_or_else(|| Error::MissingState(states[state].clone()))?;
                        // Construct the sorted evidence.
                        E::CertainPositive { event, state }
                    }
                    E::CertainNegative { not_states, .. } => {
                        // Sort the variable states.
                        let not_states = not_states
                            .iter()
                            .map(|&state| {
                                new_states
                                    .get_index_of(&states[state])
                                    .ok_or_else(|| Error::MissingState(states[state].clone()))
                            })
                            .collect::<Result<_>>()?;
                        // Construct the sorted evidence.
                        E::CertainNegative { event, not_states }
                    }
                    E::UncertainPositive { p_states, .. } => {
                        // Allocate new variable states.
                        let mut new_p_states = Array::zeros(p_states.len());
                        // Sort the variable states.
                        for (i, &p) in p_states.indexed_iter() {
                            // Get sorted index.
                            let state = new_states
                                .get_index_of(&states[i])
                                .ok_or_else(|| Error::MissingState(states[i].clone()))?;
                            // Assign probability to sorted index.
                            new_p_states[state] = p;
                        }
                        // Substitute the sorted states.
                        let p_states = new_p_states;
                        // Construct the sorted evidence.
                        E::UncertainPositive { event, p_states }
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        // Allocate new variable states.
                        let mut new_p_not_states = Array::zeros(p_not_states.len());
                        // Sort the variable states.
                        for (i, &p) in p_not_states.indexed_iter() {
                            // Get sorted index.
                            let state = new_states
                                .get_index_of(&states[i])
                                .ok_or_else(|| Error::MissingState(states[i].clone()))?;
                            // Assign probability to sorted index.
                            new_p_not_states[state] = p;
                        }
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
                new_evidences[event] = Some(e);
            }

            // Update the states.
            states = new_states;
            // Update the evidences.
            evidences = new_evidences;
            // Update the labels.
            labels = states.keys().cloned().collect();
            // Update the shape.
            shape = states.values().map(Set::len).collect();
        }

        // For each variable ...
        for (i, e) in evidences.iter_mut().enumerate() {
            if let Some(e) = e.as_ref() {
                match e {
                    E::CertainPositive { .. } => {}
                    E::CertainNegative { .. } => {}
                    E::UncertainPositive { p_states, .. } => {
                        if p_states.len() != shape[i] {
                            return Err(Error::IncompatibleShape(
                                p_states.len().to_string(),
                                shape[i].to_string(),
                            ));
                        }
                        if !p_states.iter().all(|&x| x >= 0.) {
                            return Err(Error::Probability(
                                "Evidence states distributions must be non-negative.".to_string(),
                            ));
                        }
                        if !relative_eq!(p_states.sum(), 1.) {
                            return Err(Error::Probability(
                                "Evidence states distributions must sum to 1.".to_string(),
                            ));
                        }
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        if p_not_states.len() != shape[i] {
                            return Err(Error::IncompatibleShape(
                                p_not_states.len().to_string(),
                                shape[i].to_string(),
                            ));
                        }
                        if !p_not_states.iter().all(|&x| x >= 0.) {
                            return Err(Error::Probability(
                                "Evidence states distributions must be non-negative.".to_string(),
                            ));
                        }
                        if !relative_eq!(p_not_states.sum(), 1.) {
                            return Err(Error::Probability(
                                "Evidence states distributions must sum to 1.".to_string(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(Self {
            labels,
            states,
            shape,
            evidences,
        })
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

    /// Restrict the evidence to the specified variables.
    ///
    /// # Arguments
    ///
    /// * `x` - Set of variables to select.
    ///
    /// # Errors
    ///
    /// * If the set of variables is empty.
    /// * If any variable in the set is out of bounds.
    ///
    /// # Returns
    ///
    /// The evidence restricted to the specified variables.
    ///
    pub fn select(&self, x: &Set<usize>) -> Result<Self>
    where
        Self: Sized,
    {
        // Check that the variables are in bounds.
        x.iter().try_for_each(|&i| {
            if i >= self.labels.len() {
                return Err(Error::VertexOutOfBounds(i));
            }
            Ok(())
        })?;

        // Sort the indices.
        let mut x = x.clone();
        x.sort();

        // Get the new states.
        let states: States = x
            .iter()
            .map(|&i| {
                self.states
                    .get_index(i)
                    .map(|(label, states)| (label.clone(), states.clone()))
                    .ok_or_else(|| Error::VertexOutOfBounds(i))
            })
            .collect::<Result<_>>()?;

        // Get the new values.
        let evidences = x.into_iter().enumerate().filter_map(|(i, x)| {
            // Set the event index to the new index.
            self.evidences[x].clone().map(|mut e| {
                e.set_event(i);
                e
            })
        });

        // Create the new evidence.
        Self::new(states, evidences)
    }
}

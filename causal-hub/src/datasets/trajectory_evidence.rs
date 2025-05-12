use approx::relative_eq;
use ndarray::prelude::*;

use super::CategoricalEv;
use crate::{
    datasets::Dataset,
    types::{FxIndexMap, FxIndexSet},
};

/// A type representing the evidence type for categorical trajectories.
#[derive(Clone, Debug)]
pub enum CategoricalTrajectoryEvidenceType {
    /// Certain positive interval evidence.
    CertainPositiveInterval {
        /// The observed state.
        state: usize,
        /// The start time of the observed interval.
        start_time: f64,
        /// The end time of the observed interval.
        end_time: f64,
    },
    /// Certain negative interval evidence.
    CertainNegativeInterval {
        /// The non-observed states.
        not_states: FxIndexSet<usize>,
        /// The start time of the non-observed interval.
        start_time: f64,
        /// The end time of the non-observed interval.
        end_time: f64,
    },
    /// Uncertain positive interval evidence.
    UncertainPositiveInterval {
        /// The distribution of the observed states.
        p_states: Array1<f64>,
        /// The start time of the observed interval.
        start_time: f64,
        /// The end time of the observed interval.
        end_time: f64,
    },
    /// Uncertain negative interval evidence.
    UncertainNegativeInterval {
        /// The distribution of the non-observed states.
        p_not_states: Array1<f64>,
        /// The start time of the non-observed interval.
        start_time: f64,
        /// The end time of the non-observed interval.
        end_time: f64,
    },
}

/// Type alias for `CategoricalTrajectoryEvidenceType`.
pub type CategoricalTrjEvT = CategoricalTrajectoryEvidenceType;

impl CategoricalTrjEvT {
    /// Returns the start time of the evidence.
    ///
    /// # Returns
    ///
    /// The start time of the evidence.
    ///
    pub const fn start_time(&self) -> f64 {
        match self {
            Self::CertainPositiveInterval { start_time, .. }
            | Self::CertainNegativeInterval { start_time, .. }
            | Self::UncertainPositiveInterval { start_time, .. }
            | Self::UncertainNegativeInterval { start_time, .. } => *start_time,
        }
    }

    /// Returns the end time of the evidence.
    ///
    /// # Returns
    ///
    /// The end time of the evidence.
    ///
    pub const fn end_time(&self) -> f64 {
        match self {
            Self::CertainPositiveInterval { end_time, .. }
            | Self::CertainNegativeInterval { end_time, .. }
            | Self::UncertainPositiveInterval { end_time, .. }
            | Self::UncertainNegativeInterval { end_time, .. } => *end_time,
        }
    }

    /// Checks if the evidence contains a given time.
    ///
    /// # Arguments
    ///
    /// * `time` - The time to check.
    ///
    /// # Returns
    ///
    /// `true` if the time is in [start_time, end_time), `false` otherwise.
    ///
    pub fn contains(&self, time: &f64) -> bool {
        (self.start_time()..self.end_time()).contains(time)
    }
}

/// A type representing a collection of evidences for a categorical trajectory.
pub struct CategoricalTrajectoryEvidence {
    labels: FxIndexSet<String>,
    states: FxIndexMap<String, FxIndexSet<String>>,
    cardinality: Array1<usize>,
    evidences: FxIndexMap<String, Vec<CategoricalTrjEvT>>,
}

/// Type alias for `CategoricalTrajectoryEvidence`.
pub type CategoricalTrjEv = CategoricalTrajectoryEvidence;

impl CategoricalTrjEv {
    /// Constructs a new `CategoricalTrajectoryEvidence` instance.
    ///
    /// # Arguments
    ///
    /// * `labels` - A set of labels for the variables.
    /// * `states` - A map of states for each variable.
    /// * `events` - A map of events for each variable.
    ///
    /// # Returns
    ///
    /// A new `CategoricalTrajectoryEvidence` instance.
    ///
    pub fn new<I, J, K, L, M, N>(states: I, values: M) -> Self
    where
        I: IntoIterator<Item = (K, J)>,
        J: IntoIterator<Item = L>,
        K: AsRef<str>,
        L: AsRef<str>,
        M: IntoIterator<Item = (N, CategoricalTrjEvT)>,
        N: AsRef<str>,
    {
        // Initialize variables counter.
        let mut n = 0;
        // Get the states of the variables.
        let mut states: FxIndexMap<_, _> = states
            .into_iter()
            .inspect(|_| n += 1)
            .map(|(label, states)| {
                // Convert the variable label to a string.
                let label = label.as_ref().to_owned();

                // Initialize states counter.
                let mut n = 0;
                // Convert the variable states to a set of strings.
                let states: FxIndexSet<_> = states
                    .into_iter()
                    .inspect(|_| n += 1)
                    .map(|x| x.as_ref().to_owned())
                    .collect();
                // Assert unique states.
                assert_eq!(states.len(), n, "Variables states must be unique.");

                (label, states)
            })
            .collect();

        // Assert unique labels.
        assert_eq!(states.len(), n, "Variables labels must be unique.");

        // Get the indices to sort the labels and states labels.
        let mut indices: Vec<(_, Vec<_>)> = states
            .values()
            .enumerate()
            .map(|(label_idx, states)| {
                // Allocate the indices of the states labels.
                let mut states_idx: Vec<_> = (0..states.len()).collect();
                // Sort the indices by the states labels.
                states_idx.sort_by_key(|&i| &states[i]);

                (label_idx, states_idx)
            })
            .collect();
        // Sort the indices by the states labels.
        indices.sort_by_key(|&(i, _)| states.get_index(i).unwrap().0);
        // Sort the states labels.
        states.values_mut().for_each(|states| states.sort());
        // Sort the labels.
        states.sort_keys();

        // Get the sorted labels.
        let labels = states.keys().cloned().collect();
        // Get the cardinality of the states.
        let cardinality = Array::from_iter(states.values().map(|x| x.len()));

        // Get shortened variable type.
        use CategoricalTrjEvT as E;

        // Allocate evidences.
        let mut evidences: FxIndexMap<_, Vec<_>> = states
            .keys()
            .map(|label| (label.clone(), Vec::new()))
            .collect();

        // Iterate over the values and insert them into the events map using sorted indices.
        values.into_iter().for_each(|(label, evidence)| {
            // Convert the label to a string.
            let label = label.as_ref().to_owned();
            // Get the variable index.
            let variable = states
                .get_index_of(&label)
                .expect("Variable label not found in states.");
            // Get the variable index, starting time, and ending time.
            let (start_time, end_time) = (evidence.start_time(), evidence.end_time());
            // Sort variable index.
            let (variable, states) = &indices[variable];

            // Sort the variable states.
            let e = match evidence {
                E::CertainPositiveInterval { state, .. } => {
                    // Sort the variable states.
                    let state = states[state];
                    // Construct the sorted evidence.
                    E::CertainPositiveInterval {
                        state,
                        start_time,
                        end_time,
                    }
                }
                E::CertainNegativeInterval { not_states, .. } => {
                    // Sort the variable states.
                    let not_states = not_states.iter().map(|state| states[*state]).collect();
                    // Construct the sorted evidence.
                    E::CertainNegativeInterval {
                        not_states,
                        start_time,
                        end_time,
                    }
                }
                E::UncertainPositiveInterval { p_states, .. } => {
                    // Allocate new variable states.
                    let mut new_p_states = Array::zeros(p_states.len());
                    // Sort the variable states.
                    p_states.indexed_iter().for_each(|(i, &p)| {
                        new_p_states[states[i]] = p;
                    });
                    // Substitute the sorted states.
                    let p_states = new_p_states;
                    // Construct the sorted evidence.
                    E::UncertainPositiveInterval {
                        p_states,
                        start_time,
                        end_time,
                    }
                }
                E::UncertainNegativeInterval { p_not_states, .. } => {
                    // Allocate new variable states.
                    let mut new_p_not_states = Array::zeros(p_not_states.len());
                    // Sort the variable states.
                    p_not_states.indexed_iter().for_each(|(i, &p)| {
                        new_p_not_states[states[i]] = p;
                    });
                    // Substitute the sorted states.
                    let p_not_states = new_p_not_states;
                    // Construct the sorted evidence.
                    E::UncertainNegativeInterval {
                        p_not_states,
                        start_time,
                        end_time,
                    }
                }
            };

            // Push the value into the variable events.
            evidences[*variable].push(e);
        });

        // For each variable ...
        for (i, evidence) in evidences.values_mut().enumerate() {
            // Assert starting times must be positive and finite.
            assert!(
                evidence
                    .iter()
                    .all(|e| e.start_time().is_finite() && e.start_time() >= 0.0),
                "Starting time must be positive and finite."
            );
            // Assert ending times must be positive and finite.
            assert!(
                evidence
                    .iter()
                    .all(|e| e.end_time().is_finite() && e.end_time() >= 0.0),
                "Ending time must be positive and finite."
            );

            // Sort the events by starting time.
            evidence.sort_by(|a, b| {
                a.start_time()
                    .partial_cmp(&b.start_time())
                    // Due to previous assertions, this should never fail.
                    .unwrap_or_else(|| unreachable!())
            });

            // Assert starting time is less than ending time.
            assert!(
                evidence.iter().all(|e| e.start_time() < e.end_time()),
                "Starting time must be less than ending time."
            );
            // Assert current ending time is less than next starting time.
            assert!(
                evidence
                    .windows(2)
                    .all(|e| e[0].end_time() < e[1].start_time()),
                "Ending time must be less than starting time."
            );
            // Assert states distributions have the correct size.
            assert!(
                evidence.iter().all(|e| match e {
                    E::CertainPositiveInterval { .. } => true,
                    E::CertainNegativeInterval { .. } => true,
                    E::UncertainPositiveInterval { p_states, .. } => {
                        p_states.len() == cardinality[i]
                    }
                    E::UncertainNegativeInterval { p_not_states, .. } => {
                        p_not_states.len() == cardinality[i]
                    }
                }),
                "States distributions must have the correct size."
            );
            // Assert states distributions are not negative.
            assert!(
                evidence.iter().all(|e| match e {
                    E::CertainPositiveInterval { .. } => true,
                    E::CertainNegativeInterval { .. } => true,
                    E::UncertainPositiveInterval { p_states, .. } => {
                        p_states.iter().all(|&x| x >= 0.0)
                    }
                    E::UncertainNegativeInterval { p_not_states, .. } => {
                        p_not_states.iter().all(|&x| x >= 0.0)
                    }
                }),
                "States distributions must be non-negative."
            );
            // Assert states distributions sum to 1.
            assert!(
                evidence.iter().all(|e| match e {
                    E::CertainPositiveInterval { .. } => true,
                    E::CertainNegativeInterval { .. } => true,
                    E::UncertainPositiveInterval { p_states, .. } => {
                        relative_eq!(p_states.sum(), 1.)
                    }
                    E::UncertainNegativeInterval { p_not_states, .. } => {
                        relative_eq!(p_not_states.sum(), 1.)
                    }
                }),
                "States distributions must sum to 1."
            );
        }

        // Create a new categorical trajectory evidence instance.
        Self {
            labels,
            states,
            cardinality,
            evidences,
        }
    }

    /// Returns the states of the trajectory evidence.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectory evidence.
    ///
    #[inline]
    pub const fn states(&self) -> &FxIndexMap<String, FxIndexSet<String>> {
        &self.states
    }

    /// Returns the cardinality of the trajectory evidence.
    ///
    /// # Returns
    ///
    /// A reference to the cardinality of the trajectory evidence.
    ///
    #[inline]
    pub const fn cardinality(&self) -> &Array1<usize> {
        &self.cardinality
    }

    /// Returns the evidences of the trajectory evidence.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the trajectory evidence.
    ///
    #[inline]
    pub const fn evidences(&self) -> &FxIndexMap<String, Vec<CategoricalTrjEvT>> {
        &self.evidences
    }

    /// Returns the evidences at time zero.
    ///
    /// # Returns
    ///
    /// The evidences at time zero.
    ///
    pub fn initial_evidence(&self) -> CategoricalEv {
        // Get the evidences at time zero.
        let evidences = self.evidences.iter().filter_map(|(label, evidence)| {
            // Get the first evidence, if any.
            let evidence = evidence.iter().next().cloned();
            // Check if the evidence is at time zero.
            let evidence = evidence.filter(|e| relative_eq!(e.start_time(), 0.));
            // Map the evidence to its variable.
            evidence.map(|e| (label, e.into()))
        });

        // Clone the states.
        let states = self.states.clone();

        // Create a new categorical evidence instance.
        CategoricalEv::new(states, evidences)
    }
}

impl Dataset for CategoricalTrjEv {
    type Labels = FxIndexSet<String>;
    type Values = FxIndexMap<String, Vec<CategoricalTrjEvT>>;

    #[inline]
    fn labels(&self) -> &Self::Labels {
        &self.labels
    }

    #[inline]
    fn values(&self) -> &Self::Values {
        &self.evidences
    }

    #[inline]
    fn sample_size(&self) -> usize {
        self.evidences.values().map(|v| v.len()).sum()
    }
}

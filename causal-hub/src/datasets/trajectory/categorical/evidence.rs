use approx::relative_eq;
use ndarray::prelude::*;
use rayon::prelude::*;

use crate::{
    datasets::CatEv,
    models::Labelled,
    types::{EPSILON, Labels, Set, States},
};

/// A type representing the evidence type for categorical trajectories.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum CatTrjEvT {
    /// Certain positive interval evidence.
    CertainPositiveInterval {
        /// The observed event.
        event: usize,
        /// The observed state.
        state: usize,
        /// The start time of the observed interval.
        start_time: f64,
        /// The end time of the observed interval.
        end_time: f64,
    },
    /// Certain negative interval evidence.
    CertainNegativeInterval {
        /// The observed event.
        event: usize,
        /// The non-observed states.
        not_states: Set<usize>,
        /// The start time of the non-observed interval.
        start_time: f64,
        /// The end time of the non-observed interval.
        end_time: f64,
    },
    /// Uncertain positive interval evidence.
    UncertainPositiveInterval {
        /// The observed event.
        event: usize,
        /// The distribution of the observed states.
        p_states: Array1<f64>,
        /// The start time of the observed interval.
        start_time: f64,
        /// The end time of the observed interval.
        end_time: f64,
    },
    /// Uncertain negative interval evidence.
    UncertainNegativeInterval {
        /// The observed event.
        event: usize,
        /// The distribution of the non-observed states.
        p_not_states: Array1<f64>,
        /// The start time of the non-observed interval.
        start_time: f64,
        /// The end time of the non-observed interval.
        end_time: f64,
    },
}

impl CatTrjEvT {
    /// Return the observed event of the evidence.
    ///
    /// # Returns
    ///
    /// The observed event of the evidence.
    ///
    pub const fn event(&self) -> usize {
        match self {
            Self::CertainPositiveInterval { event, .. }
            | Self::CertainNegativeInterval { event, .. }
            | Self::UncertainPositiveInterval { event, .. }
            | Self::UncertainNegativeInterval { event, .. } => *event,
        }
    }

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
#[derive(Clone, Debug)]
pub struct CatTrjEv {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    evidences: Vec<Vec<CatTrjEvT>>,
}

impl Labelled for CatTrjEv {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CatTrjEv {
    /// Constructs a new `CatTrjEv` instance.
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
    pub fn new<I>(mut states: States, values: I) -> Self
    where
        I: IntoIterator<Item = CatTrjEvT>,
    {
        // Get shortened variable type.
        use CatTrjEvT as E;

        // Get the sorted labels.
        let mut labels = states.keys().cloned().collect();
        // Get the shape of the states.
        let mut shape = Array::from_iter(states.values().map(Set::len));
        // Allocate evidences.
        let mut evidences = vec![vec![]; states.len()];

        // Fill the evidences.
        values.into_iter().for_each(|e| {
            // Get the event index.
            let event = e.event();
            // Push the value into the events.
            evidences[event].push(e);
        });

        // Sort states, if necessary.
        if !states.keys().is_sorted() || !states.values().all(|x| x.iter().is_sorted()) {
            // Clone the states.
            let mut new_states = states.clone();
            // Sort the states.
            new_states.sort_keys();
            new_states.values_mut().for_each(Set::sort);

            // Allocate new evidences.
            let mut new_evidences = vec![vec![]; states.len()];

            // Iterate over the values and insert them into the events map using sorted indices.
            evidences.into_iter().flatten().for_each(|e| {
                // Get the event index, starting time, and ending time.
                let (start_time, end_time) = (e.start_time(), e.end_time());
                // Get the event and states of the evidence.
                let (event, states) = states
                    .get_index(e.event())
                    .expect("Failed to get label of evidence.");
                // Sort the event index.
                let (event, _, new_states) = new_states
                    .get_full(event)
                    .expect("Failed to get full state.");

                // Sort the event states.
                let e = match e {
                    E::CertainPositiveInterval { state, .. } => {
                        // Sort the variable states.
                        let state = new_states
                            .get_index_of(&states[state])
                            .expect("Failed to get index of state.");
                        // Construct the sorted evidence.
                        E::CertainPositiveInterval {
                            event,
                            state,
                            start_time,
                            end_time,
                        }
                    }
                    E::CertainNegativeInterval { not_states, .. } => {
                        // Sort the event states.
                        let not_states = not_states
                            .iter()
                            .map(|&state| {
                                new_states
                                    .get_index_of(&states[state])
                                    .expect("Failed to get index of state.")
                            })
                            .collect();
                        // Construct the sorted evidence.
                        E::CertainNegativeInterval {
                            event,
                            not_states,
                            start_time,
                            end_time,
                        }
                    }
                    E::UncertainPositiveInterval { p_states, .. } => {
                        // Allocate new event states.
                        let mut new_p_states = Array::zeros(p_states.len());
                        // Sort the event states.
                        p_states.indexed_iter().for_each(|(i, &p)| {
                            // Get sorted index.
                            let state = new_states
                                .get_index_of(&states[i])
                                .expect("Failed to get index of state.");
                            // Assign probability to sorted index.
                            new_p_states[state] = p;
                        });
                        // Substitute the sorted states.
                        let p_states = new_p_states;
                        // Construct the sorted evidence.
                        E::UncertainPositiveInterval {
                            event,
                            p_states,
                            start_time,
                            end_time,
                        }
                    }
                    E::UncertainNegativeInterval { p_not_states, .. } => {
                        // Allocate new event states.
                        let mut new_p_not_states = Array::zeros(p_not_states.len());
                        // Sort the event states.
                        p_not_states.indexed_iter().for_each(|(i, &p)| {
                            // Get sorted index.
                            let state = new_states
                                .get_index_of(&states[i])
                                .expect("Failed to get index of state.");
                            // Assign probability to sorted index.
                            new_p_not_states[state] = p;
                        });
                        // Substitute the sorted states.
                        let p_not_states = new_p_not_states;
                        // Construct the sorted evidence.
                        E::UncertainNegativeInterval {
                            event,
                            p_not_states,
                            start_time,
                            end_time,
                        }
                    }
                };

                // Push the value into the events.
                new_evidences[event].push(e);
            });

            // Update the states.
            states = new_states;
            // Update the evidences.
            evidences = new_evidences;
            // Update the labels.
            labels = states.keys().cloned().collect();
            // Update the shape.
            shape = states.values().map(Set::len).collect();
        }

        // Check and fix incoherent evidences.
        evidences.iter_mut().zip(&shape).for_each(
            |(e, shape): (&mut Vec<E>, &usize)| {
                // Assert state, starting and ending times are coherent.
                e.iter().for_each(|e| {
                    // Assert starting time must be positive and finite.
                    assert!(
                        e.start_time().is_finite() && e.start_time() >= 0.0,
                        "Starting time must be positive and finite."
                    );
                    // Assert ending time must be positive and finite.
                    assert!(
                        e.end_time().is_finite() && e.end_time() >= 0.0,
                        "Ending time must be positive and finite."
                    );
                    // Assert starting time is less or equal than ending time.
                    assert!(
                        e.start_time() <= e.end_time(),
                        "Starting time must be less or equal than ending time."
                    );
                    // Assert states distributions have the correct size.
                    assert!(
                        match e {
                            E::CertainPositiveInterval { .. } => true,
                            E::CertainNegativeInterval { .. } => true,
                            E::UncertainPositiveInterval { p_states, .. } => {
                                p_states.len() == *shape
                            }
                            E::UncertainNegativeInterval { p_not_states, .. } => {
                                p_not_states.len() == *shape
                            }
                        },
                        "States distributions must have the correct size."
                    );
                    // Assert states distributions are not negative.
                    assert!(
                        match e {
                            E::CertainPositiveInterval { .. } => true,
                            E::CertainNegativeInterval { .. } => true,
                            E::UncertainPositiveInterval { p_states, .. } => {
                                p_states.iter().all(|&x| x >= 0.)
                            }
                            E::UncertainNegativeInterval { p_not_states, .. } => {
                                p_not_states.iter().all(|&x| x >= 0.)
                            }
                        },
                        "States distributions must be non-negative."
                    );
                    // Assert states distributions sum to 1.
                    assert!(
                        match e {
                            E::CertainPositiveInterval { .. } => true,
                            E::CertainNegativeInterval { .. } => true,
                            E::UncertainPositiveInterval { p_states, .. } => {
                                relative_eq!(p_states.sum(), 1., epsilon = EPSILON)
                            }
                            E::UncertainNegativeInterval { p_not_states, .. } => {
                                relative_eq!(p_not_states.sum(), 1., epsilon = EPSILON)
                            }
                        },
                        "States distributions must sum to one."
                    );
                });

                // Sort the events by starting time.
                e.sort_by(|a, b| {
                    a.start_time()
                        .partial_cmp(&b.start_time())
                        // Due to previous assertions, this should never fail.
                        .unwrap_or_else(|| unreachable!())
                });

                // Handle overlapping intervals.
                *e = e.iter().fold(Vec::new(), |mut e: Vec<E>, e_j: &E| {
                    // Ii evence is empty ...
                    if e.is_empty() {
                        // ... push current evidence and exit.
                        e.push(e_j.clone());
                        return e;
                    }

                    // Get the last evidence.
                    let e_i: &E = e.last().unwrap();
                    // Assert intervals times are coherent.
                    assert!(
                        e_i.start_time() <= e_j.start_time(),
                        "Two evidences for the same variable must have non-increasing starting time: \n\
                        \t expected: e(i).start_time <= e(i+1).start_time, \n\
                        \t found:    e(i).start_time >  e(i+1).start_time, \n\
                        \t for:      e(i).start_time == {} , \n\
                        \t and:    e(i+1).start_time == {} .",
                        e_i.start_time(),
                        e_j.start_time()
                    );
                    // If the current evidence ends before the next one starts ...
                    if e_i.end_time() <= e_j.start_time() {
                        // ... push current evidence and exit.
                        e.push(e_j.clone());
                        return e;
                    }
                    // Otherwise, we have overlapping intervals,
                    // check if they are the same type of evidence.
                    match (e_i, e_j) {
                        // If they are the same type of evidence ...
                        (
                            E::CertainPositiveInterval { state: s_i, .. },
                            E::CertainPositiveInterval { state: s_j, .. },
                        ) => {
                            // Check if they are the same state.
                            if s_i == s_j {
                                // Get evidence event, state, start time and end time.
                                let (event, state, start_time) = (e_i.event(), *s_i, e_i.start_time());
                                // Set end time to the maximum of both.
                                let end_time = e_i.end_time().max(e_j.end_time());
                                // Set the last evidence end time to the maximum of both.
                                *e.last_mut().unwrap() = E::CertainPositiveInterval {
                                    event,
                                    state,
                                    start_time,
                                    end_time,
                                };
                            // Otherwise, merge the two certain evidences into an uncertain one.
                            } else {
                                // Construct uncertain positive evidence.
                                let mut p_states = Array::zeros(*shape);
                                // Set the state of the evidence with a weight proportion to the time.
                                p_states[*s_i] = e_i.end_time() - e_i.start_time();
                                p_states[*s_j] = e_j.end_time() - e_j.start_time();
                                // Normalize the states.
                                p_states /= p_states.sum();
                                // Get evidence event, states, start time and end time.
                                let event = e_i.event();
                                let start_time = e_i.start_time().min(e_j.start_time());
                                let end_time = e_i.end_time().max(e_j.end_time());
                                // Set the last evidence end time to the maximum of both.
                                *e.last_mut().unwrap() = E::UncertainPositiveInterval {
                                    event,
                                    p_states,
                                    start_time,
                                    end_time,
                                };
                            }
                        }
                        (
                            E::CertainNegativeInterval {
                                not_states: s_i, ..
                            },
                            E::CertainNegativeInterval {
                                not_states: s_j, ..
                            },
                        ) => {
                            // Check if they are the same states.
                            assert_eq!(
                                s_i, s_j,
                                "Overlapping negative evidence must have the same states."
                            );
                            // Get evidence event, not states, start time and end time.
                            let (event, not_states, start_time) =
                                (e_i.event(), s_i.clone(), e_i.start_time());
                            // Set end time to the maximum of both.
                            let end_time = e_i.end_time().max(e_j.end_time());
                            // Set the last evidence end time to the maximum of both.
                            *e.last_mut().unwrap() = E::CertainNegativeInterval {
                                event,
                                not_states,
                                start_time,
                                end_time,
                            };
                        }
                        (
                            E::UncertainPositiveInterval { p_states: s_i, .. },
                            E::UncertainPositiveInterval { p_states: s_j, .. },
                        ) => {
                            // Check if they are the same states.
                            assert!(
                                relative_eq!(s_i, s_j),
                                "Overlapping uncertain evidence must have the same states."
                            );
                            // Get evidence event, states, start time and end time.
                            let (event, p_states, start_time) =
                                (e_i.event(), s_i.clone(), e_i.start_time());
                            // Set end time to the maximum of both.
                            let end_time = e_i.end_time().max(e_j.end_time());
                            // Set the last evidence end time to the maximum of both.
                            *e.last_mut().unwrap() = E::UncertainPositiveInterval {
                                event,
                                p_states,
                                start_time,
                                end_time,
                            };
                        }
                        (
                            E::UncertainNegativeInterval {
                                p_not_states: s_i, ..
                            },
                            E::UncertainNegativeInterval {
                                p_not_states: s_j, ..
                            },
                        ) => {
                            // Check if they are the same states.
                            assert!(
                                relative_eq!(s_i, s_j),
                                "Overlapping uncertain evidence must have the same states."
                            );
                            // Get evidence event, not states, start time and end time.
                            let (event, p_not_states, start_time) =
                                (e_i.event(), s_i.clone(), e_i.start_time());
                            // Set end time to the maximum of both.
                            let end_time = e_i.end_time().max(e_j.end_time());
                            // Set the last evidence end time to the maximum of both.
                            *e.last_mut().unwrap() = E::UncertainNegativeInterval {
                                event,
                                p_not_states,
                                start_time,
                                end_time,
                            };
                        }
                        // If they are not the same type of evidence ...
                        _ => panic!("Overlapping evidence must have the same type"),
                    }

                    e
                });

                // Assert current ending time is less or equal than next starting time.
                assert!(
                    e
                        .windows(2)
                        .all(|e| e[0].end_time() <= e[1].start_time()),
                    "Ending time must be less or equal than next starting time."
                );
            },
        );

        // Create a new categorical trajectory evidence instance.
        Self {
            labels,
            states,
            shape,
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
    pub const fn states(&self) -> &States {
        &self.states
    }

    /// Returns the shape of the trajectory evidence.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the trajectory evidence.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        &self.shape
    }

    /// Returns the evidences of the trajectory.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the trajectory.
    ///
    #[inline]
    pub fn evidences(&self) -> &Vec<Vec<CatTrjEvT>> {
        &self.evidences
    }

    /// Returns the evidences at time zero.
    ///
    /// # Returns
    ///
    /// The evidences at time zero.
    ///
    pub fn initial_evidence(&self) -> CatEv {
        // Get the evidences at time zero.
        let evidences = self.evidences.iter().filter_map(|e| {
            // Get the first evidence, if any.
            let e = e.iter().next().cloned();
            // Check if the evidence is at time zero.
            let e = e.filter(|e| relative_eq!(e.start_time(), 0.));
            // Map the evidence to its variable.
            e.map(|e| e.into())
        });

        // Clone the states.
        let states = self.states.clone();

        // Create a new categorical evidence instance.
        CatEv::new(states, evidences)
    }
}

/// A collection of multivariate trajectories evidence.
#[derive(Clone, Debug)]
pub struct CatTrjsEv {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    evidences: Vec<CatTrjEv>,
}

impl Labelled for CatTrjsEv {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl CatTrjsEv {
    /// Constructs a new collection of trajectories evidence.
    ///
    /// # Arguments
    ///
    /// * `trajectories` - An iterator of `CatTrjEv` instances.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * The trajectories have different labels.
    /// * The trajectories have different states.
    /// * The trajectories have different shape.
    ///
    /// # Returns
    ///
    /// A new instance of `CategoricalTrajectoriesEvidence`.
    ///
    pub fn new<I>(evidences: I) -> Self
    where
        I: IntoIterator<Item = CatTrjEv>,
    {
        // Collect the trajectories into a vector.
        let evidences: Vec<_> = evidences.into_iter().collect();

        // Assert every trajectory has the same labels.
        assert!(
            evidences
                .windows(2)
                .all(|trjs| trjs[0].labels().eq(trjs[1].labels())),
            "All trajectories must have the same labels."
        );
        // Assert every trajectory has the same states.
        assert!(
            evidences
                .windows(2)
                .all(|trjs| trjs[0].states().eq(trjs[1].states())),
            "All trajectories must have the same states."
        );
        // Assert every trajectory has the same shape.
        assert!(
            evidences
                .windows(2)
                .all(|trjs| trjs[0].shape().eq(trjs[1].shape())),
            "All trajectories must have the same shape."
        );

        // Get the labels, states and shape from the first trajectory.
        let (labels, states, shape) = match evidences.first() {
            None => (Labels::default(), States::default(), Array1::default((0,))),
            Some(x) => (x.labels().clone(), x.states().clone(), x.shape().clone()),
        };

        Self {
            labels,
            states,
            shape,
            evidences,
        }
    }

    /// Returns the states of the trajectories evidence.
    ///
    /// # Returns
    ///
    /// A reference to the states of the trajectories evidence.
    ///
    #[inline]
    pub fn states(&self) -> &States {
        &self.states
    }

    /// Returns the shape of the trajectories evidence.
    ///
    /// # Returns
    ///
    /// A reference to the shape of the trajectories evidence.
    ///
    #[inline]
    pub fn shape(&self) -> &Array1<usize> {
        &self.shape
    }

    /// Returns the evidences of the trajectories.
    ///
    /// # Returns
    ///
    /// A reference to the evidences of the trajectories.
    ///
    #[inline]
    pub fn evidences(&self) -> &Vec<CatTrjEv> {
        &self.evidences
    }
}

impl FromIterator<CatTrjEv> for CatTrjsEv {
    #[inline]
    fn from_iter<I: IntoIterator<Item = CatTrjEv>>(iter: I) -> Self {
        Self::new(iter)
    }
}

impl FromParallelIterator<CatTrjEv> for CatTrjsEv {
    #[inline]
    fn from_par_iter<I: IntoParallelIterator<Item = CatTrjEv>>(iter: I) -> Self {
        Self::new(iter.into_par_iter().collect::<Vec<_>>())
    }
}

impl<'a> IntoIterator for &'a CatTrjsEv {
    type IntoIter = std::slice::Iter<'a, CatTrjEv>;
    type Item = &'a CatTrjEv;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.evidences.iter()
    }
}

impl<'a> IntoParallelRefIterator<'a> for CatTrjsEv {
    type Item = &'a CatTrjEv;
    type Iter = rayon::slice::Iter<'a, CatTrjEv>;

    #[inline]
    fn par_iter(&'a self) -> Self::Iter {
        self.evidences.par_iter()
    }
}

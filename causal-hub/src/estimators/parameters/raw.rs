use std::ops::Deref;

use itertools::Itertools;
use ndarray::{Zip, prelude::*};
use rand::{Rng, SeedableRng, seq::SliceRandom};
use rand_distr::{Distribution, weighted::WeightedIndex};
use rayon::prelude::*;

use crate::{
    datasets::{CatTrj, CatTrjEv, CatTrjEvT, CatTrjs, CatTrjsEv, CatType},
    estimators::{BE, CIMEstimator, ParCIMEstimator},
    models::{CatCIM, Labelled},
    types::{Labels, Set},
};

// TODO: This must be refactored to be stateless.

/// A struct representing a raw estimator.
///
/// This estimator is used to find an initial guess of the parameters with the given evidence.
/// Its purpose is to provide a starting point for the other estimators, like EM.
///
#[derive(Debug)]
pub struct RAWE<'a, R, E, D> {
    rng: &'a mut R,
    evidence: &'a E,
    dataset: Option<D>,
}

impl<R, E, D> Deref for RAWE<'_, R, E, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        self.dataset.as_ref().unwrap()
    }
}

impl<R, E, D> Labelled for RAWE<'_, R, E, D>
where
    D: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.dataset.as_ref().unwrap().labels()
    }
}

impl<'a, R: Rng + SeedableRng> RAWE<'a, R, CatTrjEv, CatTrj> {
    /// Constructs a new raw estimator from the evidence.
    ///
    /// # Arguments
    ///
    /// * `evidence` - A reference to the evidence to fill.
    ///
    /// # Returns
    ///
    /// A new `RAWE` instance.
    ///
    pub fn par_new(rng: &'a mut R, evidence: &'a CatTrjEv) -> Self {
        // Initialize the estimator.
        let mut estimator = Self {
            rng,
            evidence,
            dataset: None,
        };

        // Fill the evidence with the raw estimator.
        estimator.dataset = Some(estimator.par_fill());

        estimator
    }

    /// Sample uncertain evidence.
    /// TODO: Taken from importance sampling, deduplicate.
    fn sample_evidence(&mut self) -> CatTrjEv {
        // Get shortened variable type.
        use CatTrjEvT as E;

        // Sample the evidence for each variable.
        let certain_evidence = self
            .evidence
            // Flatten the evidence.
            .evidences()
            .iter()
            // Map (label, [evidence]) to (label, evidence) pairs.
            .flatten()
            .flat_map(|e| {
                // Get the variable index, starting time, and ending time.
                let (event, start_time, end_time) = (e.event(), e.start_time(), e.end_time());
                // Sample the evidence.
                let e = match e {
                    E::UncertainPositiveInterval { p_states, .. } => {
                        // Construct the sampler.
                        let state = WeightedIndex::new(p_states).unwrap();
                        // Sample the state.
                        let state = state.sample(self.rng);
                        // Return the sample.
                        E::CertainPositiveInterval {
                            event,
                            state,
                            start_time,
                            end_time,
                        }
                    }
                    E::UncertainNegativeInterval { p_not_states, .. } => {
                        // Allocate the not states.
                        let mut not_states: Set<_> = (0..p_not_states.len()).collect();
                        // Repeat until only a subset of the not states are sampled.
                        while not_states.len() == p_not_states.len() {
                            // Sample the not states.
                            not_states = p_not_states
                                .indexed_iter()
                                // For each (state, p_not_state) pair ...
                                .filter_map(|(i, &p_i)| {
                                    // ... with p_i probability, retain the state.
                                    Some(i).filter(|_| self.rng.random_bool(p_i))
                                })
                                .collect();
                        }
                        // Return the sample and weight.
                        E::CertainNegativeInterval {
                            event,
                            not_states,
                            start_time,
                            end_time,
                        }
                    }
                    _ => e.clone(), // Due to evidence sampling.
                };

                // Return the certain evidence.
                Some(e)
            });

        // Collect the certain evidence.
        CatTrjEv::new(self.evidence.states().clone(), certain_evidence)
    }

    /// Fills the evidence with the raw estimator.
    ///
    /// # Arguments
    ///
    /// * `evidence` - A reference to the evidence to fill.
    ///
    /// # Returns
    ///
    /// A new `CatTrj` instance.
    ///
    fn par_fill(&mut self) -> CatTrj {
        // Short the evidence name.
        use CatTrjEvT as E;
        // Set missing placeholder.
        const M: CatType = CatType::MAX;

        // Get labels and states.
        let states = self.evidence.states().clone();

        // Get the ending time of the last event.
        let end_time = self
            .evidence
            .evidences()
            .iter()
            // Get the ending time of each event.
            .flatten()
            .map(|e| e.end_time())
            // Get the maximum time.
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            // Unwrap the maximum time.
            .unwrap_or(0.);

        // Sort the evidence by starting time, adding initial and ending time.
        let times: Array1<_> = self
            .evidence
            .evidences()
            .iter()
            // Get the starting time of each event.
            .flatten()
            .map(|e| e.start_time())
            // Add initial and ending time.
            .chain([0., end_time])
            // Sort the times.
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            // Deduplicate the times to aggregate the events.
            .dedup()
            .collect();

        // Allocate the matrix of events with unknown states.
        let mut events = Array2::from_elem((times.len(), states.len()), M);

        // Reduce the uncertain evidences to certain evidences.
        let evidence = self.sample_evidence();

        // Set the states of the events given the evidence.
        Zip::from(&times)
            .and(events.axis_iter_mut(Axis(0)))
            .par_for_each(|time, mut event| {
                // For each event, set the state of the variable at that time, if any.
                event.iter_mut().enumerate().for_each(|(i, e)| {
                    // Get the evidence vector for that variable.
                    let e_i = &evidence.evidences()[i];
                    // Get the evidence for that time.
                    let e_i_t = e_i.iter().find(|e| e.contains(time));
                    // If the evidence is present, set the state.
                    if let Some(e_i_t) = e_i_t {
                        match e_i_t {
                            E::CertainPositiveInterval { state, .. } => *e = *state as CatType,
                            E::CertainNegativeInterval { .. } => todo!(), // FIXME:
                            _ => unreachable!(), // Due to the previous assertions, this should never happen.
                        }
                    }
                });
            });

        // Get the events with no evidence at all.
        let no_evidence: Vec<_> = events
            .axis_iter(Axis(1))
            .into_par_iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if e.iter().all(|&x| x == M) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        // If no evidence is present, fill it randomly.
        for i in no_evidence {
            // Sample a state uniformly at random.
            let random_state = Array::from_iter({
                let random_state = || self.rng.random_range(0..(states[i].len() as CatType));
                std::iter::repeat_with(random_state).take(events.nrows())
            });
            // Fill the event with the sampled state.
            events.column_mut(i).assign(&random_state);
        }

        // Fill the unknown states by propagating the known states.
        events
            .axis_iter_mut(Axis(1))
            .into_par_iter()
            .for_each(|mut event| {
                // Set the first known state position.
                let mut first_known = 0;
                // Check if the first state is known.
                if event[first_known] == M {
                    // If the first state is unknown, get the first known state.
                    // NOTE: Safe unwrap since we know at least one state is present.
                    first_known = event.iter().position(|e| *e != M).unwrap();
                    // Get the event to fill with.
                    let e = event[first_known];
                    // Backward fill the unknown states.
                    event.slice_mut(s![..first_known]).fill(e);
                }
                // Set the first known state position as the last known state position.
                let mut last_known = first_known;
                // Get the first unknown state.
                while let Some(first_unknown) = event.iter().skip(last_known).position(|e| *e == M)
                {
                    // Add displacement to the first known state position because we skipped some elements.
                    let first_unknown = first_unknown + last_known;
                    // Get the last known state.
                    // NOTE: Safe because we know at least one state is present.
                    let e = event[first_unknown - 1];
                    // Get the last unknown state after the first unknown state.
                    // NOTE: We get the "first known state after the first unknown state",
                    // but we fill with an excluding range, so we can use the same position.
                    let last_unknown = event.iter().skip(first_unknown).position(|e| *e != M);
                    // Add displacement to the first unknown state position because we skipped some elements.
                    let last_unknown =
                        last_unknown.map(|last_unknown| last_unknown + first_unknown);
                    // If no last unknown state, set the end.
                    let last_unknown = last_unknown.unwrap_or(event.len());
                    // Fill the unknown states with the last known state, or till the end if none.
                    event.slice_mut(s![first_unknown..last_unknown]).fill(e);
                    // Set the last known state position as the last unknown state position.
                    last_known = last_unknown;
                }
            });

        // Initialize the events and times with first event and time, if any.
        let mut new_events: Vec<_> = events
            .rows()
            .into_iter()
            .map(|x| x.to_owned())
            .take(1)
            .collect();
        let mut new_times: Vec<_> = times.iter().cloned().take(1).collect();

        // Check if there is at max one state change per transition.
        events
            .rows()
            .into_iter()
            .zip(&times)
            .tuple_windows()
            .for_each(|((e_i, t_i), (e_j, t_j))| {
                // Count the number of state changes.
                let mut diff: Vec<_> = e_i
                    .indexed_iter()
                    .zip(e_j.indexed_iter())
                    .filter_map(|(i, j)| if i != j { Some(j) } else { None })
                    .collect();
                // Check if there is at most one state change.
                if diff.len() <= 1 {
                    // Add the event and time to the new events.
                    new_events.push(e_j.to_owned());
                    new_times.push(*t_j);
                    // Nothing to fix, just return.
                    return;
                }
                // Otherwise, we have multiple state changes.
                // Shuffle them to generate a transition order.
                diff.shuffle(self.rng);
                // Ignore the last state change to avoid overlap with the next event.
                diff.pop();
                // Get the first state change.
                let (mut e_k, mut t_k) = (e_i.to_owned(), *t_i);
                // Compute uniform time delta.
                let t_delta = (t_j - t_i) / (diff.len() + 1) as f64;
                // Generate the events to add to fill the gaps between e_i and e_j.
                diff.into_iter().for_each(|(i, x)| {
                    // Set the state to the event.
                    e_k[i] = *x;
                    // Set the time to the event.
                    t_k += t_delta;
                    // Add the event and time to the new events.
                    new_events.push(e_k.clone());
                    new_times.push(t_k);
                });
                // Add the last event and time to the new events.
                new_events.push(e_j.to_owned());
                new_times.push(*t_j);
            });

        // Reshape the events to the number of events and states.
        let events = Array::from_iter(new_events.into_iter().flatten())
            .into_shape_with_order((new_times.len(), states.len()))
            .expect("Failed to reshape events.");
        // Reshape the times to the number of events.
        let times = Array::from_iter(new_times);

        // Construct the fully observed trajectory.
        CatTrj::new(states, events, times)
    }
}

impl<'a, R: Rng + SeedableRng> RAWE<'a, R, CatTrjsEv, CatTrjs> {
    /// Constructs a new raw estimator from the evidence.
    ///
    /// # Arguments
    ///
    /// * `evidence` - A reference to the evidence to fill.
    ///
    /// # Returns
    ///
    /// A new `RAWE` instance.
    ///
    pub fn par_new(rng: &'a mut R, evidence: &'a CatTrjsEv) -> Self {
        // Get evidence.
        let _evidence = evidence.evidences();
        // Sample seed for parallel sampling.
        let seeds: Vec<_> = (0.._evidence.len()).map(|_| rng.next_u64()).collect();
        // Fill the evidence with the raw estimator.
        let dataset: Option<CatTrjs> = Some(
            seeds
                .into_par_iter()
                .zip(_evidence)
                .map(|(seed, e)| {
                    // Create a new random number generator with the seed.
                    let mut rng = R::seed_from_u64(seed);
                    // Fill the evidence with the raw estimator.
                    RAWE::<'_, R, CatTrjEv, CatTrj>::par_new(&mut rng, e)
                        .dataset
                        .unwrap()
                })
                .collect(),
        );

        Self {
            rng,
            evidence,
            dataset,
        }
    }
}

impl<R: Rng + SeedableRng> CIMEstimator<CatCIM> for RAWE<'_, R, CatTrjEv, CatTrj> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
        // Estimate the CIM with a uniform prior.
        BE::new(self.dataset.as_ref().unwrap())
            .with_prior((1, 1.))
            .fit(x, z)
    }
}

impl<R: Rng + SeedableRng> CIMEstimator<CatCIM> for RAWE<'_, R, CatTrjsEv, CatTrjs> {
    fn fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
        // Estimate the CIM with a uniform prior.
        BE::new(self.dataset.as_ref().unwrap())
            .with_prior((1, 1.))
            .fit(x, z)
    }
}

impl<R: Rng + SeedableRng> ParCIMEstimator<CatCIM> for RAWE<'_, R, CatTrjsEv, CatTrjs> {
    fn par_fit(&self, x: &Set<usize>, z: &Set<usize>) -> CatCIM {
        // Estimate the CIM with a uniform prior.
        BE::new(self.dataset.as_ref().unwrap())
            .with_prior((1, 1.))
            .par_fit(x, z)
    }
}

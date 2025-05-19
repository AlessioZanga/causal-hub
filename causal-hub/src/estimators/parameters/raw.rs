use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

use super::{BE, CPDEstimator, ParCPDEstimator};
use crate::{
    datasets::{CatTrj, CatTrjEv, CatTrjEvT, CatTrjs, CatTrjsEv, Dataset},
    distributions::CatCIM,
};

/// A struct representing a raw estimator.
///
/// This estimator is used to find an initial guess of the parameters with the given evidence.
/// Its purpose is to provide a starting point for the other estimators, like EM.
///
#[derive(Clone, Copy, Debug)]
pub struct RawEstimator<D> {
    dataset: D,
}

/// A type alias for a raw estimator.
pub type RE<D> = RawEstimator<D>;

impl RE<CatTrj> {
    pub fn new(evidence: &CatTrjEv) -> Self {
        // Fill the evidence with the raw estimator.
        let dataset = Self::fill(evidence);

        Self { dataset }
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
    pub fn fill(evidence: &CatTrjEv) -> CatTrj {
        // Short the evidence name.
        use CatTrjEvT as E;
        // Set missing placeholder.
        const M: u8 = u8::MAX;

        // Assert at least one evidence for each variable is present.
        assert!(
            evidence.values().iter().all(|(_, e)| e.len() > 0),
            "At least one evidence for each variable is required."
        );

        // Get labels and states.
        let states = evidence.states();

        // Get the ending time of the last event.
        let end_time = evidence
            .values()
            .iter()
            // Get the ending time of each event.
            .flat_map(|(_, e)| e.iter())
            .map(|e| e.end_time())
            // Get the maximum time.
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            // Unwrap the maximum time.
            .unwrap();

        // Deduplicate and sort the evidence by starting time.
        let times: Array1<_> = evidence
            .values()
            .iter()
            // Get the starting time of each event.
            .flat_map(|(_, e)| e.iter())
            .map(|e| e.start_time())
            // Add initial and ending time.
            .chain([0., end_time])
            // Sort the times.
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            // Deduplicate the times.
            .dedup()
            .collect();
        // Allocate the matrix of events with unknown states.
        let mut events = Array2::from_elem((times.len(), states.len()), M);

        // Set the states of the events given the evidence.
        times
            .iter()
            .zip(events.rows_mut())
            .for_each(|(time, mut event)| {
                // For each event, set the state of the variable at that time, if any.
                event.iter_mut().enumerate().for_each(|(i, e)| {
                    // Get the evidence vector for that variable.
                    let e_i = &evidence.values()[i];
                    // Get the evidence for that time.
                    let e_i_t = e_i.iter().find(|e| e.contains(time));
                    // If the evidence is present, set the state.
                    if let Some(e_i_t) = e_i_t {
                        match e_i_t {
                            E::CertainPositiveInterval { state, .. } => *e = *state as u8,
                            E::CertainNegativeInterval { not_states, .. } => todo!(), // FIXME:
                            E::UncertainPositiveInterval { p_states, .. } => todo!(), // FIXME:
                            E::UncertainNegativeInterval { p_not_states, .. } => todo!(), // FIXME:
                        }
                    }
                });
            });

        // Fill the unknown states by propagating the known states.
        events.columns_mut().into_iter().for_each(|mut event| {
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
            while let Some(fist_unknown) = event.iter().skip(last_known).position(|e| *e == M) {
                // Get the last known state.
                // NOTE: Safe because we know at least one state is present.
                let e = event[fist_unknown - 1];
                // Get the last unknown state after the first unknown state.
                // NOTE: We get the "first known state after the first unknown state",
                // but we fill with an excluding range, so we can use the same position.
                let last_unknown = event.iter().skip(fist_unknown).position(|e| *e != u8::MAX);
                // Get the last unknown state position, or the end if none.
                let last_unknown = last_unknown.unwrap_or(event.len());
                // Fill the unknown states with the last known state, or till the end if none.
                event.slice_mut(s![fist_unknown..last_unknown]).fill(e);
                // Set the last known state position as the last unknown state position.
                last_known = last_unknown;
            }
        });

        // TODO: Random split events if multiple states transition at the same time.

        // Construct the fully observed trajectory.
        CatTrj::new(states, events, times)
    }
}

impl RE<CatTrjs> {
    pub fn new(evidence: &CatTrjsEv) -> Self {
        // Fill the evidence with the raw estimator.
        let dataset: CatTrjs = evidence
            .values()
            .par_iter()
            .map(RE::<CatTrj>::fill)
            .collect();

        Self { dataset }
    }
}

impl CPDEstimator<CatCIM> for RE<CatTrj> {
    // (conditional counts, conditional time spent, sample size)
    type SS = (Array3<f64>, Array2<f64>, f64);

    fn fit_transform(&self, x: usize, z: &[usize]) -> (Self::SS, CatCIM) {
        // Estimate the CIM with a uniform prior.
        BE::new(&self.dataset, (1, 1.)).fit_transform(x, z)
    }
}

impl CPDEstimator<CatCIM> for RE<CatTrjs> {
    // (conditional counts, conditional time spent, sample size)
    type SS = (Array3<f64>, Array2<f64>, f64);

    fn fit_transform(&self, x: usize, z: &[usize]) -> (Self::SS, CatCIM) {
        // Estimate the CIM with a uniform prior.
        BE::new(&self.dataset, (1, 1.)).fit_transform(x, z)
    }
}

impl ParCPDEstimator<CatCIM> for RE<CatTrjs> {
    // (conditional counts, conditional time spent, sample size)
    type SS = (Array3<f64>, Array2<f64>, f64);

    fn par_fit_transform(&self, x: usize, z: &[usize]) -> (Self::SS, CatCIM) {
        // Estimate the CIM with a uniform prior.
        BE::new(&self.dataset, (1, 1.)).par_fit_transform(x, z)
    }
}

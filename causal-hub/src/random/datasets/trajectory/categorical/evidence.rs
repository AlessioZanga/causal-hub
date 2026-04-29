use itertools::Itertools;
use rand::{Rng, RngExt, seq::index::sample};

use crate::{
    datasets::{CatTrj, CatTrjEv, CatTrjEvT, CatTrjs, CatTrjsEv, Dataset},
    random::Random,
    types::{Error, Result},
};

/// A struct representing a random evidence generator.
pub struct RngCatTrjEv<'a, R, D> {
    rng: &'a mut R,
    dataset: &'a D,
    p: f64,
}

impl<'a, R, D> RngCatTrjEv<'a, R, D> {
    /// Creates a new `RngCatTrjEv` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `dataset` - A reference to the dataset.
    /// * `p` - The probability of selecting an evidence.
    ///
    /// # Returns
    ///
    /// A new `RngCatTrjEv` instance.
    pub fn new(rng: &'a mut R, dataset: &'a D, p: f64) -> Result<Self> {
        // Check that the probability is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self { rng, dataset, p })
    }
}

impl<R: Rng> Random for RngCatTrjEv<'_, R, CatTrj> {
    type Output = Result<CatTrjEv>;

    fn random(&mut self) -> Self::Output {
        // Get shortened variable type.
        use CatTrjEvT as E;

        // Get times.
        let times = self.dataset.times();
        // Get events.
        let events = self.dataset.values().rows();
        // Zip times and events.
        let times_events = times.into_iter().zip(events);

        // Iterate over (time, event) pairs.
        let evidence = times_events
            .tuple_windows()
            .filter_map(|((&start_time, v), (&end_time, _))| {
                // Choose if the event is selected.
                if !self.rng.random_bool(self.p) {
                    // If the event is not selected, skip it.
                    return None;
                }
                // Select how many events to select.
                let n = self.rng.random_range(1..=v.len());
                // Sample the events.
                let evidence = sample(self.rng, v.len(), n).into_iter().map(move |index| {
                    // Get label and state.
                    let (event, state) = (index, v[index] as usize);
                    // Create the evidence.
                    E::CertainPositiveInterval {
                        event,
                        state,
                        start_time,
                        end_time,
                    }
                });
                // Return the evidences.
                Some(evidence)
            })
            .flatten();

        // Collect the evidence.
        CatTrjEv::new(self.dataset.states().clone(), evidence)
    }
}

impl<R: Rng> Random for RngCatTrjEv<'_, R, CatTrjs> {
    type Output = Result<CatTrjsEv>;

    fn random(&mut self) -> Self::Output {
        let evidences = self
            .dataset
            .values()
            .iter()
            .map(|trj| RngCatTrjEv::<_, CatTrj>::new(self.rng, trj, self.p)?.random())
            .collect::<Result<Vec<_>>>()?;

        CatTrjsEv::new(evidences)
    }
}

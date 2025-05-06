use core::f64;

use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use rand::{
    Rng,
    distr::{Distribution as _Distribution, weighted::WeightedIndex},
};
use rand_distr::Exp;

use super::{BNSampler, CTBNSampler};
use crate::{
    datasets::{CategoricalDataset, CategoricalTrj},
    distributions::CPD,
    graphs::Graph,
    models::{BayesianNetwork, CategoricalBN, CategoricalCTBN, ContinuousTimeBayesianNetwork},
};

/// A forward sampler.
#[derive(Debug)]
pub struct ForwardSampler<'a, R, M> {
    rng: &'a mut R,
    model: &'a M,
}

impl<'a, R, M> ForwardSampler<'a, R, M> {
    /// Construct a new forward sampler.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    /// * `model` - A reference to the model to sample from.
    ///
    /// # Returns
    ///
    /// Return a new `ForwardSampler` instance.
    ///
    pub fn new(rng: &'a mut R, model: &'a M) -> Self {
        Self { rng, model }
    }
}

impl<R: Rng> BNSampler<CategoricalBN> for ForwardSampler<'_, R, CategoricalBN> {
    #[inline]
    fn sample(&mut self) -> <CategoricalBN as BayesianNetwork>::Sample {
        // Cache the parents.
        let parents: Vec<_> = self
            .model
            .graph()
            .vertices()
            .map(|x| self.model.graph().parents(x))
            .collect();

        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&x| {
            // Get the parents of the vertex.
            let pa_x = &parents[x];
            // Get the CPD.
            let cpd_x = &self.model.cpds()[x];
            // Compute the index on the parents to condition on.
            // NOTE: We can to this because the labels and states are sorted (i.e. aligned).
            let pa_i = pa_x.iter().map(|&z| sample[z] as usize);
            let pa_i = cpd_x.ravel_multi_index().ravel(pa_i);
            // Get the distribution of the vertex.
            let p_x = cpd_x.parameters().row(pa_i);
            // Construct the sampler.
            let s_x = WeightedIndex::new(p_x).expect("Failed to construct sampler.");
            // Sample from the distribution.
            sample[x] = s_x.sample(self.rng) as u8;
        });

        sample
    }

    fn sample_n(&mut self, n: usize) -> <CategoricalBN as BayesianNetwork>::Dataset {
        // Allocate the dataset.
        let mut dataset = Array::zeros((n, self.model.labels().len()));

        // For each sample ...
        dataset
            .rows_mut()
            .into_iter()
            .zip(self.take(n))
            .for_each(|(mut row, sample)| {
                // Sample from the distribution.
                row.assign(&sample);
            });

        // Get the states.
        let states = self
            .model
            .cpds()
            .iter()
            .map(|(label, cpd)| (label, cpd.states()));

        // Construct the dataset.
        CategoricalDataset::new(states, dataset)
    }
}

impl<R: Rng, M> Iterator for ForwardSampler<'_, R, M>
where
    Self: BNSampler<M>,
    M: BayesianNetwork,
{
    type Item = <M as BayesianNetwork>::Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.sample())
    }
}

impl<R: Rng> CTBNSampler<CategoricalCTBN> for ForwardSampler<'_, R, CategoricalCTBN> {
    fn sample(&mut self) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Event {
        todo!() // FIXME:
    }

    #[inline]
    fn sample_n(
        &mut self,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectory {
        // Delegate to the sample_n_or_t function.
        self.sample_n_or_t(n, f64::MAX)
    }

    #[inline]
    fn sample_t(
        &mut self,
        tau: f64,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectory {
        // Delegate to the sample_n_or_t function.
        self.sample_n_or_t(usize::MAX, tau)
    }

    fn sample_n_or_t(
        &mut self,
        n: usize,
        tau: f64,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectory {
        // Assert n is positive.
        assert!(n > 0, "The number of samples must be positive.");
        // Assert tau is positive.
        assert!(tau > 0., "The time must be positive.");

        // Allocate the trajectory components.
        let mut events = Vec::new();
        let mut times = Vec::new();

        // Sample the initial states.
        let mut initial_sampler = ForwardSampler::new(self.rng, self.model.initial_distribution());
        let mut s_i = initial_sampler.sample();
        // Append the initial state to the trajectory.
        events.push(s_i.clone());
        times.push(0.);

        // Function to sample transition times.
        let sample_times =
            |rng: &mut R, model: &CategoricalCTBN, s_i: &Array1<u8>| -> Array1<f64> {
                // Sample the transition times for each variable.
                s_i.iter()
                    .enumerate()
                    .map(|(i, &x)| {
                        // Cast the state to usize.
                        let x = x as usize;
                        // Get the parents of the vertex.
                        let pa_i = model.graph().parents(i);
                        // Get the CIM.
                        let cim_i = &model.cims()[i];
                        // Compute the index on the parents to condition on.
                        let pa_i = pa_i.iter().map(|&z| s_i[z] as usize);
                        let pa_i = cim_i.ravel_multi_index().ravel(pa_i);
                        // Get the distribution of the vertex.
                        let q_i_x = -cim_i.parameters()[[pa_i, x, x]];
                        // Initialize the exponential distribution.
                        let exp_i_x = Exp::new(q_i_x).unwrap();
                        // Sample the transition time.
                        exp_i_x.sample(rng)
                    })
                    .collect()
            };

        // Sample the transition time.
        let mut t_i = sample_times(self.rng, self.model, &s_i);

        // Get the variable that transitions first.
        let mut i = t_i.argmin().unwrap();
        // Set global time.
        let mut t = t_i[i];

        // While:
        //  1. the length of the trajectory is less than n, and ...
        //  2. the time is less than tau ...
        while events.len() < n && t < tau {
            // Cast the state to usize.
            let x = s_i[i] as usize;
            // Get the parents of the vertex.
            let pa_i = self.model.graph().parents(i);
            // Get the CIM.
            let cim_i = &self.model.cims()[i];
            // Compute the index on the parents to condition on.
            let pa_i = pa_i.iter().map(|&z| s_i[z] as usize);
            let pa_i = cim_i.ravel_multi_index().ravel(pa_i);
            // Get the distribution of the vertex.
            let mut q_i_zx = cim_i.parameters().slice(s![pa_i, x, ..]).to_owned();
            // Set the diagonal element to zero.
            q_i_zx[x] = 0.;
            // Initialize a weighted index sampler.
            let s_i_zx = WeightedIndex::new(q_i_zx).unwrap();
            // Sample the next event.
            s_i[x] = s_i_zx.sample(self.rng) as u8;
            // Append the event to the trajectory.
            events.push(s_i.clone());
            times.push(t);
            // Update the transition times.
            t_i += &sample_times(self.rng, self.model, &s_i);
            // Get the variable to transition first.
            i = t_i.argmin().unwrap();
            // Update the global time.
            t += t_i[i];
        }

        // Get the states of the CIMs.
        let states = self
            .model
            .cims()
            .iter()
            .map(|(label, cim)| (label, cim.states()));

        // Convert the events to a 2D array.
        let shape = (events.len(), events[0].len());
        let events = Array::from_iter(events.into_iter().flatten())
            .into_shape_with_order(shape)
            .expect("Failed to convert events to 2D array.");
        // Convert the times to a 1D array.
        let times = Array::from_shape_vec((times.len(),), times)
            .expect("Failed to convert times to 1D array.");

        // Return the trajectory.
        CategoricalTrj::new(states, events, times)
    }
}

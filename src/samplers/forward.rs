use core::f64;

use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use rand::{
    Rng, SeedableRng,
    distr::{Distribution as _Distribution, weighted::WeightedIndex},
};
use rand_distr::Exp;
use rayon::prelude::*;

use super::{BNSampler, CTBNSampler, ParCTBNSampler};
use crate::{
    datasets::{CategoricalDataset, CategoricalTrj},
    distributions::CPD,
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
        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&x| {
            // Get the parents of the vertex.
            let pa_x = self.model.graph().parents(x);
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
        dataset.rows_mut().into_iter().for_each(|mut row| {
            // Sample from the distribution.
            row.assign(&self.sample());
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

impl<R: Rng> CTBNSampler<CategoricalCTBN> for ForwardSampler<'_, R, CategoricalCTBN> {
    #[inline]
    fn sample_by_length(
        &mut self,
        length: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectory {
        // Delegate to generic function.
        self.sample_by_length_or_time(length, f64::MAX)
    }

    #[inline]
    fn sample_by_time(
        &mut self,
        time: f64,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectory {
        // Delegate to generic function.
        self.sample_by_length_or_time(usize::MAX, time)
    }

    fn sample_by_length_or_time(
        &mut self,
        length: usize,
        time: f64,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectory {
        // Assert length is positive.
        assert!(
            length > 0,
            "The length of the trajectory must be strictly positive."
        );
        // Assert time is positive.
        assert!(time > 0., "The time must be positive.");

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
        //  1. the length of the trajectory is less than length, and ...
        //  2. the time is less than time ...
        while events.len() < length && t < time {
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
            s_i[i] = s_i_zx.sample(self.rng) as u8;
            // Append the event to the trajectory.
            events.push(s_i.clone());
            times.push(t);
            // Update the transition times.
            t_i = t + &sample_times(self.rng, self.model, &s_i);
            // Get the variable to transition first.
            i = t_i.argmin().unwrap();
            // Update the global time.
            t = t_i[i];
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

    #[inline]
    fn sample_n_by_length(
        &mut self,
        length: usize,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectories {
        (0..n).map(|_| self.sample_by_length(length)).collect()
    }

    #[inline]
    fn sample_n_by_time(
        &mut self,
        time: f64,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectories {
        (0..n).map(|_| self.sample_by_time(time)).collect()
    }

    #[inline]
    fn sample_n_by_length_or_time(
        &mut self,
        length: usize,
        time: f64,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectories {
        (0..n)
            .map(|_| self.sample_by_length_or_time(length, time))
            .collect()
    }
}

impl<R: Rng + SeedableRng> ParCTBNSampler<CategoricalCTBN>
    for ForwardSampler<'_, R, CategoricalCTBN>
{
    fn par_sample_n_by_length(
        &mut self,
        length: usize,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectories {
        // Generate a random seed for each trajectory.
        let seeds: Vec<u64> = self.rng.random_iter().take(n).collect();
        // Sample the trajectories in parallel.
        seeds
            .into_par_iter()
            .map(|seed| {
                // Create a new random number generator with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the random number generator and model.
                let mut sampler = ForwardSampler::new(&mut rng, self.model);
                // Sample the trajectory.
                sampler.sample_by_length(length)
            })
            .collect()
    }

    fn par_sample_n_by_time(
        &mut self,
        time: f64,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectories {
        // Generate a random seed for each trajectory.
        let seeds: Vec<u64> = self.rng.random_iter().take(n).collect();
        // Sample the trajectories in parallel.
        seeds
            .into_par_iter()
            .map(|seed| {
                // Create a new random number generator with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the random number generator and model.
                let mut sampler = ForwardSampler::new(&mut rng, self.model);
                // Sample the trajectory.
                sampler.sample_by_time(time)
            })
            .collect()
    }

    fn par_sample_n_by_length_or_time(
        &mut self,
        length: usize,
        time: f64,
        n: usize,
    ) -> <CategoricalCTBN as ContinuousTimeBayesianNetwork>::Trajectories {
        // Generate a random seed for each trajectory.
        let seeds: Vec<u64> = self.rng.random_iter().take(n).collect();
        // Sample the trajectories in parallel.
        seeds
            .into_par_iter()
            .map(|seed| {
                // Create a new random number generator with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the random number generator and model.
                let mut sampler = ForwardSampler::new(&mut rng, self.model);
                // Sample the trajectory.
                sampler.sample_by_length_or_time(length, time)
            })
            .collect()
    }
}

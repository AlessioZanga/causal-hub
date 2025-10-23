use core::f64;
use std::cell::RefCell;

use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, weighted::WeightedIndex},
};
use rand_distr::Exp;
use rayon::prelude::*;

use crate::{
    datasets::{CatSample, CatTable, CatTrj, CatType, GaussTable},
    models::{BN, CIM, CPD, CTBN, CatBN, CatCTBN, GaussBN, Labelled},
    samplers::{BNSampler, CTBNSampler, ParBNSampler, ParCTBNSampler},
    set,
    types::EPSILON,
};

/// A forward sampler.
#[derive(Debug)]
pub struct ForwardSampler<'a, R, M> {
    rng: RefCell<&'a mut R>,
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
    #[inline]
    pub const fn new(rng: &'a mut R, model: &'a M) -> Self {
        // Wrap the RNG in a RefCell to allow interior mutability.
        let rng = RefCell::new(rng);

        Self { rng, model }
    }
}

impl<R: Rng> BNSampler<CatBN> for ForwardSampler<'_, R, CatBN> {
    type Sample = <CatBN as BN>::Sample;
    type Samples = <CatBN as BN>::Samples;

    fn sample(&self) -> Self::Sample {
        // Get a mutable reference to the RNG.
        let mut rng = self.rng.borrow_mut();
        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&i| {
            // Get the CPD.
            let cpd_i = &self.model.cpds()[i];
            // Compute the index on the parents to condition on.
            let pa_i = self.model.graph().parents(&set![i]);
            let pa_i = pa_i.iter().map(|&z| sample[z]).collect();
            // Sample from the distribution.
            sample[i] = cpd_i.sample(&mut rng, &pa_i)[0];
        });

        sample
    }

    fn sample_n(&self, n: usize) -> Self::Samples {
        // Allocate the dataset.
        let mut dataset = Array::zeros((n, self.model.labels().len()));

        // For each sample ...
        dataset.rows_mut().into_iter().for_each(|mut row| {
            // Sample from the distribution.
            row.assign(&self.sample());
        });

        // Construct the dataset.
        CatTable::new(self.model.states().clone(), dataset)
    }
}

impl<R: Rng + SeedableRng> ParBNSampler<CatBN> for ForwardSampler<'_, R, CatBN> {
    type Samples = <CatBN as BN>::Samples;

    fn par_sample_n(&self, n: usize) -> Self::Samples {
        // Get a mutable reference to the RNG.
        let rng = self.rng.borrow_mut();
        // Generate a random seed for each sample.
        let seeds: Vec<_> = rng.random_iter().take(n).collect();

        // Allocate the samples.
        let mut samples = Array::zeros((n, self.model.labels().len()));

        // Sample the samples in parallel.
        seeds
            .into_par_iter()
            .zip(samples.axis_iter_mut(Axis(0)))
            .for_each(|(seed, mut row)| {
                // Create a new random number generator with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the random number generator and model.
                let sampler = ForwardSampler::new(&mut rng, self.model);
                // Sample from the distribution.
                row.assign(&sampler.sample());
            });

        // Construct the dataset.
        CatTable::new(self.model.states().clone(), samples)
    }
}

impl<R: Rng> BNSampler<GaussBN> for ForwardSampler<'_, R, GaussBN> {
    type Sample = <GaussBN as BN>::Sample;
    type Samples = <GaussBN as BN>::Samples;

    fn sample(&self) -> Self::Sample {
        // Get a mutable reference to the RNG.
        let mut rng = self.rng.borrow_mut();
        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&i| {
            // Get the CPD.
            let cpd_i = &self.model.cpds()[i];
            // Get the parents.
            let pa_i = self.model.graph().parents(&set![i]);
            let pa_i = pa_i.iter().map(|&z| sample[z]).collect();
            // Compute the value of the variable.
            sample[i] = cpd_i.sample(&mut rng, &pa_i)[0];
        });

        sample
    }

    fn sample_n(&self, n: usize) -> Self::Samples {
        // Allocate the samples.
        let mut samples = Array::zeros((n, self.model.labels().len()));

        // For each sample ...
        samples.rows_mut().into_iter().for_each(|mut row| {
            // Sample from the distribution.
            row.assign(&self.sample());
        });

        // Construct the dataset.
        GaussTable::new(self.model.labels().clone(), samples)
    }
}

impl<R: Rng + SeedableRng> ParBNSampler<GaussBN> for ForwardSampler<'_, R, GaussBN> {
    type Samples = <GaussBN as BN>::Samples;

    fn par_sample_n(&self, n: usize) -> Self::Samples {
        // Get a mutable reference to the RNG.
        let rng = self.rng.borrow_mut();
        // Generate a random seed for each sample.
        let seeds: Vec<_> = rng.random_iter().take(n).collect();

        // Allocate the samples.
        let mut samples = Array::zeros((n, self.model.labels().len()));

        // Sample the samples in parallel.
        seeds
            .into_par_iter()
            .zip(samples.axis_iter_mut(Axis(0)))
            .for_each(|(seed, mut row)| {
                // Create a new random number generator with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the random number generator and model.
                let sampler = ForwardSampler::new(&mut rng, self.model);
                // Sample from the distribution.
                row.assign(&sampler.sample());
            });

        // Construct the dataset.
        GaussTable::new(self.model.labels().clone(), samples)
    }
}

impl<R: Rng> ForwardSampler<'_, R, CatCTBN> {
    /// Sample transition time for variable X_i with state x_i.
    fn sample_time(&self, event: &CatSample, i: usize) -> f64 {
        // Cast the state to usize.
        let x = event[i] as usize;
        // Get the CIM.
        let cim_i = &self.model.cims()[i];
        // Compute the index on the parents to condition on.
        let pa_i = self.model.graph().parents(&set![i]);
        let pa_i = pa_i.iter().map(|&z| event[z] as usize);
        let pa_i = cim_i.conditioning_multi_index().ravel(pa_i);
        // Get the distribution of the vertex.
        let q_i_x = -cim_i.parameters()[[pa_i, x, x]];
        // Initialize the exponential distribution.
        let exp_i_x = Exp::new(q_i_x).unwrap();
        // Sample the transition time.
        exp_i_x.sample(&mut self.rng.borrow_mut())
    }
}

impl<R: Rng> CTBNSampler<CatCTBN> for ForwardSampler<'_, R, CatCTBN> {
    type Sample = <CatCTBN as CTBN>::Trajectory;
    type Samples = <CatCTBN as CTBN>::Trajectories;

    #[inline]
    fn sample_by_length(&self, max_length: usize) -> Self::Sample {
        // Delegate to generic function.
        self.sample_by_length_or_time(max_length, f64::MAX)
    }

    #[inline]
    fn sample_by_time(&self, max_time: f64) -> Self::Sample {
        // Delegate to generic function.
        self.sample_by_length_or_time(usize::MAX, max_time)
    }

    fn sample_by_length_or_time(&self, max_length: usize, max_time: f64) -> Self::Sample {
        // Assert length is positive.
        assert!(
            max_length > 0,
            "The maximum length of the trajectory must be strictly positive."
        );
        // Assert time is positive.
        assert!(max_time > 0., "The maximum time must be positive.");

        // Allocate the trajectory components.
        let mut sample_events = Vec::new();
        let mut sample_times = Vec::new();

        // Sample the initial states.
        let mut event = {
            let mut rng = self.rng.borrow_mut();
            let initial = self.model.initial_distribution();
            let initial = ForwardSampler::new(&mut rng, initial);
            initial.sample()
        };
        // Append the initial state to the trajectory.
        sample_events.push(event.clone());
        sample_times.push(0.);

        // Sample the transition time.
        let mut times: Array1<_> = (0..event.len())
            .map(|i| self.sample_time(&event, i))
            .collect();

        // Get the variable that transitions first.
        let mut i = times.argmin().unwrap();
        // Set global time.
        let mut time = times[i];

        // While:
        //  1. the length of the trajectory is less than max_length, and ...
        //  2. the time is less than max_time ...
        while sample_events.len() < max_length && time < max_time {
            // Cast the state to usize.
            let x = event[i] as usize;
            // Get the CIM.
            let cim_i = &self.model.cims()[i];
            // Compute the index on the parents to condition on.
            let pa_i = self.model.graph().parents(&set![i]);
            let pa_i = pa_i.iter().map(|&z| event[z] as usize);
            let pa_i = cim_i.conditioning_multi_index().ravel(pa_i);
            // Get the distribution of the vertex.
            let mut q_i_zx = cim_i.parameters().slice(s![pa_i, x, ..]).to_owned();
            // Set the diagonal element to zero.
            q_i_zx[x] = 0.;
            // Normalize the probabilities.
            q_i_zx /= q_i_zx.sum();
            // Initialize a weighted index sampler.
            let s_i_zx = WeightedIndex::new(&q_i_zx).unwrap();
            // Sample the next event.
            event[i] = s_i_zx.sample(&mut self.rng.borrow_mut()) as CatType;
            // Append the event to the trajectory.
            sample_events.push(event.clone());
            sample_times.push(time);
            // Update the transition times for { X } U Ch(X).
            std::iter::once(i)
                .chain(self.model.graph().children(&set![i]))
                .for_each(|j| {
                    // Sample the transition time.
                    times[j] = time + self.sample_time(&event, j);
                });
            // Add a small epsilon to avoid zero transition times.
            times += EPSILON;
            // Get the variable to transition first.
            i = times.argmin().unwrap();
            // Update the global time.
            time = times[i];
        }

        // Get the states of the CIMs.
        let states = self.model.states().clone();

        // Convert the events to a 2D array.
        let shape = (sample_events.len(), sample_events[0].len());
        let sample_events = Array::from_iter(sample_events.into_iter().flatten())
            .into_shape_with_order(shape)
            .expect("Failed to convert events to 2D array.");
        // Convert the times to a 1D array.
        let sample_times = Array::from_iter(sample_times);

        // Return the trajectory.
        CatTrj::new(states, sample_events, sample_times)
    }

    #[inline]
    fn sample_n_by_length(&self, max_length: usize, n: usize) -> Self::Samples {
        (0..n).map(|_| self.sample_by_length(max_length)).collect()
    }

    #[inline]
    fn sample_n_by_time(&self, max_time: f64, n: usize) -> Self::Samples {
        (0..n).map(|_| self.sample_by_time(max_time)).collect()
    }

    #[inline]
    fn sample_n_by_length_or_time(
        &self,
        max_length: usize,
        max_time: f64,
        n: usize,
    ) -> Self::Samples {
        (0..n)
            .map(|_| self.sample_by_length_or_time(max_length, max_time))
            .collect()
    }
}

impl<R: Rng + SeedableRng> ParCTBNSampler<CatCTBN> for ForwardSampler<'_, R, CatCTBN> {
    type Samples = <CatCTBN as CTBN>::Trajectories;

    #[inline]
    fn par_sample_n_by_length(&self, max_length: usize, n: usize) -> Self::Samples {
        self.par_sample_n_by_length_or_time(max_length, f64::MAX, n)
    }

    #[inline]
    fn par_sample_n_by_time(&self, max_time: f64, n: usize) -> Self::Samples {
        self.par_sample_n_by_length_or_time(usize::MAX, max_time, n)
    }

    fn par_sample_n_by_length_or_time(
        &self,
        max_length: usize,
        max_time: f64,
        n: usize,
    ) -> Self::Samples {
        // Get a mutable reference to the RNG.
        let rng = self.rng.borrow_mut();
        // Generate a random seed for each trajectory.
        let seeds: Vec<_> = rng.random_iter().take(n).collect();
        // Sample the trajectories in parallel.
        seeds
            .into_par_iter()
            .map(|seed| {
                // Create a new random number generator with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the random number generator and model.
                let sampler = ForwardSampler::new(&mut rng, self.model);
                // Sample the trajectory.
                sampler.sample_by_length_or_time(max_length, max_time)
            })
            .collect()
    }
}

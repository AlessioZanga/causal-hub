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
    datasets::{
        CatEv, CatEvT, CatSample, CatTable, CatTrj, CatTrjEv, CatTrjEvT, CatType, CatWtdSample,
        CatWtdTable, CatWtdTrj, CatWtdTrjs, GaussEv, GaussEvT, GaussTable, GaussType,
        GaussWtdSample, GaussWtdTable,
    },
    models::{BN, CIM, CPD, CTBN, CatBN, CatCTBN, GaussBN, Labelled},
    samplers::{BNSampler, CTBNSampler, ParBNSampler, ParCTBNSampler},
    set,
    types::{EPSILON, Set},
};

/// A struct for sampling using importance sampling.
#[derive(Debug)]
pub struct ImportanceSampler<'a, R, M, E> {
    rng: RefCell<&'a mut R>,
    model: &'a M,
    evidence: &'a E,
}

impl<'a, R, M, E> ImportanceSampler<'a, R, M, E>
where
    M: Labelled,
    E: Labelled,
{
    /// Construct a new importance sampler.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    /// * `model` - A reference to the model to sample from.
    /// * `evidence` - A reference to the evidence to sample from.
    ///
    /// # Returns
    ///
    /// Return a new `ImportanceSampler` instance.
    ///
    #[inline]
    pub fn new(rng: &'a mut R, model: &'a M, evidence: &'a E) -> Self {
        // Wrap the RNG in a RefCell to allow interior mutability.
        let rng = RefCell::new(rng);

        // Assert the model and the evidences have the same labels.
        assert_eq!(
            model.labels(),
            evidence.labels(),
            "The model and the evidences must have the same variables."
        );

        Self {
            rng,
            model,
            evidence,
        }
    }
}

impl<R: Rng> ImportanceSampler<'_, R, CatBN, CatEv> {
    /// Sample uncertain evidence.
    fn sample_evidence<T: Rng>(&self, rng: &mut T) -> CatEv {
        // Get shortened variable type.
        use CatEvT as E;

        // Sample the evidence for each variable.
        let certain_evidence = self
            .evidence
            // Flatten the evidence.
            .evidences()
            .iter()
            // Filter empty evidences.
            .filter_map(|e| {
                e.as_ref().map(|e| {
                    // Get the event index.
                    let event = e.event();
                    // Sample the evidence.
                    match e {
                        E::UncertainPositive { p_states, .. } => {
                            // Construct the sampler.
                            let state = WeightedIndex::new(p_states).unwrap();
                            // Sample the state.
                            let state = state.sample(rng);
                            // Return the sample.
                            E::CertainPositive { event, state }
                        }
                        E::UncertainNegative { p_not_states, .. } => {
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
                                        Some(i).filter(|_| rng.random_bool(p_i))
                                    })
                                    .collect();
                            }
                            // Return the sample and weight.
                            E::CertainNegative { event, not_states }
                        }
                        _ => e.clone(), // Due to evidence sampling.
                    }
                })
            });

        // Collect the certain evidence.
        CatEv::new(self.evidence.states().clone(), certain_evidence)
    }
}

impl<R: Rng> BNSampler<CatBN> for ImportanceSampler<'_, R, CatBN, CatEv> {
    type Sample = CatWtdSample;
    type Samples = CatWtdTable;

    fn sample(&self) -> Self::Sample {
        // Get shortened variable type.
        use CatEvT as E;

        // Assert the model and the evidences have the same states.
        // TODO: Move this assertion to the constructor.
        assert_eq!(
            self.model.states(),
            self.evidence.states(),
            "The model and the evidences must have the same states."
        );

        // Get a mutable reference to the RNG.
        let mut rng = self.rng.borrow_mut();
        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());
        // Initialize the weight.
        let mut weight = 1.;

        // Reduce the uncertain evidences to certain evidences.
        let evidence = self.sample_evidence(&mut rng);

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&i| {
            // Get the evidence of the vertex.
            let e_i = &evidence.evidences()[i];

            // Get the CPD.
            let cpd_i = &self.model.cpds()[i];
            // Compute the index on the parents to condition on.
            let pa_i = self.model.graph().parents(&set![i]);
            let pa_i = pa_i.iter().map(|&z| sample[z] as usize);
            let pa_i = cpd_i.conditioning_multi_index().ravel(pa_i);
            // Get the distribution of the vertex.
            let p_i = cpd_i.parameters().row(pa_i);

            // Get the evidence of the vertex.
            let (s_i, w_i) = match e_i {
                // If there is evidence, sample from the constrained distribution.
                Some(e_i) => match e_i {
                    E::CertainPositive { state, .. } => {
                        // Get the state.
                        let s_i = *state as CatType;
                        // Return the state and its weight.
                        (s_i, p_i[*state])
                    }
                    E::CertainNegative { not_states, .. } => {
                        // Initialize the weight.
                        let mut w_i = 1.;
                        // Clone the distribution.
                        let mut p_i = p_i.to_owned();
                        // For each not state ...
                        not_states.iter().for_each(|&j| {
                            // Update the weight.
                            w_i -= p_i[j];
                            // Zero out the not states.
                            p_i[j] = 0.;
                        });
                        // Normalize the probabilities.
                        p_i /= p_i.sum();
                        // Construct the sampler.
                        let s_i = WeightedIndex::new(&p_i).unwrap();
                        // Sample the state.
                        let s_i = s_i.sample(&mut rng) as CatType;
                        // Return the sample and weight.
                        (s_i, w_i)
                    }
                    _ => unreachable!(), // Due to evidence sampling.
                },
                // If there is no evidence, sample as usual.
                None => {
                    // Construct the sampler.
                    let s_i = WeightedIndex::new(&p_i).unwrap();
                    // Sample the state.
                    let s_i = s_i.sample(&mut rng) as CatType;
                    // Return the sample and weight.
                    (s_i, 1.)
                }
            };

            // Sample from the distribution.
            sample[i] = s_i;
            // Update the weight.
            weight *= w_i;
        });

        (sample, weight)
    }

    fn sample_n(&self, n: usize) -> Self::Samples {
        // Allocate the samples.
        let mut samples = Array2::zeros((n, self.model.labels().len()));
        // Allocate the weights.
        let mut weights = Array1::zeros(n);

        // Sample the weighted samples.
        samples
            .rows_mut()
            .into_iter()
            .zip(weights.iter_mut())
            .for_each(|(mut sample, weight)| {
                // Sample a weighted sample.
                let (s_i, w_i) = self.sample();
                // Assign the sample.
                sample.assign(&s_i);
                // Assign the weight.
                *weight = w_i;
            });

        // Construct the samples.
        let samples = CatTable::new(self.model.states().clone(), samples);

        // Return the weighted samples.
        CatWtdTable::new(samples, weights)
    }
}

impl<R: Rng> BNSampler<GaussBN> for ImportanceSampler<'_, R, GaussBN, GaussEv> {
    type Sample = GaussWtdSample;
    type Samples = GaussWtdTable;

    fn sample(&self) -> Self::Sample {
        // Get shortened variable type.
        use GaussEvT as E;

        // Get a mutable reference to the RNG.
        let mut rng = self.rng.borrow_mut();
        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());
        // Initialize the weight.
        let mut weight = 1.;

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&i| {
            // Get the evidence of the vertex.
            let e_i = &self.evidence.evidences()[i];

            // Get the CPD.
            let cpd_i = &self.model.cpds()[i];
            // Compute the index on the parents to condition on.
            let pa_i = self.model.graph().parents(&set![i]);
            let pa_i = pa_i.iter().map(|&z| sample[z]).collect();

            // Get the evidence of the vertex.
            let (s_i, w_i) = match e_i {
                // If there is evidence, sample from the constrained distribution.
                Some(e_i) => match e_i {
                    E::CertainPositive { value, .. } => {
                        // Get the state.
                        let s_i = *value;
                        // Get the probability.
                        let p_i = cpd_i.pf(&array![s_i], &pa_i);
                        // Return the state and its weight.
                        (s_i, p_i)
                    }
                },
                // If there is no evidence, sample as usual.
                None => {
                    // Sample from the distribution.
                    let s_i = cpd_i.sample(&mut rng, &pa_i)[0];
                    // Return the sample and weight.
                    (s_i, 1.)
                }
            };

            // Sample from the distribution.
            sample[i] = s_i;
            // Update the weight.
            weight *= w_i;
        });

        (sample, weight)
    }
    fn sample_n(&self, n: usize) -> Self::Samples {
        // Allocate the samples.
        let mut samples = Array2::zeros((n, self.model.labels().len()));
        // Allocate the weights.
        let mut weights = Array1::zeros(n);

        // Sample the weighted samples.
        samples
            .rows_mut()
            .into_iter()
            .zip(weights.iter_mut())
            .for_each(|(mut sample, weight)| {
                // Sample a weighted sample.
                let (s_i, w_i) = self.sample();
                // Assign the sample.
                sample.assign(&s_i);
                // Assign the weight.
                *weight = w_i;
            });

        // Construct the samples.
        let samples = GaussTable::new(self.model.labels().clone(), samples);

        // Return the weighted samples.
        GaussWtdTable::new(samples, weights)
    }
}

impl<R: Rng + SeedableRng> ParBNSampler<CatBN> for ImportanceSampler<'_, R, CatBN, CatEv> {
    type Samples = CatWtdTable;

    fn par_sample_n(&self, n: usize) -> Self::Samples {
        // Allocate the samples.
        let mut samples: Array2<CatType> = Array::zeros((n, self.model.labels().len()));
        // Allocate the weights.
        let mut weights: Array1<f64> = Array::zeros(n);

        // Get a mutable reference to the RNG.
        let rng = self.rng.borrow_mut();
        // Generate a random seed for each trajectory.
        let seeds: Vec<_> = rng.random_iter().take(n).collect();
        // Sample the trajectories in parallel.
        seeds
            .into_par_iter()
            .zip(samples.axis_iter_mut(Axis(0)))
            .zip(weights.axis_iter_mut(Axis(0)))
            .for_each(|((seed, mut sample), mut weight)| {
                // Create a new RNG with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the RNG.
                let sampler = ImportanceSampler::new(&mut rng, self.model, self.evidence);
                // Sample a weighted sample.
                let (s_i, w_i) = sampler.sample();
                // Assign the sample.
                sample.assign(&s_i);
                // Assign the weight.
                weight.fill(w_i);
            });

        // Construct the samples.
        let samples = CatTable::new(self.model.states().clone(), samples);

        // Return the weighted samples.
        CatWtdTable::new(samples, weights)
    }
}

impl<R: Rng + SeedableRng> ParBNSampler<GaussBN> for ImportanceSampler<'_, R, GaussBN, GaussEv> {
    type Samples = GaussWtdTable;

    fn par_sample_n(&self, n: usize) -> Self::Samples {
        // Allocate the samples.
        let mut samples: Array2<GaussType> = Array::zeros((n, self.model.labels().len()));
        // Allocate the weights.
        let mut weights: Array1<f64> = Array::zeros(n);

        // Get a mutable reference to the RNG.
        let rng = self.rng.borrow_mut();
        // Generate a random seed for each trajectory.
        let seeds: Vec<_> = rng.random_iter().take(n).collect();
        // Sample the trajectories in parallel.
        seeds
            .into_par_iter()
            .zip(samples.axis_iter_mut(Axis(0)))
            .zip(weights.axis_iter_mut(Axis(0)))
            .for_each(|((seed, mut sample), mut weight)| {
                // Create a new RNG with the seed.
                let mut rng = R::seed_from_u64(seed);
                // Create a new sampler with the RNG.
                let sampler = ImportanceSampler::new(&mut rng, self.model, self.evidence);
                // Sample a weighted sample.
                let (s_i, w_i) = sampler.sample();
                // Assign the sample.
                sample.assign(&s_i);
                // Assign the weight.
                weight.fill(w_i);
            });

        // Construct the samples.
        let samples = GaussTable::new(self.model.labels().clone(), samples);

        // Return the weighted samples.
        GaussWtdTable::new(samples, weights)
    }
}

impl<R: Rng> ImportanceSampler<'_, R, CatCTBN, CatTrjEv> {
    /// Sample uncertain evidence.
    fn sample_evidence<T: Rng>(&self, rng: &mut T) -> CatTrjEv {
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
                        let state = state.sample(rng);
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
                                    Some(i).filter(|_| rng.random_bool(p_i))
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

    /// Sample transition time for variable X_i with state x_i.
    fn sample_time<T: Rng>(
        &self,
        rng: &mut T,
        evidence: &CatTrjEv,
        event: &CatSample,
        i: usize,
        t: f64,
    ) -> f64 {
        // Get shortened variable type.
        use CatTrjEvT as E;

        // Get the evidence of the vertex.
        let e_i = &evidence.evidences()[i];

        // Check if there is certain positive evidence at this point in time.
        let e = e_i.iter().find(|e| match e {
            E::CertainPositiveInterval { .. } => e.contains(&t),
            E::CertainNegativeInterval { .. } => false, // Due to state sampling.
            _ => unreachable!(),                        // Due to evidence sampling.
        });

        // If there is certain positive evidence return the time until the end.
        if let Some(e) = e {
            return e.end_time() - t;
        }

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

        // Find an upcoming evidence, if any.
        let e = e_i.iter().find(|e| t < e.start_time());
        // Check if there is conflict between current state and upcoming evidence.
        let e = e.filter(|e| match e {
            E::CertainPositiveInterval { state, .. } => *state != x,
            E::CertainNegativeInterval { not_states, .. } => not_states.contains(&x),
            _ => unreachable!(), // Due to evidence sampling.
        });

        // If there is a conflict ...
        if let Some(e) = e {
            // Get the time until the conflict.
            let t_c = e.start_time() - t;
            // Sample from a uniform distribution in the range [0, 1).
            let u = rng.random_range(0.0..1.0);
            // Sample from a truncated exponential distribution, where:
            //  1. The lower bound is 0.
            //  2. The upper bound is the time until the conflict.
            //  3. The rate is the negative of the transition rate.
            return -1. / q_i_x * f64::ln(1. - u * (1. - f64::exp(-q_i_x * t_c)));
        }

        // If there is no conflict, initialize the exponential distribution.
        let exp_i_x = Exp::new(q_i_x).unwrap();
        // Sample the transition time.
        let t_i = exp_i_x.sample(rng);

        // Find an upcoming evidence, if any.
        let e = e_i.iter().find(|e| t < e.start_time());
        // Check if there is compliance between the current state and upcoming evidence ...
        let e = e.filter(|e| match e {
            // ... for which starting time is greater than the sampled transition time.
            E::CertainPositiveInterval { state, .. } => (t_i + t) > e.start_time() && *state == x,
            E::CertainNegativeInterval { .. } => false, // Due to state sampling.
            _ => unreachable!(),                        // Due to evidence sampling.
        });

        // If there is compliance ...
        if let Some(e) = e {
            // Get the time until the compliance.
            return e.start_time() - t;
        }

        // Otherwise, return the transition time.
        t_i
    }

    fn update_weight(
        &self,
        evidence: &CatTrjEv,
        event: &CatSample,
        i: usize,
        t_a: f64,
        t_b: f64,
    ) -> f64 {
        // Get shortened variable type.
        use CatTrjEvT as E;

        // For each ...
        event
            .indexed_iter()
            .map(|(j, &y)| {
                // Get the evidence of the vertex.
                let e_j = &evidence.evidences()[j];

                // Cast the state to usize.
                let y = y as usize;
                // Get the CIM.
                let cim_j = &self.model.cims()[j];
                // Compute the index on the parents to condition on.
                let pa_j = self.model.graph().parents(&set![j]);
                let pa_j = pa_j.iter().map(|&z| event[z] as usize);
                let pa_j = cim_j.conditioning_multi_index().ravel(pa_j);
                // Get the distribution of the vertex.
                let q_j_y = -cim_j.parameters()[[pa_j, y, y]];

                // Check if there is certain positive evidence at this point in time.
                let e = e_j.iter().find(|e| match e {
                    E::CertainPositiveInterval { .. } => e.contains(&t_a),
                    E::CertainNegativeInterval { .. } => false, // Due to state sampling.
                    _ => unreachable!(),                        // Due to evidence sampling.
                });
                // Find an upcoming evidence, if any. NOTE: t_a < start_time .
                let e_next = e_j.iter().find(|e| t_a < e.start_time());
                // Check if there is a difference between current state and upcoming evidence.
                let e_next = e_next.filter(|e| match e {
                    E::CertainPositiveInterval { state, .. } => *state != y,
                    E::CertainNegativeInterval { not_states, .. } => not_states.contains(&y),
                    _ => unreachable!(), // Due to evidence sampling.
                });
                // Check if current state has been set to a certain positive evidence, or
                // if the upcoming evidence is non-existent or set given a certain negative evidence.
                if let (
                    Some(E::CertainPositiveInterval { .. }),
                    None | Some(E::CertainNegativeInterval { .. }),
                ) = (e, e_next)
                {
                    return f64::exp(-q_j_y * (t_b - t_a));
                }

                // Find an upcoming evidence, if any. NOTE: t_b < start_time .
                let e = e_j.iter().find(|e| t_b < e.start_time());
                // Check if there is conflict between current state and upcoming evidence.
                let e = e.filter(|e| match e {
                    E::CertainPositiveInterval { state, .. } => *state != y,
                    E::CertainNegativeInterval { not_states, .. } => not_states.contains(&y),
                    _ => unreachable!(), // Due to evidence sampling.
                });
                // If there is a conflict ...
                if let Some(e) = e {
                    // Get starting time of the evidence.
                    let t_e = e.start_time();
                    // Check if the variable is the same as the one that transitioned.
                    return if i == j {
                        1. - f64::exp(-q_j_y * (t_e - t_a))
                    } else {
                        (1. - f64::exp(-q_j_y * (t_e - t_a))) / // .
                        (1. - f64::exp(-q_j_y * (t_e - t_b)))
                    };
                }

                // Otherwise, return one.
                1.
            })
            // Check numeric stability.
            .map(|w| if !w.is_finite() { 1. } else { w.clamp(0., 1.) })
            // Collect the weights.
            .product()
    }
}

impl<R: Rng> CTBNSampler<CatCTBN> for ImportanceSampler<'_, R, CatCTBN, CatTrjEv> {
    type Sample = CatWtdTrj;
    type Samples = CatWtdTrjs;

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
        // Get shortened variable type.
        use CatTrjEvT as E;

        // Assert the model and the evidences have the same states.
        // TODO: Move this assertion to the constructor.
        assert_eq!(
            self.model.states(),
            self.evidence.states(),
            "The model and the evidences must have the same states."
        );
        // Assert length is positive.
        assert!(
            max_length > 0,
            "The maximum length of the trajectory must be strictly positive."
        );
        // Assert time is positive.
        assert!(max_time > 0., "The maximum time must be positive.");

        // Get a mutable reference to the RNG.
        let mut rng = self.rng.borrow_mut();

        // Allocate the trajectory components.
        let mut sample_events = Vec::new();
        let mut sample_times = Vec::new();

        // Reduce the uncertain evidences to certain evidences.
        let evidence = self.sample_evidence(&mut rng);

        // Sample the initial states with given initial evidence.
        let (mut event, mut weight) = {
            // Get the initial state distribution.
            let initial_d = self.model.initial_distribution();
            // Get the initial evidence.
            let initial_e = &evidence.initial_evidence();
            // Initialize the sampler for the initial state.
            let initial = ImportanceSampler::new(&mut rng, initial_d, initial_e);
            // Sample the initial state.
            initial.sample()
        };

        // Append the initial state to the trajectory.
        sample_events.push(event.clone());
        sample_times.push(0.);

        // Sample the transition time.
        let mut times: Array1<_> = (0..event.len())
            .map(|i| self.sample_time(&mut rng, &evidence, &event, i, 0.))
            .collect();

        // Get the variable that transitions first.
        let mut i = times.argmin().unwrap();
        // Update the weight.
        weight *= self.update_weight(&evidence, &event, i, 0., times[i]);
        // Set global time.
        let mut time = times[i];

        // While:
        //  1. the length of the trajectory is less than max_length, and ...
        //  2. the time is less than max_time ...
        while sample_events.len() < max_length && time < max_time {
            // Get evidence of the vertex.
            let e_i = &evidence.evidences()[i];

            // Cast the state to usize.
            let x = event[i] as usize;

            // Check if there is evidence at this point in time.
            let e = e_i.iter().find(|e| e.contains(&time));
            // Check if there is certain evidence at this point in time.
            if e.is_some_and(|e| match e {
                E::CertainPositiveInterval { state, .. } => *state == x,
                E::CertainNegativeInterval { not_states, .. } => !not_states.contains(&x),
                _ => false,
            }) {
                // Sample the transition time.
                times[i] = time + self.sample_time(&mut rng, &evidence, &event, i, time);
            } else {
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

                // Check if there is evidence at this point in time.
                let (s_i, w_i) = if e.is_some_and(|e| match e {
                    E::CertainPositiveInterval { state, .. } => *state != x,
                    _ => false,
                }) {
                    // Get the state of the certain positive interval.
                    match e {
                        Some(E::CertainPositiveInterval { state, .. }) => {
                            (*state as CatType, q_i_zx[*state])
                        }
                        _ => unreachable!(), // Due to previous checks.
                    }
                } else {
                    //
                    match e {
                        Some(E::CertainNegativeInterval { not_states, .. }) => {
                            // Initialize the weight.
                            let mut w_i = 1.;
                            // Clone the distribution.
                            let mut q_i_zx = q_i_zx.to_owned();
                            // For each not state ...
                            not_states.iter().for_each(|&j| {
                                // Update the weight.
                                w_i -= q_i_zx[j];
                                // Zero out the not states.
                                q_i_zx[j] = 0.;
                            });
                            // Normalize the probabilities.
                            q_i_zx /= q_i_zx.sum();
                            // Construct the sampler.
                            let s_i = WeightedIndex::new(&q_i_zx).unwrap();
                            // Sample the state.
                            let s_i = s_i.sample(&mut rng) as CatType;
                            // Return the sample and weight.
                            (s_i, w_i)
                        }
                        None => {
                            // Initialize a weighted index sampler.
                            let s_i_zx = WeightedIndex::new(&q_i_zx).unwrap();
                            // Sample the next event.
                            let s_i = s_i_zx.sample(&mut rng) as CatType;
                            // Return the sample and weight.
                            (s_i, 1.)
                        }
                        _ => unreachable!(), // Due to previous checks.
                    }
                };

                // Set the state.
                event[i] = s_i;
                // Update the weight.
                weight *= w_i;

                // Append the event to the trajectory.
                sample_events.push(event.clone());
                sample_times.push(time);
                // Update the transition times for { X } U Ch(X).
                std::iter::once(i)
                    .chain(self.model.graph().children(&set![i]))
                    .for_each(|j| {
                        // Sample the transition time.
                        times[j] = time + self.sample_time(&mut rng, &evidence, &event, j, time);
                    });
            }

            // Add a small epsilon to avoid zero transition times.
            times += EPSILON;
            // Get the variable to transition first.
            i = times.argmin().unwrap();
            // Update the weight.
            weight *= self.update_weight(&evidence, &event, i, time, times[i].min(max_time));
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

        // Construct the trajectory.
        let trajectory = CatTrj::new(states, sample_events, sample_times);

        // Return the trajectory and its weight.
        (trajectory, weight).into()
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

impl<R: Rng + SeedableRng> ParCTBNSampler<CatCTBN> for ImportanceSampler<'_, R, CatCTBN, CatTrjEv> {
    type Samples = CatWtdTrjs;

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
                let sampler = ImportanceSampler::new(&mut rng, self.model, self.evidence);
                // Sample the trajectory.
                sampler.sample_by_length_or_time(max_length, max_time)
            })
            .collect()
    }
}

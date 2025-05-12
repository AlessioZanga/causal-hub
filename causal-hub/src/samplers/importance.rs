use core::f64;
use std::f32::consts::E;

use approx::relative_eq;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use rand::{
    Rng,
    distr::{Distribution as _Distribution, weighted::WeightedIndex},
};
use rand_distr::Exp;

use crate::{
    datasets::{
        CategoricalEv, CategoricalEvT, CategoricalTrj, CategoricalTrjEv, CategoricalTrjEvT, Dataset,
    },
    distributions::CPD,
    models::{BN, CTBN, CategoricalBN, CategoricalCTBN},
};

#[derive(Debug)]
pub struct ImportanceSampler<'a, R, M> {
    rng: &'a mut R,
    model: &'a M,
}

impl<'a, R, M> ImportanceSampler<'a, R, M> {
    /// Construct a new importance sampler.
    ///
    /// # Arguments
    ///
    /// * `rng` - A random number generator.
    /// * `model` - A reference to the model to sample from.
    ///
    /// # Returns
    ///
    /// Return a new `ImportanceSampler` instance.
    ///
    pub fn new(rng: &'a mut R, model: &'a M) -> Self {
        Self { rng, model }
    }
}

impl<'a, R: Rng> ImportanceSampler<'a, R, CategoricalBN> {
    /// Sample uncertain evidence.
    fn sample_evidence(&mut self, evidence: &CategoricalEv) -> CategoricalEv {
        // Get shortened variable type.
        use CategoricalEvT as E;

        // Sample the evidence for each variable.
        let certain_evidence = evidence
            // Flatten the evidence.
            .evidences()
            .iter()
            // Filter empty evidences.
            .filter_map(|(l, e)| e.clone().map(|e| (l, e)))
            .flat_map(|(l, e)| {
                // Sample the evidence.
                let e = match e {
                    E::UncertainPositive { p_states, .. } => {
                        // Construct the sampler.
                        let state = WeightedIndex::new(p_states).unwrap();
                        // Sample the state.
                        let state = state.sample(self.rng);
                        // Return the sample.
                        E::CertainPositive { state }
                    }
                    E::UncertainNegative { p_not_states, .. } => {
                        // Sample the not states.
                        let not_states = p_not_states
                            .indexed_iter()
                            // For each (state, p_not_state) pair ...
                            .filter_map(|(i, &p_i)| {
                                // ... with p_i probability, retain the state.
                                Some(i).filter(|_| self.rng.random_bool(p_i))
                            })
                            .collect();
                        // Return the sample and weight.
                        E::CertainNegative { not_states }
                    }
                    _ => e.clone(), // Due to evidence sampling.
                };

                // Return the certain evidence.
                Some((l, e))
            });

        // Collect the certain evidence.
        CategoricalEv::new(evidence.states(), certain_evidence)
    }

    pub fn sample_with_evidence(&mut self, evidence: &CategoricalEv) -> (Array1<u8>, f64) {
        // Get shortened variable type.
        use CategoricalEvT as E;

        // Allocate the sample.
        let mut sample = Array::zeros(self.model.labels().len());
        // Initialize the weight.
        let mut weight = 1.;

        // Reduce the uncertain evidences to certain evidences.
        let evidence = self.sample_evidence(&evidence);

        // For each vertex in the topological order ...
        self.model.topological_order().iter().for_each(|&i| {
            // Get the evidence of the vertex.
            let e_i = &evidence.evidences()[i];

            // Get the CPD.
            let cpd_i = &self.model.cpds()[i];
            // Compute the index on the parents to condition on.
            // NOTE: Labels and states are sorted (i.e. aligned).
            let pa_i = self.model.graph().parents(i);
            let pa_i = pa_i.iter().map(|&z| sample[z] as usize);
            let pa_i = cpd_i.ravel_multi_index().ravel(pa_i);
            // Get the distribution of the vertex.
            let p_i = cpd_i.parameters().row(pa_i);

            // Get the evidence of the vertex.
            let (s_i, w_i) = match e_i {
                // If there is evidence, sample from the constrained distribution.
                Some(e_i) => match e_i {
                    E::CertainPositive { state, .. } => {
                        // Get the state.
                        let s_i = *state as u8;
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
                        let s_i = s_i.sample(self.rng) as u8;
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
                    let s_i = s_i.sample(self.rng) as u8;
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
}

impl<'a, R: Rng> ImportanceSampler<'a, R, CategoricalCTBN> {
    /// Sample uncertain evidence.
    fn sample_evidence(&mut self, evidence: &CategoricalTrjEv) -> CategoricalTrjEv {
        // Get shortened variable type.
        use CategoricalTrjEvT as E;

        // Sample the evidence for each variable.
        let certain_evidence = evidence
            // Flatten the evidence.
            .evidences()
            .iter()
            // Map (label, [evidence]) to (label, evidence) pairs.
            .map(|(l, e)| std::iter::repeat(l).zip(e))
            .flatten()
            .flat_map(|(l, e)| {
                // Get the variable index, starting time, and ending time.
                let (start_time, end_time) = (e.start_time(), e.end_time());
                // Sample the evidence.
                let e = match e {
                    E::UncertainPositiveInterval { p_states, .. } => {
                        // Construct the sampler.
                        let state = WeightedIndex::new(p_states).unwrap();
                        // Sample the state.
                        let state = state.sample(self.rng);
                        // Return the sample.
                        E::CertainPositiveInterval {
                            state,
                            start_time,
                            end_time,
                        }
                    }
                    E::UncertainNegativeInterval { p_not_states, .. } => {
                        // Sample the not states.
                        let not_states = p_not_states
                            .indexed_iter()
                            // For each (state, p_not_state) pair ...
                            .filter_map(|(i, &p_i)| {
                                // ... with p_i probability, retain the state.
                                Some(i).filter(|_| self.rng.random_bool(p_i))
                            })
                            .collect();
                        // Return the sample and weight.
                        E::CertainNegativeInterval {
                            not_states,
                            start_time,
                            end_time,
                        }
                    }
                    _ => e.clone(), // Due to evidence sampling.
                };

                // Return the certain evidence.
                Some((l, e))
            });

        // Collect the certain evidence.
        CategoricalTrjEv::new(evidence.states(), certain_evidence)
    }

    /// Sample transition time for variable X_i with state x_i.
    fn sample_time(
        &mut self,
        evidence: &CategoricalTrjEv,
        event: &Array1<u8>,
        i: usize,
        t: f64,
    ) -> f64 {
        // Get shortened variable type.
        use CategoricalTrjEvT as E;

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
        let pa_i = self.model.graph().parents(i);
        let pa_i = pa_i.iter().map(|&z| event[z] as usize);
        let pa_i = cim_i.ravel_multi_index().ravel(pa_i);
        // Get the distribution of the vertex.
        let q_i_x = -cim_i.parameters()[[pa_i, x, x]];

        // Find an upcoming evidence, if any.
        let e = e_i.iter().find(|e| t < e.start_time());
        // Check if there is conflict between current state and upcoming evidence.
        let e = e.filter(|e| match e {
            E::CertainPositiveInterval { state, .. } => state != &x,
            E::CertainNegativeInterval { not_states, .. } => not_states.contains(&x),
            _ => unreachable!(), // Due to evidence sampling.
        });

        // If there is a conflict ...
        if let Some(e) = e {
            // Get the time until the conflict.
            let t_c = e.start_time() - t;
            // Sample from a uniform distribution in the range [0, 1).
            let u = self.rng.random_range(0.0..1.0);
            // Sample from a truncated exponential distribution, where:
            //  1. The lower bound is 0.
            //  2. The upper bound is the time until the conflict.
            //  3. The rate is the negative of the transition rate.
            return -1. / q_i_x * f64::ln(1. - u * (1. - f64::exp(-q_i_x * t_c)));
        }

        // If there is no conflict, initialize the exponential distribution.
        let exp_i_x = Exp::new(q_i_x).unwrap();
        // Sample the transition time.
        let t_i = exp_i_x.sample(self.rng);

        // Find an upcoming evidence, if any.
        let e = e_i.iter().find(|e| t < e.start_time());
        // Check if there is compliance between the current state and upcoming evidence ...
        let e = e.filter(|e| match e {
            // ... for which starting time is greater than the sampled transition time.
            E::CertainPositiveInterval { state, .. } => (t_i + t) > e.start_time() && state == &x,
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
        evidence: &CategoricalTrjEv,
        event: &Array1<u8>,
        i: usize,
        t_a: f64,
        t_b: f64,
    ) -> f64 {
        // Get shortened variable type.
        use CategoricalTrjEvT as E;

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
                let pa_j = self.model.graph().parents(j);
                let pa_j = pa_j.iter().map(|&z| event[z] as usize);
                let pa_j = cim_j.ravel_multi_index().ravel(pa_j);
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
                    E::CertainPositiveInterval { state, .. } => state != &y,
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
                    E::CertainPositiveInterval { state, .. } => state != &y,
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
            // Collect the weights.
            .product()
    }

    pub fn sample_with_evidence_by_length_or_time(
        &mut self,
        evidence: &CategoricalTrjEv,
        max_length: usize,
        max_time: f64,
    ) -> (CategoricalTrj, f64) {
        // Get shortened variable type.
        use CategoricalTrjEvT as E;

        // Assert the model and the evidences have the same labels.
        assert_eq!(
            self.model.labels(),
            evidence.labels(),
            "The model and the evidences must have the same variables."
        );
        // TODO: Assert the model and the evidences have the same states.
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

        // Reduce the uncertain evidences to certain evidences.
        let evidence = self.sample_evidence(&evidence);

        // Get the initial state distribution.
        let initial_distribution = self.model.initial_distribution();
        // Get the initial evidence.
        let initial_evidence = evidence.initial_evidence();
        // Initialize the sampler for the initial state.
        let mut initial_sampler = ImportanceSampler::new(self.rng, initial_distribution);
        // Sample the initial states with given initial evidence.
        let (mut event, mut weight) = initial_sampler.sample_with_evidence(&initial_evidence);

        // Append the initial state to the trajectory.
        sample_events.push(event.clone());
        sample_times.push(0.);

        // Sample the transition time.
        let mut times: Array1<_> = (0..event.len())
            .map(|i| self.sample_time(&evidence, &event, i, 0.))
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
            // FIXME:
            if true {
                // Sample the transition time.
                times[i] = time + self.sample_time(&evidence, &event, i, time);
            // FIXME:
            } else {
                // Get the CIM.
                let cim_i = &self.model.cims()[i];
                // Compute the index on the parents to condition on.
                let pa_i = self.model.graph().parents(i);
                let pa_i = pa_i.iter().map(|&z| event[z] as usize);
                let pa_i = cim_i.ravel_multi_index().ravel(pa_i);
                // Get the distribution of the vertex.
                let mut q_i_zx = cim_i.parameters().slice(s![pa_i, x, ..]).to_owned();
                // Set the diagonal element to zero.
                q_i_zx[x] = 0.;

                // Check if there is evidence at this point in time.
                match e {
                    _ => todo!(), // FIXME:
                };

                // Append the event to the trajectory.
                sample_events.push(event.clone());
                sample_times.push(time);
                // Update the transition times for { X } U Ch(X).
                [i].into_iter()
                    .chain(self.model.graph().children(i))
                    .for_each(|j| {
                        // Sample the transition time.
                        times[j] = time + self.sample_time(&evidence, &event, j, time);
                    });
            }

            // Get the variable to transition first.
            i = times.argmin().unwrap();
            // Update the weight.
            weight *= self.update_weight(&evidence, &event, i, time, times[i].min(max_time));
            // Update the global time.
            time = times[i];
        }

        // Get the states of the CIMs.
        let states = self
            .model
            .cims()
            .iter()
            .map(|(label, cim)| (label, cim.states()));

        // Convert the events to a 2D array.
        let shape = (sample_events.len(), sample_events[0].len());
        let sample_events = Array::from_iter(sample_events.into_iter().flatten())
            .into_shape_with_order(shape)
            .expect("Failed to convert events to 2D array.");
        // Convert the times to a 1D array.
        let sample_times = Array::from_shape_vec((sample_times.len(),), sample_times)
            .expect("Failed to convert times to 1D array.");

        // Construct the trajectory.
        let trajectory = CategoricalTrj::new(states, sample_events, sample_times);

        // Return the trajectory and its weight.
        (trajectory, weight)
    }
}

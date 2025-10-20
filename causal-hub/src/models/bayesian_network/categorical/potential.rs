use std::ops::{Div, DivAssign, Mul, MulAssign};

use approx::{AbsDiffEq, RelativeEq};
use itertools::Itertools;
use ndarray::prelude::*;

use crate::{
    datasets::{CatEv, CatEvT},
    models::{CPD, CatCPD, Labelled, Phi},
    types::{Labels, Set, States},
};

/// A categorical potential.
#[derive(Clone, Debug)]
pub struct CatPhi {
    labels: Labels,
    states: States,
    shape: Array1<usize>,
    parameters: ArrayD<f64>,
}

impl Labelled for CatPhi {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for CatPhi {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.parameters.eq(&other.parameters)
    }
}

impl AbsDiffEq for CatPhi {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self.parameters.abs_diff_eq(&other.parameters, epsilon)
    }
}

impl RelativeEq for CatPhi {
    fn default_max_relative() -> Self::Epsilon {
        Self::Epsilon::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.labels.eq(&other.labels)
            && self.states.eq(&other.states)
            && self.shape.eq(&other.shape)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl MulAssign<&CatPhi> for CatPhi {
    fn mul_assign(&mut self, rhs: &CatPhi) {
        // Get the union of the states.
        let mut states = self.states.clone();
        states.extend(rhs.states.clone());
        // Sort the states by labels.
        states.sort_keys();

        // Order LHS axes w.r.t. new states.
        let mut lhs_axes: Vec<_> = (0..self.states.len()).collect();
        lhs_axes.sort_by_key(|&i| self.states.get_index(i).unwrap().0);
        let mut lhs_parameters = self.parameters.clone().permuted_axes(lhs_axes);
        // Get the axes to insert for LHS broadcasting.
        let lhs_axes = states.keys().enumerate();
        let lhs_axes = lhs_axes.filter_map(|(i, k)| (!self.states.contains_key(k)).then_some(i));
        let lhs_axes: Vec<_> = lhs_axes.sorted().collect();
        // Insert axes in sorted order for LHS broadcasting.
        lhs_axes.into_iter().for_each(|i| {
            lhs_parameters.insert_axis_inplace(Axis(i));
        });

        // Order RHS axes w.r.t. new states.
        let mut rhs_axes: Vec<_> = (0..rhs.states.len()).collect();
        rhs_axes.sort_by_key(|&i| rhs.states.get_index(i).unwrap().0);
        let mut rhs_parameters = rhs.parameters.clone().permuted_axes(rhs_axes);
        // Get the axes to insert for RHS broadcasting.
        let rhs_axes = states.keys().enumerate();
        let rhs_axes = rhs_axes.filter_map(|(i, k)| (!rhs.states.contains_key(k)).then_some(i));
        let rhs_axes: Vec<_> = rhs_axes.sorted().collect();
        // Insert axes in sorted order for RHS broadcasting.
        rhs_axes.into_iter().for_each(|i| {
            rhs_parameters.insert_axis_inplace(Axis(i));
        });

        // Perform element-wise multiplication.
        let parameters = lhs_parameters * rhs_parameters;

        // Get new labels.
        let labels: Labels = states.keys().cloned().collect();
        // Get new shape.
        let shape = Array::from_iter(states.values().map(Set::len));

        // Update self.
        self.states = states;
        self.labels = labels;
        self.shape = shape;
        self.parameters = parameters;
    }
}

impl Mul<&CatPhi> for &CatPhi {
    type Output = CatPhi;

    #[inline]
    fn mul(self, rhs: &CatPhi) -> Self::Output {
        let mut lhs = self.clone();
        lhs *= rhs;
        lhs
    }
}

impl DivAssign<&CatPhi> for CatPhi {
    fn div_assign(&mut self, rhs: &CatPhi) {
        // Assert that RHS states are a subset of LHS states.
        assert!(
            rhs.states.keys().all(|k| self.states.contains_key(k)),
            "Failed to divide potentials: \n\
            \t expected:    RHS states to be a subset of LHS states , \n\
            \t found:       LHS states = {:?} , \n\
            \t              RHS states = {:?} .",
            self.states,
            rhs.states,
        );

        // Add a small constant to ensure 0 / 0 = 0.
        let rhs_parameters = &rhs.parameters + f64::MIN_POSITIVE;

        // Order RHS axes w.r.t. new states.
        let mut rhs_axes: Vec<_> = (0..rhs.states.len()).collect();
        rhs_axes.sort_by_key(|&i| rhs.states.get_index(i).unwrap().0);
        let mut rhs_parameters = rhs_parameters.permuted_axes(rhs_axes);
        // Get the axes to insert for RHS broadcasting.
        let rhs_axes = self.states.keys().enumerate();
        let rhs_axes = rhs_axes.filter_map(|(i, k)| (!rhs.states.contains_key(k)).then_some(i));
        let rhs_axes: Vec<_> = rhs_axes.sorted().collect();
        // Insert axes in sorted order for RHS broadcasting.
        rhs_axes.into_iter().for_each(|i| {
            rhs_parameters.insert_axis_inplace(Axis(i));
        });

        // Perform element-wise division with 0 / 0 = 0.
        self.parameters /= &rhs_parameters;
    }
}

impl Div<&CatPhi> for &CatPhi {
    type Output = CatPhi;

    #[inline]
    fn div(self, rhs: &CatPhi) -> Self::Output {
        let mut lhs = self.clone();
        lhs /= rhs;
        lhs
    }
}

impl Phi for CatPhi {
    type CPD = CatCPD;
    type Parameters = ArrayD<f64>;
    type Evidence = CatEv;

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    fn parameters_size(&self) -> usize {
        self.parameters.len()
    }

    fn condition(&self, e: &Self::Evidence) -> Self {
        // Assert that the evidence states match the potential states.
        assert_eq!(
            e.states(),
            self.states(),
            "Failed to condition on evidence: \n\
            \t expected:    evidence states to match potential states , \n\
            \t found:       potential states = {:?} , \n\
            \t              evidence  states = {:?} .",
            self.states(),
            e.states(),
        );

        // Get the evidence and remove nones.
        let e = e.evidences().iter().flatten();
        // Assert that the evidence is certain and positive.
        let e = e.cloned().map(|e| match e {
            CatEvT::CertainPositive { event, state } => (event, state),
            _ => panic!(
                "Failed to condition on evidence: \n
                \t expected:    CertainPositive , \n\
                \t found:       {:?} .",
                e
            ),
        });

        // Get states and parameters.
        let mut states = self.states.clone();
        let mut parameters = self.parameters.clone();

        // Condition in reverse order to avoid axis shifting.
        e.rev().for_each(|(event, state)| {
            parameters.index_axis_inplace(Axis(event), state);
            states.shift_remove_index(event);
        });

        // Return self.
        Self::new(states, parameters)
    }

    fn marginalize(&self, x: &Set<usize>) -> Self {
        // Base case: if no variables to marginalize, return self.
        if x.is_empty() {
            return self.clone();
        }

        // Assert X is a subset of the variables.
        x.iter().for_each(|&x| {
            assert!(
                x < self.labels.len(),
                "Variable index out of bounds: \n\
                \t expected:    x <  {} , \n\
                \t found:       x == {} .",
                self.labels.len(),
                x,
            );
        });

        // Get the states and the parameters.
        let states = self.states.clone();
        let mut parameters = self.parameters.clone();

        // Filter the states.
        let states = states.into_iter().enumerate();
        let states = states.filter_map(|(i, s)| (!x.contains(&i)).then_some(s));
        let states = states.collect();

        // Sum over the axes in reverse order to avoid shifting.
        x.iter().sorted().rev().for_each(|&i| {
            parameters = parameters.sum_axis(Axis(i));
        });

        // Return the new potential.
        Self::new(states, parameters)
    }

    #[inline]
    fn normalize(&self) -> Self {
        // Get the parameters.
        let mut parameters = self.parameters.clone();
        // Normalize the parameters.
        parameters /= parameters.sum();
        // Return the new potential.
        Self::new(self.states.clone(), parameters)
    }

    fn from_cpd(cpd: Self::CPD) -> Self {
        // Merge conditioning states and states in this order.
        let mut states = cpd.conditioning_states().clone();
        states.extend(cpd.states().clone());
        // Get n-dimensional shape.
        let shape: Vec<_> = states.values().map(Set::len).collect();
        // Reshape the parameters to match the new shape.
        let parameters = cpd.parameters().clone();
        let parameters = parameters
            .into_dyn()
            .into_shape_with_order(shape)
            .expect("Failed to reshape parameters.");

        // Get the new axes order w.r.t. sorted labels.
        let mut axes: Vec<_> = (0..states.len()).collect();
        axes.sort_by_key(|&i| states.get_index(i).unwrap().0);
        // Sort the states by labels.
        states.sort_keys();
        // Swap axes to match the new order.
        let parameters = parameters.permuted_axes(axes);

        // Return the new potential.
        Self::new(states, parameters)
    }

    fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> Self::CPD {
        // Assert that X and Z are disjoint.
        assert!(
            x.is_disjoint(z),
            "Variables and conditioning variables must be disjoint."
        );
        // Assert that X and Z cover all variables.
        assert!(
            (x | z).iter().sorted().cloned().eq(0..self.labels.len()),
            "Variables and conditioning variables must cover all potential variables."
        );

        // Split states into states and conditioning states.
        let states_x: States = x
            .iter()
            .map(|&i| {
                self.states
                    .get_index(i)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .unwrap()
            })
            .collect();
        let states_z: States = z
            .iter()
            .map(|&i| {
                self.states
                    .get_index(i)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .unwrap()
            })
            .collect();

        // Get new axes order.
        let axes: Vec<_> = z.iter().chain(x).cloned().collect();
        // Permute parameters to match the new order.
        let parameters = self.parameters.permuted_axes(axes);
        // Get the new 2D shape.
        let shape: (usize, usize) = (
            states_z.values().map(Set::len).product(),
            states_x.values().map(Set::len).product(),
        );
        // Reshape the parameters to the new 2D shape.
        let mut parameters = parameters
            .into_shape_clone(shape)
            .expect("Failed to reshape parameters.");

        // Normalize the parameters.
        parameters /= &parameters.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Create the new CPD.
        CatCPD::new(states_x, states_z, parameters)
    }
}

impl CatPhi {
    /// Creates a new categorical potential.
    ///
    /// # Arguments
    ///
    /// * `states` - A map from variable names to their possible states.
    /// * `parameters` - A multi-dimensional array of parameters.
    ///
    /// # Returns
    ///
    /// A new categorical potential instance.
    ///
    pub fn new(mut states: States, mut parameters: ArrayD<f64>) -> Self {
        // Get labels.
        let mut labels: Labels = states.keys().cloned().collect();
        // Get shape.
        let mut shape = Array::from_iter(states.values().map(Set::len));
        // Assert parameters shape matches states shape.
        assert_eq!(
            parameters.shape(),
            shape.as_slice().unwrap(),
            "Parameters shape does not match states shape: \n\
            \t expected:    {:?} , \n\
            \t found:       {:?} .",
            shape,
            parameters.shape(),
        );

        // Sort states if not sorted and permute parameters accordingly.
        if !states.keys().is_sorted() {
            // Get the new axes order w.r.t. sorted labels.
            let mut axes: Vec<_> = (0..states.len()).collect();
            axes.sort_by_key(|&i| states.get_index(i).unwrap().0);
            // Sort the states by labels.
            states.sort_keys();
            // Permute the parameters to match the new order.
            parameters = parameters.permuted_axes(axes);
            // Update the labels.
            labels = states.keys().cloned().collect();
            // Update the shape.
            shape = states.values().map(Set::len).collect();
        }

        Self {
            labels,
            states,
            shape,
            parameters,
        }
    }

    /// States of the potential.
    ///
    /// # Returns
    ///
    /// A reference to the states.
    ///
    #[inline]
    pub const fn states(&self) -> &States {
        &self.states
    }

    /// Shape of the potential.
    ///
    /// # Returns
    ///
    /// A reference to the shape.
    ///
    #[inline]
    pub const fn shape(&self) -> &Array1<usize> {
        &self.shape
    }
}

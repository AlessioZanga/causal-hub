use std::ops::{Div, DivAssign, Mul, MulAssign};

use itertools::Itertools;
use ndarray::{Zip, prelude::*};

use crate::{
    models::{CPD, CatCPD, Labelled},
    types::{Labels, Set, States},
};

/// A categorical potential.
#[derive(Debug, Clone)]
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
    pub fn new(states: States, parameters: ArrayD<f64>) -> Self {
        // FIXME: Sort states if not sorted and permute parameters accordingly.

        // Get labels.
        let labels: Labels = states.keys().cloned().collect();
        // Get shape.
        let shape: Array1<_> = states.values().map(|s| s.len()).collect();
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

    /// Parameters of the potential.
    ///
    /// # Returns
    ///
    /// A reference to the parameters.
    ///
    #[inline]
    pub const fn parameters(&self) -> &ArrayD<f64> {
        &self.parameters
    }

    /// Conditions the potential on a set of variables.
    ///
    /// # Arguments
    ///
    /// * `x` - A map from variable indices to their observed states.
    ///
    /// # Returns
    ///
    /// A new categorical potential instance.
    ///
    pub fn condition(&self, _x: ()) -> Self {
        todo!() // FIXME:
    }

    /// Marginalizes the potential over a set of variables.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of variable indices to marginalize over.
    ///
    /// # Returns
    ///
    /// A new categorical potential instance.
    ///
    pub fn marginalize(&self, x: &Set<usize>) -> Self {
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
        let states = states.filter_map(|(i, s)| if !x.contains(&i) { Some(s) } else { None });
        let states = states.collect();

        // Sum over the axes in reverse order to avoid shifting.
        x.iter().sorted().rev().for_each(|&i| {
            parameters = parameters.sum_axis(Axis(i));
        });

        // Return the new potential.
        Self::new(states, parameters)
    }

    /// Normalizes the potential.
    ///
    /// # Returns
    ///
    /// The normalized potential.
    ///
    #[inline]
    pub fn normalize(&mut self) -> &mut Self {
        self.parameters /= self.parameters.sum();
        self
    }

    /// Converts a CPD P(X | Z) to a potential \phi(X \cup Z).
    ///
    /// # Arguments
    ///
    /// * `cpd` - The CPD to convert.
    ///
    /// # Returns
    ///
    /// The corresponding potential.
    ///
    pub fn from_cpd(cpd: CatCPD) -> Self {
        // Merge conditioning states and states in this order.
        let mut states = cpd.conditioning_states().clone();
        states.extend(cpd.states().clone());
        // Get n-dimensional shape.
        let shape: Vec<_> = states.values().map(|s| s.len()).collect();
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

    /// Converts a potential \phi(X \cup Z) to a CPD P(X | Z).
    ///
    /// # Arguments
    ///
    /// * `x` - The set of variables.
    /// * `z` - The set of conditioning variables.
    ///
    /// # Returns
    ///
    /// The corresponding CPD.
    ///
    pub fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> CatCPD {
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

        // Split states into states.
        let states_x = self.states.clone().into_iter().enumerate();
        let states_x = states_x.filter_map(|(i, s)| if x.contains(&i) { Some(s) } else { None });
        let states_x: States = states_x.collect();
        // Split states into conditioning states.
        let states_z = self.states.clone().into_iter().enumerate();
        let states_z = states_z.filter_map(|(i, s)| if z.contains(&i) { Some(s) } else { None });
        let states_z: States = states_z.collect();

        // Get new axes order.
        let axes = z.iter().sorted();
        let axes = axes.chain(x.iter().sorted());
        let axes: Vec<_> = axes.cloned().collect();
        // Permute parameters to match the new order.
        let parameters = self.parameters.permuted_axes(axes);
        // Get the new 2D shape.
        let shape: (usize, usize) = (
            states_z.values().map(|s| s.len()).product(),
            states_x.values().map(|s| s.len()).product(),
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

impl MulAssign for CatPhi {
    fn mul_assign(&mut self, _rhs: Self) {
        todo!() // FIXME:
    }
}

impl Mul for CatPhi {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Mul for &CatPhi {
    type Output = CatPhi;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut lhs = self.clone();
        lhs *= rhs.clone();
        lhs
    }
}

impl Mul for &mut CatPhi {
    type Output = CatPhi;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut lhs = self.clone();
        lhs *= rhs.clone();
        lhs
    }
}

impl DivAssign for CatPhi {
    fn div_assign(&mut self, rhs: Self) {
        // Assert that the two potentials have the same states.
        assert_eq!(
            self.states, rhs.states,
            "Cannot divide potentials with different states: \n\
            \t expected states: {:?} , \n\
            \t found states:    {:?} .",
            self.states, rhs.states,
        );

        // Perform element-wise division with 0 / 0 = 0.
        Zip::from(&mut self.parameters)
            .and(&rhs.parameters)
            .for_each(|lhs, &rhs| {
                // If lhs != 0 && rhs != 0 ...
                let flag = (lhs != &0.) && (rhs != 0.);
                // ... then perform the division, else set to 0.
                *lhs = if flag { *lhs / rhs } else { 0. };
            });
    }
}

impl Div for CatPhi {
    type Output = Self;

    #[inline]
    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl Div for &CatPhi {
    type Output = CatPhi;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let mut lhs = self.clone();
        lhs /= rhs.clone();
        lhs
    }
}

impl Div for &mut CatPhi {
    type Output = CatPhi;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let mut lhs = self.clone();
        lhs /= rhs.clone();
        lhs
    }
}

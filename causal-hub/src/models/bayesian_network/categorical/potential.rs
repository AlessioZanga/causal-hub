use std::{
    borrow::Cow,
    ops::{Div, DivAssign, Mul, MulAssign},
};

use approx::{AbsDiffEq, RelativeEq};
use itertools::Itertools;
use ndarray::prelude::*;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

use crate::{
    datasets::{CatEv, CatEvT},
    impl_json_io,
    models::{CPD, CatCPD, CatSupport, Labelled, Phi},
    types::{Error, Labels, Result, Set},
};

/// A categorical potential.
#[derive(Clone, Debug)]
pub struct CatPhi {
    labels: Labels,
    support: CatSupport,
    shape: Array1<usize>,
    parameters: ArrayD<f64>,
}

impl CatPhi {
    /// Creates a new categorical potential.
    ///
    /// # Arguments
    ///
    /// * `support` - A map from variable names to their possible support.
    /// * `parameters` - A multi-dimensional array of parameters.
    ///
    /// # Returns
    ///
    /// A new categorical potential instance.
    ///
    pub fn new(mut support: CatSupport, mut parameters: ArrayD<f64>) -> Result<Self> {
        // Get labels.
        let mut labels: Labels = support.keys().cloned().collect();
        // Get shape.
        let mut shape = Array::from_iter(support.values().map(Set::len));
        // Validate parameters shape matches support shape.
        let shape_slice = shape.as_slice().ok_or_else(|| {
            Error::Shape("Failed to convert shape array to slice: shape is not contiguous")
        })?;
        if parameters.shape() != shape_slice {
            return Err(Error::Shape(&format!(
                "Parameters shape does not match support shape: \n\
                \t expected:    {:?} , \n\
                \t found:       {:?} .",
                shape_slice,
                parameters.shape(),
            )));
        }

        // Sort support if not sorted and permute parameters accordingly.
        if !support.keys().is_sorted() {
            // Get the new axes order w.r.t. sorted labels.
            let mut axes: Vec<_> = (0..support.len()).collect();
            axes.sort_by(|&i, &j| {
                support
                    .get_index(i)
                    .map(|(l, _)| l)
                    .cmp(&support.get_index(j).map(|(l, _)| l))
            });
            // Sort the support by labels.
            support.sort_keys();
            // Permute the parameters to match the new order.
            parameters = parameters.permuted_axes(axes);
            // Update the labels.
            labels = support.keys().cloned().collect();
            // Update the shape.
            shape = support.values().map(Set::len).collect();
        }

        Ok(Self {
            labels,
            support,
            shape,
            parameters,
        })
    }

    /// CatSupport of the potential.
    ///
    /// # Returns
    ///
    /// A reference to the support.
    ///
    #[inline]
    pub const fn support(&self) -> &CatSupport {
        &self.support
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

impl Labelled for CatPhi {
    #[inline]
    fn labels(&self) -> &Labels {
        &self.labels
    }
}

impl PartialEq for CatPhi {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels)
            && self.support.eq(&other.support)
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
            && self.support.eq(&other.support)
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
            && self.support.eq(&other.support)
            && self.shape.eq(&other.shape)
            && self
                .parameters
                .relative_eq(&other.parameters, epsilon, max_relative)
    }
}

impl MulAssign<&CatPhi> for CatPhi {
    fn mul_assign(&mut self, rhs: &CatPhi) {
        // Get the union of the support.
        let mut support = self.support.clone();
        support.extend(rhs.support.clone());
        // Sort the support by labels.
        support.sort_keys();

        // Order LHS axes w.r.t. new support.
        let mut lhs_axes: Vec<_> = (0..self.support.len()).collect();
        lhs_axes.sort_by(|&i, &j| {
            self.support
                .get_index(i)
                .map(|(l, _)| l)
                .cmp(&self.support.get_index(j).map(|(l, _)| l))
        });
        let mut lhs_parameters = self.parameters.clone().permuted_axes(lhs_axes);
        // Get the axes to insert for LHS broadcasting.
        let lhs_axes = support.keys().enumerate();
        let lhs_axes = lhs_axes.filter_map(|(i, k)| (!self.support.contains_key(k)).then_some(i));
        let lhs_axes: Vec<_> = lhs_axes.sorted().collect();
        // Insert axes in sorted order for LHS broadcasting.
        lhs_axes.into_iter().for_each(|i| {
            lhs_parameters.insert_axis_inplace(Axis(i));
        });

        // Order RHS axes w.r.t. new support.
        let mut rhs_axes: Vec<_> = (0..rhs.support.len()).collect();
        rhs_axes.sort_by(|&i, &j| {
            rhs.support
                .get_index(i)
                .map(|(l, _)| l)
                .cmp(&rhs.support.get_index(j).map(|(l, _)| l))
        });
        let mut rhs_parameters = rhs.parameters.clone().permuted_axes(rhs_axes);
        // Get the axes to insert for RHS broadcasting.
        let rhs_axes = support.keys().enumerate();
        let rhs_axes = rhs_axes.filter_map(|(i, k)| (!rhs.support.contains_key(k)).then_some(i));
        let rhs_axes: Vec<_> = rhs_axes.sorted().collect();
        // Insert axes in sorted order for RHS broadcasting.
        rhs_axes.into_iter().for_each(|i| {
            rhs_parameters.insert_axis_inplace(Axis(i));
        });

        // Perform element-wise multiplication.
        let parameters = lhs_parameters * rhs_parameters;

        // Get new labels.
        let labels: Labels = support.keys().cloned().collect();
        // Get new shape.
        let shape = Array::from_iter(support.values().map(Set::len));

        // Update self.
        self.support = support;
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
        // Check that RHS support are a subset of LHS support.
        if !rhs.support.keys().all(|k| self.support.contains_key(k)) {
            panic!(
                "Failed to divide potentials: RHS support must be a subset of LHS support, \
                found LHS support = {:?}, RHS support = {:?}",
                self.support, rhs.support,
            );
        }

        // Add a small constant to ensure 0 / 0 = 0.
        let rhs_parameters = &rhs.parameters + f64::MIN_POSITIVE;

        // Order RHS axes w.r.t. new support.
        let mut rhs_axes: Vec<_> = (0..rhs.support.len()).collect();
        rhs_axes.sort_by(|&i, &j| {
            rhs.support
                .get_index(i)
                .map(|(l, _)| l)
                .cmp(&rhs.support.get_index(j).map(|(l, _)| l))
        });
        let mut rhs_parameters = rhs_parameters.permuted_axes(rhs_axes);
        // Get the axes to insert for RHS broadcasting.
        let rhs_axes = self.support.keys().enumerate();
        let rhs_axes = rhs_axes.filter_map(|(i, k)| (!rhs.support.contains_key(k)).then_some(i));
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
    type Support = CatSupport;
    type Parameters = ArrayD<f64>;
    type Evidence = CatEv;

    #[inline]
    fn support(&self) -> Cow<'_, Self::Support> {
        Cow::Borrowed(&self.support)
    }

    #[inline]
    fn parameters(&self) -> &Self::Parameters {
        &self.parameters
    }

    fn parameters_size(&self) -> usize {
        self.parameters.len()
    }

    fn condition(&self, e: &Self::Evidence) -> Result<Self> {
        // Check that the evidence support match the potential support.
        if e.support() != self.support() {
            return Err(Error::InvalidParameter(
                "evidence",
                &format!(
                    "Failed to condition on evidence: \n\
                    \t expected:    evidence support to match potential support , \n\
                    \t found:       potential support = {:?} , \n\
                    \t              evidence  support = {:?} .",
                    self.support(),
                    e.support(),
                ),
            ));
        }

        // Get the evidence and remove nones.
        let e = e.evidences().iter().flatten().map(|ev| match ev {
            CatEvT::CertainPositive { event, state } => Ok((event, state)),
            _ => Err(Error::InvalidParameter(
                "evidence",
                &format!(
                    "Failed to condition on evidence: \n\
                    \t expected:    CertainPositive , \n\
                    \t found:       {:?} .",
                    ev
                ),
            )),
        });

        // Get support and parameters.
        let mut support = self.support.clone();
        let mut parameters = self.parameters.clone();

        // Condition in reverse order to avoid axis shifting.
        e.rev().try_for_each(|e| -> Result<_> {
            let (&event, &state) = e?;
            parameters.index_axis_inplace(Axis(event), state);
            support.shift_remove_index(event);
            Ok(())
        })?;

        // Return self.
        Self::new(support, parameters)
    }

    fn marginalize(&self, x: &Set<usize>) -> Result<Self> {
        // Base case: if no variables to marginalize, return self.
        if x.is_empty() {
            return Ok(self.clone());
        }

        // Check X is a subset of the variables.
        x.iter().try_for_each(|&x| {
            if x >= self.labels.len() {
                return Err(Error::IndexOutOfBounds(x));
            }
            Ok(())
        })?;

        // Get the support and the parameters.
        let support = self.support.clone();
        let mut parameters = self.parameters.clone();

        // Filter the support.
        let support = support.into_iter().enumerate();
        let support = support.filter_map(|(i, s)| (!x.contains(&i)).then_some(s));
        let support = support.collect();

        // Sum over the axes in reverse order to avoid shifting.
        x.iter().sorted().rev().for_each(|&i| {
            parameters = parameters.sum_axis(Axis(i));
        });

        // Return the new potential.
        Self::new(support, parameters)
    }

    #[inline]
    fn normalize(&self) -> Result<Self> {
        // Get the parameters.
        let mut parameters = self.parameters.clone();
        // Normalize the parameters.
        parameters /= parameters.sum();
        // Return the new potential.
        Self::new(self.support.clone(), parameters)
    }

    fn from_cpd(cpd: Self::CPD) -> Result<Self> {
        // Merge conditioning support and support in this order.
        let mut support = cpd.conditioning_support().clone();
        support.extend(cpd.support().clone());
        // Get n-dimensional shape.
        let shape: Vec<_> = support.values().map(Set::len).collect();
        // Reshape the parameters to match the new shape.
        let parameters = cpd.parameters().clone();
        let parameters = parameters
            .into_dyn()
            .into_shape_with_order(shape)
            .map_err(Error::NdarrayShape)?;

        // Get the new axes order w.r.t. sorted labels.
        let mut axes: Vec<_> = (0..support.len()).collect();
        axes.sort_by(|&i, &j| {
            support
                .get_index(i)
                .map(|(l, _)| l)
                .cmp(&support.get_index(j).map(|(l, _)| l))
        });
        // Sort the support by labels.
        support.sort_keys();
        // Swap axes to match the new order.
        let parameters = parameters.permuted_axes(axes);

        // Return the new potential.
        Self::new(support, parameters)
    }

    fn into_cpd(self, x: &Set<usize>, z: &Set<usize>) -> Result<Self::CPD> {
        // Check that X and Z are disjoint.
        if !x.is_disjoint(z) {
            return Err(Error::InvalidParameter(
                "x,z",
                "Variables and conditioning variables must be disjoint.",
            ));
        }
        // Check that X and Z cover all variables.
        if !(x | z).iter().sorted().cloned().eq(0..self.labels.len()) {
            return Err(Error::InvalidParameter(
                "x,z",
                "Variables and conditioning variables must cover all potential variables.",
            ));
        }

        // Split support into support and conditioning support.
        let states_x: CatSupport = x
            .iter()
            .map(|&i| {
                self.support
                    .get_index(i)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .ok_or_else(|| Error::IndexOutOfBounds(i))
            })
            .collect::<Result<_>>()?;
        let states_z: CatSupport = z
            .iter()
            .map(|&i| {
                self.support
                    .get_index(i)
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .ok_or_else(|| Error::IndexOutOfBounds(i))
            })
            .collect::<Result<_>>()?;

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
            .map_err(|e| Error::Shape(&format!("Failed to reshape parameters: {}", e)))?;

        // Normalize the parameters.
        parameters /= &parameters.sum_axis(Axis(1)).insert_axis(Axis(1));

        // Create the new CPD.
        CatCPD::new(states_x, states_z, parameters)
    }
}

impl Serialize for CatPhi {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Allocate the map.
        let mut map = serializer.serialize_map(Some(4))?;

        // Serialize support.
        map.serialize_entry("support", &self.support)?;

        // Convert shape to a flat format.
        let shape: Vec<usize> = self.shape.to_vec();
        // Serialize shape.
        map.serialize_entry("shape", &shape)?;

        // Convert parameters to a flat format.
        let parameters: Vec<f64> = self.parameters.iter().cloned().collect();
        // Serialize parameters.
        map.serialize_entry("parameters", &parameters)?;

        // Serialize type.
        map.serialize_entry("type", "catphi")?;

        // Finalize the map serialization.
        map.end()
    }
}

impl<'de> Deserialize<'de> for CatPhi {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Support,
            Shape,
            Parameters,
            Type,
        }

        struct CatPhiVisitor;

        impl<'de> Visitor<'de> for CatPhiVisitor {
            type Value = CatPhi;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CatPhi")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CatPhi, V::Error>
            where
                V: MapAccess<'de>,
            {
                use serde::de::Error as E;

                // Allocate the fields.
                let mut support = None;
                let mut shape = None;
                let mut parameters = None;
                let mut type_ = None;

                // Parse the map.
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Support => {
                            if support.is_some() {
                                return Err(E::duplicate_field("support"));
                            }
                            support = Some(map.next_value()?);
                        }
                        Field::Shape => {
                            if shape.is_some() {
                                return Err(E::duplicate_field("shape"));
                            }
                            shape = Some(map.next_value()?);
                        }
                        Field::Parameters => {
                            if parameters.is_some() {
                                return Err(E::duplicate_field("parameters"));
                            }
                            parameters = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if type_.is_some() {
                                return Err(E::duplicate_field("type"));
                            }
                            type_ = Some(map.next_value()?);
                        }
                    }
                }

                // Check required fields.
                let support = support.ok_or_else(|| E::missing_field("support"))?;
                let shape: Vec<usize> = shape.ok_or_else(|| E::missing_field("shape"))?;
                let parameters: Vec<f64> =
                    parameters.ok_or_else(|| E::missing_field("parameters"))?;

                // Check type is correct.
                let type_: String = type_.ok_or_else(|| E::missing_field("type"))?;
                if type_ != "catphi" {
                    return Err(E::custom(format!(
                        "Invalid type for CatPhi: expected 'catphi', found '{type_}'"
                    )));
                }

                // Convert parameters to ndarray.
                let parameters = ArrayD::from_shape_vec(shape, parameters)
                    .map_err(|e| E::custom(format!("Invalid parameters shape: {e}")))?;

                CatPhi::new(support, parameters).map_err(E::custom)
            }
        }

        const FIELDS: &[&str] = &["support", "shape", "parameters", "type"];

        deserializer.deserialize_struct("CatPhi", FIELDS, CatPhiVisitor)
    }
}

// Implement `JsonIO` for `CatPhi`.
impl_json_io!(CatPhi);

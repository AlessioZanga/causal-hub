use ndarray::prelude::*;
use rayon::prelude::*;

use super::{CategoricalDataMatrix, DataSet, RavelMultiIndex};
use crate::utils::axis_chunks_size;

/// One-dimensional marginal contingency table.
pub struct MarginalCountMatrix {
    n: Array1<usize>,
}

impl MarginalCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &CategoricalDataMatrix, x: usize) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();

        // Set count matrix shape.
        let shape = (cards[x] as usize,);

        // Allocate count matrix.
        let mut n = Array1::zeros(shape);
        // Fill count matrix.
        for row in d.data().rows() {
            // Increment at given index.
            n[row[x] as usize] += 1;
        }

        Self { n }
    }

    /// Get reference to underlying values.
    #[inline]
    pub const fn values(&self) -> &Array1<usize> {
        &self.n
    }
}

impl From<MarginalCountMatrix> for Array1<usize> {
    #[inline]
    fn from(other: MarginalCountMatrix) -> Array1<usize> {
        other.n
    }
}

/// Two-dimensional conditional contingency table.
pub struct ConditionalCountMatrix {
    n: Array2<usize>,
}

impl ConditionalCountMatrix {
    #[inline]
    pub(crate) fn eval(
        shape: (usize, usize),
        rmi: &RavelMultiIndex,
        d: ArrayView2<u8>,
        x: usize,
        z: &[usize],
    ) -> Array2<usize> {
        // Allocate count matrix.
        let mut n = Array2::zeros(shape);
        // Fill count matrix.
        for row in d.rows() {
            // Get multi index.
            let row_z = z.iter().map(|&z| row[z] as usize);
            // Ravel multi index.
            let row_z = rmi.call(row_z);
            // Increment at given index.
            n[[row_z, row[x] as usize]] += 1;
        }

        n
    }

    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &CategoricalDataMatrix, x: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();
        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z] as usize));
        // Set count matrix shape.
        let shape = (rmi.len(), cards[x] as usize);

        // Count the given observations.
        let n = Self::eval(shape, &rmi, d.data().view(), x, z);

        Self { n }
    }

    /// Build new count matrix with given data matrix and indices in parallel.
    #[inline]
    pub fn par_new(d: &CategoricalDataMatrix, x: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();
        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z] as usize));
        // Set count matrix shape.
        let shape = (rmi.len(), cards[x] as usize);

        // Count the given observations in parallel.
        let n = d
            .data()
            .axis_chunks_iter(Axis(0), axis_chunks_size(d.data()))
            .into_par_iter()
            .map(|d| Self::eval(shape, &rmi, d, x, z))
            .reduce(|| Array2::zeros(shape), |acc, x| acc + x);

        Self { n }
    }

    /// Get reference to underlying values.
    #[inline]
    pub const fn values(&self) -> &Array2<usize> {
        &self.n
    }
}

impl From<ConditionalCountMatrix> for Array2<usize> {
    #[inline]
    fn from(other: ConditionalCountMatrix) -> Array2<usize> {
        other.n
    }
}

/// Two-dimensional joint contingency table.
pub struct JointCountMatrix {
    n: Array2<usize>,
}

impl JointCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &CategoricalDataMatrix, x: usize, y: usize) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();

        // Set count matrix shape.
        let shape = (cards[x] as usize, cards[y] as usize);

        // Allocate count matrix.
        let mut n = Array2::zeros(shape);
        // Fill count matrix.
        for row in d.data().rows() {
            // Increment at given index.
            n[[row[x] as usize, row[y] as usize]] += 1;
        }

        Self { n }
    }

    /// Get reference to underlying values.
    #[inline]
    pub const fn values(&self) -> &Array2<usize> {
        &self.n
    }
}

impl From<JointCountMatrix> for Array2<usize> {
    #[inline]
    fn from(other: JointCountMatrix) -> Array2<usize> {
        other.n
    }
}

/// Three-dimensional joint (conditional) contingency table.
pub struct JointConditionalCountMatrix {
    n: Array3<usize>,
}

impl JointConditionalCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &CategoricalDataMatrix, x: usize, y: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();

        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z] as usize));

        // Set count matrix shape.
        let shape = (rmi.len(), cards[x] as usize, cards[y] as usize);

        // Allocate count matrix.
        let mut n = Array3::zeros(shape);
        // Fill count matrix.
        for row in d.data().rows() {
            // Get multi index.
            let row_z = z.iter().map(|&z| row[z] as usize);
            // Ravel multi index.
            let row_z = rmi.call(row_z);
            // Increment at given index.
            n[[row_z, row[x] as usize, row[y] as usize]] += 1;
        }

        Self { n }
    }

    /// Get reference to underlying values.
    #[inline]
    pub const fn values(&self) -> &Array3<usize> {
        &self.n
    }
}

impl From<JointConditionalCountMatrix> for Array3<usize> {
    #[inline]
    fn from(other: JointConditionalCountMatrix) -> Array3<usize> {
        other.n
    }
}

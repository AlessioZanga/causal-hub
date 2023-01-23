use std::ops::Deref;

use ndarray::prelude::*;
use rayon::prelude::*;

use super::{DiscreteDataMatrix, RavelMultiIndex};
use crate::utils::axis_chunks_size;

/// One-dimensional marginal contingency table.
pub struct MarginalCountMatrix {
    n: Array1<usize>,
}

impl MarginalCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &DiscreteDataMatrix, x: usize) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();

        // Set count matrix shape.
        let shape = (cards[x],);

        // Allocate count matrix.
        let mut n = Array1::zeros(shape);
        // Fill count matrix.
        for row in d.rows() {
            // Increment at given index.
            n[row[x]] += 1;
        }

        Self { n }
    }
}

impl Deref for MarginalCountMatrix {
    type Target = Array1<usize>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.n
    }
}

/// Two-dimensional conditional contingency table.
pub struct ConditionalCountMatrix<const PARALLEL: bool> {
    n: Array2<usize>,
}

impl<const PARALLEL: bool> ConditionalCountMatrix<PARALLEL> {
    #[inline]
    pub(crate) fn eval(
        shape: (usize, usize),
        rmi: &RavelMultiIndex,
        d: ArrayView2<usize>,
        x: usize,
        z: &[usize],
    ) -> Array2<usize> {
        // Allocate count matrix.
        let mut n = Array2::zeros(shape);
        // Fill count matrix.
        for row in d.rows() {
            // Get multi index.
            let row_z = z.iter().map(|&z| row[z]);
            // Ravel multi index.
            let row_z = rmi.call(row_z);
            // Increment at given index.
            n[[row_z, row[x]]] += 1;
        }

        n
    }

    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &DiscreteDataMatrix, x: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();
        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z]));
        // Set count matrix shape.
        let shape = (rmi.len(), cards[x]);

        // Check if parallelization is enabled.
        let n = match PARALLEL {
            // Count the given observations in parallel.
            true => d
                .axis_chunks_iter(Axis(0), axis_chunks_size(d))
                .into_par_iter()
                .map(|d| Self::eval(shape, &rmi, d, x, z))
                .reduce(|| Array2::zeros(shape), |acc, x| acc + x),
            // Count the given observations.
            false => Self::eval(shape, &rmi, d.view(), x, z),
        };

        Self { n }
    }
}

impl<const PARALLEL: bool> Deref for ConditionalCountMatrix<PARALLEL> {
    type Target = Array2<usize>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.n
    }
}

/// Three-dimensional joint (conditional) contingency table.
pub struct JointCountMatrix {
    n: Array3<usize>,
}

impl JointCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    #[inline]
    pub fn new(d: &DiscreteDataMatrix, x: usize, y: usize, z: &[usize]) -> Self {
        // Get cardinalities.
        let cards = d.cardinality();

        // Get cardinalities of conditional set.
        let rmi = RavelMultiIndex::new(z.iter().map(|&z| cards[z]));

        // Set count matrix shape.
        let shape = (rmi.len(), cards[x], cards[y]);

        // Allocate count matrix.
        let mut n = Array3::zeros(shape);
        // Fill count matrix.
        for row in d.rows() {
            // Get multi index.
            let row_z = z.iter().map(|&z| row[z]);
            // Ravel multi index.
            let row_z = rmi.call(row_z);
            // Increment at given index.
            n[[row_z, row[x], row[y]]] += 1;
        }

        Self { n }
    }
}

impl Deref for JointCountMatrix {
    type Target = Array3<usize>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.n
    }
}

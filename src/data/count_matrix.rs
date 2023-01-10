use std::ops::Deref;

use ndarray::prelude::*;

use super::{DiscreteDataMatrix, RavelMultiIndex};

/// One-dimensional marginal contingency table.
pub struct MarginalCountMatrix {
    data: Array1<usize>,
}

impl MarginalCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    pub fn new(data_matrix: &DiscreteDataMatrix, x: usize) -> Self {
        // Get cardinalities.
        let cards = data_matrix.cardinality();
        // Set count matrix shape.
        let shape = (cards[x],);
        // Allocate count matrix.
        let mut data: Array1<usize> = ArrayBase::zeros(shape);
        // Fill count matrix.
        for row in data_matrix.rows() {
            // Increment at given index.
            data[row[x]] += 1;
        }

        Self { data }
    }
}

impl Deref for MarginalCountMatrix {
    type Target = Array1<usize>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// Two-dimensional conditional contingency table.
pub struct ConditionalCountMatrix {
    data: Array2<usize>,
}

impl ConditionalCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    pub fn new(data_matrix: &DiscreteDataMatrix, x: usize, z: Vec<usize>) -> Self {
        // Get cardinalities.
        let cards = data_matrix.cardinality();
        // Get cardinalities of conditional set.
        let ravel_multi_index = RavelMultiIndex::new(cards.select(Axis(0), &z));
        // Set count matrix shape.
        let shape = (ravel_multi_index.len(), cards[x]);
        // Allocate count matrix.
        let mut data: Array2<usize> = ArrayBase::zeros(shape);
        // Fill count matrix.
        for row in data_matrix.rows() {
            // Get multi index.
            let row_z = row.select(Axis(0), &z);
            // Ravel multi index.
            let row_z = ravel_multi_index.call(&row_z);
            // Increment at given index.
            data[[row_z, row[x]]] += 1;
        }

        Self { data }
    }
}

impl Deref for ConditionalCountMatrix {
    type Target = Array2<usize>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// Three-dimensional joint (conditional) contingency table.
pub struct JointCountMatrix {
    data: Array3<usize>,
}

impl JointCountMatrix {
    /// Build new count matrix with given data matrix and indices.
    pub fn new(data_matrix: &DiscreteDataMatrix, x: usize, y: usize, z: Vec<usize>) -> Self {
        // Get cardinalities.
        let cards = data_matrix.cardinality();
        // Get cardinalities of conditional set.
        let ravel_multi_index = RavelMultiIndex::new(cards.select(Axis(0), &z));
        // Set count matrix shape.
        let shape = (ravel_multi_index.len(), cards[x], cards[y]);
        // Allocate count matrix.
        let mut data: Array3<usize> = ArrayBase::zeros(shape);
        // Fill count matrix.
        for row in data_matrix.rows() {
            // Get multi index.
            let row_z = row.select(Axis(0), &z);
            // Ravel multi index.
            let row_z = ravel_multi_index.call(&row_z);
            // Increment at given index.
            data[[row_z, row[x], row[y]]] += 1;
        }

        Self { data }
    }
}

impl Deref for JointCountMatrix {
    type Target = Array3<usize>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

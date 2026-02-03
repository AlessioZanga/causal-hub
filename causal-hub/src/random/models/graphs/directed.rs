use itertools::Itertools;
use rand::{Rng, seq::SliceRandom};

use crate::{
    models::{DiGraph, Graph},
    random::Random,
    types::{Error, Labels, Result},
};

/// A struct representing a random directed graph generator.
pub struct RngDiGraph<'a, R> {
    rng: &'a mut R,
    labels: &'a Labels,
    p: f64,
}

impl<'a, R> RngDiGraph<'a, R> {
    /// Creates a new `RngDiGraph` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `labels` - The labels of the graph.
    /// * `p` - The probability of generating an edge.
    ///
    /// # Returns
    ///
    /// A new `RngDiGraph` instance.
    ///
    pub fn new(rng: &'a mut R, labels: &'a Labels, p: f64) -> Result<Self> {
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter(
                "p".to_string(),
                "must be in [0, 1]".to_string(),
            ));
        }

        Ok(Self { rng, labels, p })
    }
}

impl<R: Rng> Random for RngDiGraph<'_, R> {
    type Output = Result<DiGraph>;

    fn random(&mut self) -> Self::Output {
        // Construct the empty graph.
        let mut g = DiGraph::empty(self.labels)?;

        // Get the number of vertices.
        let n = self.labels.len();

        // For each pair of vertices ...
        (0..n)
            .cartesian_product(0..n)
            // ... filter out self-loops and sample edges ...
            .filter(|(i, j)| i != j && self.rng.random_bool(self.p))
            .try_for_each(|(i, j)| -> Result<()> {
                // ... add an edge.
                g.add_edge(i, j)?;

                Ok(())
            })?;

        Ok(g)
    }
}

/// A struct representing a random directed acyclic graph generator.
pub struct RngDag<'a, R> {
    rng: &'a mut R,
    labels: &'a Labels,
    p: f64,
}

impl<'a, R> RngDag<'a, R> {
    /// Creates a new `RngDag` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `labels` - The labels of the graph.
    /// * `p` - The probability of generating an edge.
    ///
    /// # Returns
    ///
    /// A new `RngDag` instance.
    ///
    pub fn new(rng: &'a mut R, labels: &'a Labels, p: f64) -> Result<Self> {
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter(
                "p".to_string(),
                "must be in [0, 1]".to_string(),
            ));
        }

        Ok(Self { rng, labels, p })
    }
}

impl<R: Rng> Random for RngDag<'_, R> {
    type Output = Result<DiGraph>;

    fn random(&mut self) -> Self::Output {
        // Construct the empty graph.
        let mut g = DiGraph::empty(self.labels)?;

        // Get the number of vertices.
        let n = self.labels.len();

        // Generate a random topological order.
        let mut order: Vec<_> = (0..n).collect();
        order.shuffle(self.rng);

        // For each pair of vertices in the topological order ...
        order
            .into_iter()
            .combinations(2)
            // ... sample edges ...
            .filter(|_| self.rng.random_bool(self.p))
            .try_for_each(|idx| -> Result<()> {
                // ... add an edge.
                let (i, j) = (idx[0], idx[1]);
                g.add_edge(i, j)?;

                Ok(())
            })?;

        Ok(g)
    }
}

use itertools::Itertools;
use rand::{Rng, RngExt};

use crate::{
    models::{Graph, UnGraph},
    random::Random,
    types::{Error, Labels, Result},
};

/// A struct representing a random undirected graph generator.
pub struct RngUnGraph<'a, R> {
    rng: &'a mut R,
    labels: &'a Labels,
    p: f64,
}

impl<'a, R> RngUnGraph<'a, R> {
    /// Creates a new `RngUnGraph` instance.
    ///
    /// # Arguments
    ///
    /// * `rng` - A mutable reference to a random number generator.
    /// * `labels` - The labels of the graph.
    /// * `p` - The probability of generating an edge.
    ///
    /// # Returns
    ///
    /// A new `RngUnGraph` instance.
    ///
    pub fn new(rng: &'a mut R, labels: &'a Labels, p: f64) -> Result<Self> {
        // Check if the probability is in [0, 1].
        if !(0.0..=1.0).contains(&p) {
            return Err(Error::InvalidParameter("p", "must be in [0, 1]"));
        }

        Ok(Self { rng, labels, p })
    }
}

impl<R: Rng> Random for RngUnGraph<'_, R> {
    type Output = Result<UnGraph>;

    fn random(&mut self) -> Self::Output {
        // Construct the empty graph.
        let mut g = UnGraph::empty(self.labels)?;

        // Get the number of vertices.
        let n = self.labels.len();

        // For each pair of vertices ...
        (0..n)
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

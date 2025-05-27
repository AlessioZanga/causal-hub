use itertools::Itertools;

use crate::{
    distributions::{CPD, CatCIM},
    estimators::CPDEstimator,
    graphs::{DiGraph, Graph},
};

/// A trait for scoring criteria used in score-based structure learning.
pub trait ScoringCriterion {
    /// Computes the score for a given variable and its conditioning set.
    ///
    /// # Arguments
    ///
    /// * `x` - The variable to score.
    /// * `z` - The conditioning set.
    ///
    /// # Returns
    ///
    /// The computed score.
    ///
    fn call(&self, x: usize, z: &[usize]) -> f64;
}

/// A type alias for a scoring criterion.
pub use ScoringCriterion as SC;

/// The Bayesian Information Criterion (BIC).
pub struct BayesianInformationCriterion<'a, E> {
    estimator: &'a E,
}

/// A type alias for the BIC.
pub type BIC<'a, E> = BayesianInformationCriterion<'a, E>;

impl<'a, E> BIC<'a, E> {
    /// Creates a new BIC instance.
    ///
    /// # Arguments
    ///
    /// * `estimator` - A reference to the estimator.
    ///
    /// # Returns
    ///
    /// A new `BIC` instance.
    ///
    #[inline]
    pub const fn new(estimator: &'a E) -> Self {
        Self { estimator }
    }
}

impl<E> SC for BIC<'_, E>
where
    E: CPDEstimator<CatCIM>,
{
    #[inline]
    fn call(&self, x: usize, z: &[usize]) -> f64 {
        // Compute the intensity matrices for the sets.
        let q_xz = self.estimator.fit(x, &z);
        // Get the sample size.
        let n = q_xz.sample_size().expect("Failed to get the sample size.");
        // Get the log-likelihood.
        let ll = q_xz
            .sample_log_likelihood()
            .expect("Failed to compute the log-likelihood.");
        // Get the number of parameters.
        let k = q_xz.parameters_size() as f64;

        // Compute the BIC.
        ll - 0.5 * k * f64::ln(n)
    }
}

/// The hill climbing algorithm for structure learning in CTBNs.
#[derive(Clone, Debug)]
pub struct ContinuousTimeHillClimbing<'a, S> {
    initial_graph: &'a DiGraph,
    score: &'a S,
}

/// A type alias for the continuous time hill climbing algorithm.
pub type CTHC<'a, S> = ContinuousTimeHillClimbing<'a, S>;

impl<'a, S> CTHC<'a, S>
where
    S: SC,
{
    /// Creates a new continuous time hill climbing instance.
    ///
    /// # Arguments
    ///
    /// * `initial_graph` - The initial directed graph.
    /// * `score` - The scoring criterion to use.
    ///
    /// # Returns
    ///
    /// A new `ContinuousTimeHillClimbing` instance.
    ///
    #[inline]
    pub const fn new(initial_graph: &'a DiGraph, score: &'a S) -> Self {
        // FIXME: Check initial graph and score have the same labels.

        Self {
            initial_graph,
            score,
        }
    }

    /// Execute the CTHC algorithm.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn fit(&self) -> DiGraph {
        // Clone the initial graph.
        let mut graph = DiGraph::empty(self.initial_graph.labels());

        // For each vertex in the graph ...
        for i in self.initial_graph.vertices() {
            // Initialize the previous score to negative infinity.
            let mut prev_score = f64::NEG_INFINITY;

            // Set the initial parent set as the current parent set.
            let mut curr_pa = self.initial_graph.parents(i);
            // Compute the score of the current parent set.
            let mut curr_score = self.score.call(i, &curr_pa);

            // While the score of the current parent set is higher than the previous score ...
            while prev_score < curr_score {
                // Set the previous score to the score of the current parent set.
                prev_score = curr_score;

                // Get the candidate parent sets by adding ...
                let poss_pa = {
                    // Clone the current parent set.
                    [curr_pa.clone()].into_iter().flat_map(|curr_pa| {
                        // Get the vertices that are not in the current parent set.
                        self.initial_graph
                            .vertices()
                            .into_iter()
                            .filter_map(move |j| {
                                if i != j {
                                    // If the vertex is not in the current parent set ...
                                    if let Err(p_j) = curr_pa.binary_search(&j) {
                                        // Clone the current parent set.
                                        let mut curr_pa = curr_pa.clone();
                                        // Insert the vertex in order.
                                        curr_pa.insert(p_j, j);
                                        // Return it as a candidate for addition.
                                        return Some(curr_pa);
                                    }
                                }
                                // Otherwise, the vertex is already present.
                                None
                            })
                    })
                }
                // ... or removing vertices.
                .chain({
                    // Clone the current parent set.
                    let curr_pa = curr_pa.clone();
                    // Get the size of the candidate subset, avoid underflow.
                    let k = curr_pa.len().saturating_sub(1);
                    // Generate all the k-sized subsets.
                    curr_pa.into_iter().combinations(k)
                });

                // For each candidate parent sets ...
                for next_pa in poss_pa {
                    // Compute the score of the candidate parent set.
                    let next_score = self.score.call(i, &next_pa);
                    // If the score of the candidate parent set is higher ...
                    if curr_score < next_score {
                        // Update the current parent set to the candidate parent set.
                        curr_pa = next_pa;
                        // Update the score of the current parent set.
                        curr_score = next_score;
                    }
                }
            }

            // Set the current parent set.
            for j in curr_pa {
                // Add an edge from vertex `j` to vertex `i`.
                graph.add_edge(j, i);
            }
        }

        // Return the final graph.
        graph
    }
}

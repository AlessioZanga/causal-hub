use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    estimators::{CIMEstimator, PK},
    models::{CIM, CatCIM, DiGraph, Graph, Labelled},
    set,
    types::{Error, Labels, Result, Set},
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
    fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64>;
}

/// The Bayesian Information Criterion (BIC).
pub struct BIC<'a, E> {
    estimator: &'a E,
}

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

impl<'a, E> Labelled for BIC<'a, E>
where
    E: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.estimator.labels()
    }
}

impl<E> ScoringCriterion for BIC<'_, E>
where
    E: CIMEstimator<CatCIM>,
{
    #[inline]
    fn call(&self, x: &Set<usize>, z: &Set<usize>) -> Result<f64> {
        // Compute the intensity matrices for the sets.
        let q_xz = self.estimator.fit(x, z)?;
        // Get the sample size.
        let n = q_xz
            .sample_statistics()
            .map(|s| s.sample_size())
            .ok_or(Error::MissingSufficientStatistics)?;
        // Get the log-likelihood.
        let ll = q_xz
            .sample_log_likelihood()
            .ok_or_else(|| Error::Probability("Failed to compute the log-likelihood.".into()))?;
        // Get the number of parameters.
        let k = q_xz.parameters_size() as f64;

        // Compute the BIC.
        Ok(ll - 0.5 * k * f64::ln(n))
    }
}

/// The hill climbing algorithm for structure learning in CTBNs.
#[derive(Clone, Debug)]
pub struct CTHC<'a, S> {
    initial_graph: &'a DiGraph,
    score: &'a S,
    max_parents: Option<usize>,
    prior_knowledge: Option<&'a PK>,
}

impl<'a, S> CTHC<'a, S>
where
    S: ScoringCriterion + Labelled,
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
    pub fn new(initial_graph: &'a DiGraph, score: &'a S) -> Result<Self> {
        // Check labels of the initial graph and the estimator are the same.
        if initial_graph.labels() != score.labels() {
            return Err(Error::LabelMismatch(
                format!("{:?}", initial_graph.labels()),
                format!("{:?}", score.labels()),
            ));
        }

        Ok(Self {
            initial_graph,
            score,
            max_parents: None,
            prior_knowledge: None,
        })
    }

    /// Sets the maximum number of parents for each vertex.
    ///
    /// # Arguments
    ///
    /// * `max_parents` - The maximum number of parents for each vertex.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance.
    ///
    #[inline]
    pub const fn with_max_parents(mut self, max_parents: usize) -> Self {
        self.max_parents = Some(max_parents);
        self
    }

    /// Sets the prior knowledge for the algorithm.
    ///
    /// # Arguments
    ///
    /// * `prior_knowledge` - The prior knowledge to use.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance.
    ///
    #[inline]
    pub fn with_prior_knowledge(mut self, prior_knowledge: &'a PK) -> Result<Self> {
        // Check labels of prior knowledge and initial graph are the same.
        if self.initial_graph.labels() != prior_knowledge.labels() {
            return Err(Error::LabelMismatch(
                format!("{:?}", self.initial_graph.labels()),
                format!("{:?}", prior_knowledge.labels()),
            ));
        }
        // Check prior knowledge is consistent with initial graph.
        for edge in self.initial_graph.vertices().into_iter().permutations(2) {
            // Get the edge indices.
            let (i, j) = (edge[0], edge[1]);
            // Check edge must be either present and not forbidden ...
            if self.initial_graph.has_edge(i, j)? {
                if prior_knowledge.is_forbidden(i, j) {
                    return Err(Error::PriorKnowledgeConflict(format!(
                        "Initial graph contains forbidden edge ({i}, {j})."
                    )));
                }
            // ... or absent and not required.
            } else if prior_knowledge.is_required(i, j) {
                return Err(Error::PriorKnowledgeConflict(format!(
                    "Initial graph does not contain required edge ({i}, {j})."
                )));
            }
        }
        // Set prior knowledge.
        self.prior_knowledge = Some(prior_knowledge);
        Ok(self)
    }

    /// Execute the CTHC algorithm.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn fit(&self) -> Result<DiGraph> {
        // Clone the initial graph.
        let mut graph = DiGraph::empty(self.initial_graph.labels())?;

        // For each vertex in the graph ...
        for i in self.initial_graph.vertices() {
            // Initialize the previous score to negative infinity.
            let mut prev_score = f64::NEG_INFINITY;

            // Set the initial parent set as the current parent set.
            let mut curr_pa = self.initial_graph.parents(&set![i])?;
            // Compute the score of the current parent set.
            let mut curr_score = self.score.call(&set![i], &curr_pa)?;

            // While the score of the current parent set is higher than the previous score ...
            while prev_score < curr_score {
                // Set the previous score to the score of the current parent set.
                prev_score = curr_score;

                // Get the candidate parent sets by adding ...
                let poss_pa = {
                    // Clone the current parent set.
                    [curr_pa.clone()].into_iter().filter(|curr_pa|
                        // Check if maximum parents has been reached.
                        if let Some(max_parents) = self.max_parents {
                            curr_pa.len() < max_parents
                        } else {
                            true
                        }
                    ).flat_map(|curr_pa| {
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
                                        curr_pa.shift_insert(p_j, j);
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
                    curr_pa.into_iter().combinations(k).map(Set::from_iter)
                });

                // For each candidate parent sets ...
                for next_pa in poss_pa {
                    // Compute the score of the candidate parent set.
                    let next_score = self.score.call(&set![i], &next_pa)?;
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
                graph.add_edge(j, i)?;
            }
        }

        // Return the final graph.
        Ok(graph)
    }
}

impl<'a, S> CTHC<'a, S>
where
    S: ScoringCriterion + Sync,
{
    /// Execute the CTHC algorithm in parallel.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn par_fit(&self) -> Result<DiGraph> {
        // For each vertex in the graph ...
        let parents: Vec<_> = self
            .initial_graph
            .vertices()
            .into_par_iter()
            .map(|i| {
                // Initialize the previous score to negative infinity.
                let mut prev_score = f64::NEG_INFINITY;

                // Set the initial parent set as the current parent set.
                let mut curr_pa = self.initial_graph.parents(&set![i])?;
                // Compute the score of the current parent set.
                let mut curr_score = self.score.call(&set![i], &curr_pa)?;

                // While the score of the current parent set is higher than the previous score ...
                while prev_score < curr_score {
                    // Set the previous score to the score of the current parent set.
                    prev_score = curr_score;

                    // Get the candidate parent sets by adding ...
                    let poss_pa: Vec<_> = {
                        // Clone the current parent set.
                        [curr_pa.clone()].into_iter().filter(|curr_pa|
                            // Check if maximum parents has been reached.
                            if let Some(max_parents) = self.max_parents {
                                curr_pa.len() < max_parents
                            } else {
                                true
                            }
                        ).flat_map(|curr_pa| {
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
                                            curr_pa.shift_insert(p_j, j);
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
                        curr_pa.into_iter().combinations(k).map(Set::from_iter)
                    })
                    // Collect to allow for parallel iteration.
                    .collect();

                    // For each candidate parent sets ...
                    let scores = poss_pa
                        .into_par_iter()
                        // Compute the score of the candidate parent set in parallel.
                        .map(|next_pa| self.score.call(&set![i], &next_pa).map(|s| (s, next_pa)))
                        .collect::<Result<Vec<_>>>()?;

                    if scores.iter().any(|(s, _)| s.is_nan()) {
                        return Err(Error::NanValue);
                    }

                    if let Some((next_score, next_pa)) = scores
                        .into_iter()
                        // Get the one with the highest score in parallel.
                        .max_by(|(a, _), (b, _)| {
                            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                        })
                    {
                        // If the score of the candidate parent set is higher ...
                        if curr_score < next_score {
                            // Update the current parent set to the candidate parent set.
                            curr_pa = next_pa;
                            // Update the score of the current parent set.
                            curr_score = next_score;
                        }
                    }
                }

                // Return the current parent set.
                Ok(curr_pa)
            })
            .collect::<Result<_>>()?;

        // Clone the initial graph.
        let mut graph = DiGraph::empty(self.initial_graph.labels())?;

        // Set the current parent set.
        for (i, curr_pa) in parents.into_iter().enumerate() {
            for j in curr_pa {
                // Add an edge from vertex `j` to vertex `i`.
                graph.add_edge(j, i)?;
            }
        }

        // Return the final graph.
        Ok(graph)
    }
}

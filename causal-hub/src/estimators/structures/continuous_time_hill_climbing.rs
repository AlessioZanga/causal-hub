use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    estimators::{CTBNEstimator, HasEstimator, PK, ParCTBNEstimator, ScoringCriterion},
    models::{DiGraph, Graph, HasLabels},
    set,
    types::{Error, Result, Set},
};

/// The hill climbing algorithm for structure learning in CTBNs.
#[derive(Clone, Debug)]
pub struct CTHC<'a, S> {
    score: &'a S,
    initial_graph: Option<&'a DiGraph>,
    max_parents: Option<usize>,
    prior_knowledge: Option<&'a PK>,
}

impl<'a, S> CTHC<'a, S>
where
    S: ScoringCriterion + HasLabels,
{
    /// Creates a new continuous time hill climbing instance.
    ///
    /// # Arguments
    ///
    /// * `score` - The scoring criterion to use.
    ///
    /// # Returns
    ///
    /// A new `ContinuousTimeHillClimbing` instance.
    ///
    /// # Notes
    ///
    /// By default, the search starts from an empty graph over the labels of the
    /// scoring criterion. Use [`CTHC::with_initial_graph`] to provide a different
    /// starting point.
    ///
    #[inline]
    pub fn new(score: &'a S) -> Self {
        Self {
            initial_graph: None,
            score,
            max_parents: None,
            prior_knowledge: None,
        }
    }

    /// Sets the initial directed graph.
    ///
    /// # Arguments
    ///
    /// * `initial_graph` - The initial directed graph.
    ///
    /// # Errors
    ///
    /// * If the labels of the initial graph and the scoring criterion do not match.
    ///
    /// # Returns
    ///
    /// The modified instance.
    ///
    #[inline]
    pub fn with_initial_graph(mut self, initial_graph: &'a DiGraph) -> Result<Self> {
        // Check labels of the initial graph and the scoring criterion are the same.
        if initial_graph.labels() != self.score.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", initial_graph.labels()),
                &format!("{:?}", self.score.labels()),
            ));
        }
        // Set the initial graph.
        self.initial_graph = Some(initial_graph);

        Ok(self)
    }

    /// Sets the maximum number of parents for each vertex.
    ///
    /// # Arguments
    ///
    /// * `max_parents` - The maximum number of parents for each vertex.
    ///
    /// # Returns
    ///
    /// The modified instance.
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
    /// The modified instance.
    ///
    #[inline]
    pub fn with_prior_knowledge(mut self, prior_knowledge: &'a PK) -> Result<Self> {
        // Get the initial graph, or an empty graph over the labels of the scoring criterion.
        let initial_graph = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::empty(self.score.labels())?,
        };
        // Check labels of prior knowledge and initial graph are the same.
        if initial_graph.labels() != prior_knowledge.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", initial_graph.labels()),
                &format!("{:?}", prior_knowledge.labels()),
            ));
        }
        // Check prior knowledge is consistent with initial graph.
        for edge in initial_graph.vertices().into_iter().permutations(2) {
            // Get the edge indices.
            let (i, j) = (edge[0], edge[1]);
            // Check edge must be either present and not forbidden ...
            if initial_graph.has_edge(i, j)? {
                if prior_knowledge.is_forbidden(i, j) {
                    return Err(Error::PriorKnowledgeConflict(&format!(
                        "Initial graph contains forbidden edge ({i}, {j})."
                    )));
                }
            // ... or absent and not required.
            } else if prior_knowledge.is_required(i, j) {
                return Err(Error::PriorKnowledgeConflict(&format!(
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
    /// # Errors
    ///
    /// * If the scoring criterion fails.
    ///
    /// # Returns
    ///
    /// The fitted model over the learned structure.
    ///
    pub fn fit<M>(&self) -> Result<M>
    where
        S: HasEstimator,
        S::Estimator: CTBNEstimator<M>,
    {
        // Get the initial graph, or an empty graph over the labels of the scoring criterion.
        let initial_graph = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::empty(self.score.labels())?,
        };
        // Initialize the output graph.
        let mut graph = DiGraph::empty(initial_graph.labels())?;

        // For each vertex in the graph ...
        for i in initial_graph.vertices() {
            // Initialize the previous score to negative infinity.
            let mut prev_score = f64::NEG_INFINITY;

            // Set the initial parent set as the current parent set.
            let mut curr_pa = initial_graph.parents(&set![i])?;
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
                        initial_graph
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

        // Fit the model over the learned structure.
        self.score.estimator().fit(graph)
    }
}

impl<'a, S> CTHC<'a, S>
where
    S: ScoringCriterion + HasLabels + Sync,
{
    /// Execute the CTHC algorithm in parallel.
    ///
    /// # Errors
    ///
    /// * If the scoring criterion fails.
    ///
    /// # Returns
    ///
    /// The fitted model over the learned structure.
    ///
    pub fn par_fit<M>(&self) -> Result<M>
    where
        S: HasEstimator,
        S::Estimator: ParCTBNEstimator<M>,
    {
        // Get the initial graph, or an empty graph over the labels of the scoring criterion.
        let initial_graph = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::empty(self.score.labels())?,
        };

        // For each vertex in the graph ...
        let parents: Vec<_> = initial_graph
            .vertices()
            .into_par_iter()
            .map(|i| {
                // Initialize the previous score to negative infinity.
                let mut prev_score = f64::NEG_INFINITY;

                // Set the initial parent set as the current parent set.
                let mut curr_pa = initial_graph.parents(&set![i])?;
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
                            initial_graph
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
                        .map(|next_pa| {
                            self.score
                                .call(&set![i], &next_pa)
                                .map(|stats| (stats, next_pa))
                        })
                        .collect::<Result<Vec<_>>>()?;

                    if scores.iter().any(|(stats, _)| stats.is_nan()) {
                        return Err(Error::NanValue());
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

        // Initialize the output graph.
        let mut graph = DiGraph::empty(initial_graph.labels())?;

        // Set the current parent set.
        for (i, curr_pa) in parents.into_iter().enumerate() {
            for j in curr_pa {
                // Add an edge from vertex `j` to vertex `i`.
                graph.add_edge(j, i)?;
            }
        }

        // Fit the model over the learned structure.
        self.score.estimator().par_fit(graph)
    }
}

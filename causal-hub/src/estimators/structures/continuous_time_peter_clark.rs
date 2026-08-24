use itertools::Itertools;
use log::debug;
use rayon::prelude::*;

use crate::{
    estimators::{CITest, CTBNEstimator, HasEstimator, PK, ParCTBNEstimator},
    models::{DiGraph, Graph, HasLabels},
    set,
    types::{Error, Result, Set},
};

/// A struct representing a continuous-time Peter-Clark estimator.
#[derive(Clone, Debug)]
pub struct CTPC<'a, T, S> {
    initial_graph: Option<&'a DiGraph>,
    null_time: &'a T,
    null_state: &'a S,
    prior_knowledge: Option<&'a PK>,
}

impl<'a, T, S> CTPC<'a, T, S>
where
    T: CITest + HasLabels,
    S: CITest + HasLabels,
{
    /// Creates a new `CTPC` instance.
    ///
    /// # Arguments
    ///
    /// * `null_time` - A reference to the null time to transition hypothesis test.
    /// * `null_state` - A reference to the null state-to-state transition hypothesis test.
    ///
    /// # Errors
    ///
    /// * If the labels of the two hypothesis tests do not match.
    ///
    /// # Returns
    ///
    /// A new `CTPC` instance.
    ///
    /// # Notes
    ///
    /// By default, the algorithm starts from a complete graph over the labels of
    /// the hypothesis tests. Use [`CTPC::with_initial_graph`] to provide a different
    /// starting point.
    ///
    #[inline]
    pub fn new(null_time: &'a T, null_state: &'a S) -> Result<Self> {
        // Check labels of the two hypothesis tests are the same.
        if null_time.labels() != null_state.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", null_time.labels()),
                &format!("{:?}", null_state.labels()),
            ));
        }

        Ok(Self {
            initial_graph: None,
            null_time,
            null_state,
            prior_knowledge: None,
        })
    }

    /// Sets the initial directed graph.
    ///
    /// # Arguments
    ///
    /// * `initial_graph` - A reference to the initial graph.
    ///
    /// # Errors
    ///
    /// * If the labels of the initial graph and the hypothesis tests do not match.
    ///
    /// # Returns
    ///
    /// The modified instance.
    ///
    #[inline]
    pub fn with_initial_graph(mut self, initial_graph: &'a DiGraph) -> Result<Self> {
        // Check labels of the initial graph and the time-to-transition test are the same.
        if initial_graph.labels() != self.null_time.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", initial_graph.labels()),
                &format!("{:?}", self.null_time.labels()),
            ));
        }
        // Check labels of the initial graph and the state-to-state transition test are the same.
        if initial_graph.labels() != self.null_state.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", initial_graph.labels()),
                &format!("{:?}", self.null_state.labels()),
            ));
        }
        // Set the initial graph.
        self.initial_graph = Some(initial_graph);

        Ok(self)
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
        // Get the initial graph, or a complete graph over the labels of the hypothesis tests.
        let initial_graph = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::complete(self.null_time.labels())?,
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
            }
            // ... or absent and not required.
            else if prior_knowledge.is_required(i, j) {
                return Err(Error::PriorKnowledgeConflict(&format!(
                    "Initial graph does not contain required edge ({i}, {j})."
                )));
            }
        }
        // Set prior knowledge.
        self.prior_knowledge = Some(prior_knowledge);
        Ok(self)
    }

    /// Execute the CTPC algorithm.
    ///
    /// # Errors
    ///
    /// * If a conditional independence test fails.
    ///
    /// # Returns
    ///
    /// The fitted model over the learned structure.
    ///
    /// # Notes
    ///
    /// The model parameters are estimated using the estimator wrapped by
    /// the null time-to-transition hypothesis test.
    ///
    pub fn fit<M>(&self) -> Result<M>
    where
        T: HasEstimator,
        T::Estimator: CTBNEstimator<M>,
    {
        // Get the initial graph, or a complete graph over the labels of the hypothesis tests.
        let mut graph = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::complete(self.null_time.labels())?,
        };

        // For each vertex in the graph ...
        for i in graph.vertices() {
            // Get the parents of the vertex.
            let mut pa_i = graph.parents(&set![i])?;

            // Initialize the counter.
            let mut k = 0;

            // While the counter is smaller than the number of parents ...
            while k < pa_i.len() {
                // Initialize the set of vertices to remove, to ensure stability.

                // For each parent, check if it is independent of the child given a subset of size k.
                let not_pa_i: Vec<_> = pa_i
                    .iter()
                    .filter_map(|&j| {
                        // Check prior knowledge, if available.
                        if let Some(prior_knowledge) = self.prior_knowledge {
                            // If the edge is required, skip the tests.
                            // NOTE: Since CTPC only removes edges,
                            //  it is sufficient to check for required edges.
                            if prior_knowledge.is_required(j, i) {
                                // Log the skipped CIT.
                                debug!("CIT for {j} _||_ {i} | [*] ... SKIPPED");
                                return None;
                            }
                        }

                        // Filter out the parent.
                        let pa_i_not_j = pa_i.iter().filter(|&&z| z != j).cloned();
                        // For any combination of size k of Pa(X_i) \ { X_j } ...
                        pa_i_not_j
                            .combinations(k)
                            .map(Set::from_iter)
                            .find_map(|s_ij| {
                                // Log the current combination.
                                debug!("CIT for {i} _||_ {j} | {s_ij:?} ...");
                                // If X_i _||_ X_j | S_{X_i, X_j} ...
                                match self.null_time.call(&set![i], &set![j], &s_ij) {
                                    Ok(true) => {
                                        match self.null_state.call(&set![i], &set![j], &s_ij) {
                                            Ok(true) => {
                                                // Log the result of the CIT.
                                                debug!(
                                                    "CIT for {i} _||_ {j} | {s_ij:?} ... PASSED"
                                                );
                                                Some(Ok(j))
                                            }
                                            Ok(false) => None,
                                            Err(evidence) => Some(Err(evidence)),
                                        }
                                    }
                                    Ok(false) => None,
                                    Err(evidence) => Some(Err(evidence)),
                                }
                            })
                    })
                    .collect::<Result<_>>()?;

                // Remove the vertices from the graph.
                for &j in &not_pa_i {
                    // Remove the vertex from the parents.
                    pa_i.retain(|&x| x != j);
                    // Remove the edge from the graph.
                    graph.del_edge(j, i)?;
                }

                // Increment the counter.
                k += 1;
            }
        }

        // Fit the model over the learned structure.
        self.null_time.estimator().fit(graph)
    }
}

impl<'a, T, S> CTPC<'a, T, S>
where
    T: CITest + HasLabels + Sync,
    S: CITest + HasLabels + Sync,
{
    /// Execute the CTPC algorithm in parallel.
    ///
    /// # Errors
    ///
    /// * If a conditional independence test fails.
    ///
    /// # Returns
    ///
    /// The fitted model over the learned structure.
    ///
    /// # Notes
    ///
    /// The model parameters are estimated using the estimator wrapped by
    /// the null time-to-transition hypothesis test.
    ///
    pub fn par_fit<M>(&self) -> Result<M>
    where
        T: HasEstimator,
        T::Estimator: ParCTBNEstimator<M>,
    {
        // Get the initial graph, or a complete graph over the labels of the hypothesis tests.
        let initial_graph = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::complete(self.null_time.labels())?,
        };

        // For each vertex in the graph ...
        let parents: Vec<_> = initial_graph
            .vertices()
            .into_par_iter()
            .map(|i| -> Result<Set<usize>> {
                // Get the parents of the vertex.
                let mut pa_i = initial_graph.parents(&set![i])?;

                // Initialize the counter.
                let mut k = 0;

                // While the counter is smaller than the number of parents ...
                while k < pa_i.len() {
                    // Filter the parents in parallel.
                    pa_i = pa_i
                        .par_iter()
                        .map(|&j| -> Result<Option<usize>> {
                            // Check prior knowledge, if available.
                            if let Some(prior_knowledge) = self.prior_knowledge {
                                // If the edge is required, skip the tests.
                                // NOTE: Since CTPC only removes edges,
                                //  it is sufficient to check for required edges.
                                if prior_knowledge.is_required(j, i) {
                                    // Log the skipped CIT.
                                    debug!("CIT for {j} _||_ {i} | [*] ... SKIPPED");
                                    return Ok(Some(j));
                                }
                            }
                            // Filter out the parent.
                            let pa_i_not_j = pa_i.iter().filter(|&&z| z != j).cloned();
                            // For any combination of size k of Pa(X_i) \ { X_j } ...
                            for s_ij in pa_i_not_j.combinations(k).map(Set::from_iter) {
                                // Log the current combination.
                                debug!("CIT for {i} _||_ {j} | {s_ij:?} ...");
                                // If X_i _||_ X_j | S_{X_i, X_j} ...
                                if self.null_time.call(&set![i], &set![j], &s_ij)?
                                    && self.null_state.call(&set![i], &set![j], &s_ij)?
                                {
                                    // Log the result of the CIT.
                                    debug!("CIT for {i} _||_ {j} | {s_ij:?} ... PASSED");
                                    // Add the parent to the set of vertices to remove.
                                    return Ok(None);
                                }
                            }
                            // Otherwise, keep the parent.
                            Ok(Some(j))
                        })
                        .filter_map(|x| x.transpose())
                        .collect::<Result<_>>()?;
                    // Increment the counter.
                    k += 1;
                }

                // Return the parents of the vertex.
                Ok(pa_i)
            })
            .collect::<Result<_>>()?;

        // Initialize an empty graph.
        let mut graph = DiGraph::empty(initial_graph.labels())?;

        // Set the parents of each vertex.
        parents.into_iter().enumerate().try_for_each(|(i, pa_i)| {
            // For each parent ...
            pa_i.into_iter().try_for_each(|j| -> Result<_> {
                graph.add_edge(j, i)?;
                Ok(())
            })
        })?;

        // Fit the model over the learned structure.
        self.null_time.estimator().par_fit(graph)
    }
}

use itertools::Itertools;
use log::debug;
use rayon::prelude::*;

use crate::{
    estimators::{CITest, PK},
    models::{DiGraph, Graph, Labelled},
    set,
    types::{Error, Result, Set},
};

/// A struct representing a continuous-time Peter-Clark estimator.
#[derive(Clone, Debug)]
pub struct CTPC<'a, T, S> {
    initial_graph: &'a DiGraph,
    null_time: &'a T,
    null_state: &'a S,
    prior_knowledge: Option<&'a PK>,
}

impl<'a, T, S> CTPC<'a, T, S>
where
    T: CITest + Labelled,
    S: CITest + Labelled,
{
    /// Creates a new `CTPC` instance.
    ///
    /// # Arguments
    ///
    /// * `initial_graph` - A reference to the initial graph.
    /// * `null_time` - A reference to the null time to transition hypothesis test.
    /// * `null_state` - A reference to the null state-to-state transition hypothesis test.
    ///
    /// # Returns
    ///
    /// A new `CTPC` instance.
    ///
    #[inline]
    pub fn new(initial_graph: &'a DiGraph, null_time: &'a T, null_state: &'a S) -> Result<Self> {
        // Check labels of the initial graph and the estimator are the same.
        if initial_graph.labels() != null_time.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", initial_graph.labels()),
                &format!("{:?}", null_time.labels()),
            ));
        }
        // Check labels of the initial graph and the estimator are the same.
        if initial_graph.labels() != null_state.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", initial_graph.labels()),
                &format!("{:?}", null_state.labels()),
            ));
        }

        Ok(Self {
            initial_graph,
            null_time,
            null_state,
            prior_knowledge: None,
        })
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
        // Check labels of prior knowledge and initial graph are the same.
        if self.initial_graph.labels() != prior_knowledge.labels() {
            return Err(Error::LabelMismatch(
                &format!("{:?}", self.initial_graph.labels()),
                &format!("{:?}", prior_knowledge.labels()),
            ));
        }
        // Check prior knowledge is consistent with initial graph.
        for edge in self.initial_graph.vertices().into_iter().permutations(2) {
            // Get the edge indices.
            let (i, j) = (edge[0], edge[1]);
            // Check edge must be either present and not forbidden ...
            if self.initial_graph.has_edge(i, j)? {
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
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn fit(&self) -> Result<DiGraph> {
        // Clone the initial graph.
        let mut graph = self.initial_graph.clone();

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
                        if let Some(pk) = self.prior_knowledge {
                            // If the edge is required, skip the tests.
                            // NOTE: Since CTPC only removes edges,
                            //  it is sufficient to check for required edges.
                            if pk.is_required(j, i) {
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
                                            Err(e) => Some(Err(e)),
                                        }
                                    }
                                    Ok(false) => None,
                                    Err(e) => Some(Err(e)),
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

        // Return the fitted graph.
        Ok(graph)
    }
}

impl<'a, T, S> CTPC<'a, T, S>
where
    T: CITest + Sync,
    S: CITest + Sync,
{
    /// Execute the CTPC algorithm and return the fitted graph in parallel.
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
            .map(|i| -> Result<Set<usize>> {
                // Get the parents of the vertex.
                let mut pa_i = self.initial_graph.parents(&set![i])?;

                // Initialize the counter.
                let mut k = 0;

                // While the counter is smaller than the number of parents ...
                while k < pa_i.len() {
                    // Filter the parents in parallel.
                    pa_i = pa_i
                        .par_iter()
                        .map(|&j| -> Result<Option<usize>> {
                            // Check prior knowledge, if available.
                            if let Some(pk) = self.prior_knowledge {
                                // If the edge is required, skip the tests.
                                // NOTE: Since CTPC only removes edges,
                                //  it is sufficient to check for required edges.
                                if pk.is_required(j, i) {
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
        let mut graph = DiGraph::empty(self.initial_graph.labels())?;

        // Set the parents of each vertex.
        parents.into_iter().enumerate().try_for_each(|(i, pa_i)| {
            // For each parent ...
            pa_i.into_iter().try_for_each(|j| -> Result<_> {
                graph.add_edge(j, i)?;
                Ok(())
            })
        })?;

        // Return the fitted graph.
        Ok(graph)
    }
}

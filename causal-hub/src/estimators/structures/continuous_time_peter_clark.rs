use itertools::Itertools;
use log::debug;
use ndarray::{Zip, prelude::*};
use rayon::prelude::*;
use statrs::distribution::{ChiSquared, ContinuousCDF, FisherSnedecor};

use crate::{
    estimators::{CIMEstimator, PK},
    models::{CIM, CatCIM, DiGraph, Graph, Labelled},
    set,
    types::{Error, Labels, Result, Set},
};

/// A trait for conditional independence testing.
pub trait CITest {
    /// Test for conditional independence as X _||_ Y | Z.
    ///
    /// # Arguments
    ///
    /// * `x` - The first variable.
    /// * `y` - The second variable.
    /// * `z` - The conditioning set.
    ///
    /// # Returns
    ///
    /// `true` if X _||_ Y | Z, `false` otherwise.
    ///
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool>;
}

/// A struct representing the Chi-squared test.
pub struct ChiSquaredTest<'a, E> {
    estimator: &'a E,
    alpha: f64,
}

impl<'a, E> ChiSquaredTest<'a, E> {
    /// Creates a new `ChiSquaredTest` instance.
    ///
    /// # Arguments
    ///
    /// * `estimator` - A reference to the estimator.
    /// * `alpha` - The significance level.
    ///
    /// # Returns
    ///
    /// A new `ChiSquaredTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Result<Self> {
        // Assert that the significance level is in [0, 1].
        if !(0.0..=1.0).contains(&alpha) {
            return Err(Error::InvalidParameter(
                "alpha".into(),
                "must be in [0, 1]".into(),
            ));
        }

        Ok(Self { estimator, alpha })
    }
}

impl<'a, E> Labelled for ChiSquaredTest<'a, E>
where
    E: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.estimator.labels()
    }
}

impl<E> CITest for ChiSquaredTest<'_, E>
where
    E: CIMEstimator<CatCIM>,
{
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool> {
        // Assert Y contains exactly one label.
        // TODO: Refactor code and remove this assumption.
        if y.len() != 1 {
            return Err(Error::InvalidParameter(
                "y".into(),
                "must contain exactly one label".into(),
            ));
        }

        // Compute the extended separation set.
        let mut s = z.clone();
        // Get the ordered position of Y in the extended separation set.
        let s_y = match z.binary_search(&y[0]) {
            Ok(_) => return Err(Error::SetsNotDisjoint("Y".into(), "Z".into())),
            Err(i) => i,
        };
        // Insert Y into the extended separation set in sorted order.
        s.shift_insert(s_y, y[0]);

        // Fit the intensity matrices.
        let q_xz = self.estimator.fit(x, z)?;
        let q_xs = self.estimator.fit(x, &s)?;
        // Get the sufficient statistics for the sets.
        let n_xz = q_xz
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .ok_or(Error::MissingSufficientStatistics)?;
        let n_xs = q_xs
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .ok_or(Error::MissingSufficientStatistics)?;

        // Get the shape of the extended separation set.
        let c_s = q_xs.conditioning_shape();
        // Get the shape of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        for j in 0..n_xs.shape()[0] {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the chi-squared distribution.
            let k_xz = n_xz.index_axis(Axis(0), i);
            let k_xs = n_xs.index_axis(Axis(0), j);
            // Compute the scaling factors.
            let k = &k_xz.sum_axis(Axis(1)) / &k_xs.sum_axis(Axis(1));
            let k = k.sqrt().insert_axis(Axis(1));
            let l = k.recip();
            // Compute the chi-squared statistic for uneven number of samples.
            let chi_sq_num = (&k * &k_xs - &l * &k_xz).powi(2);
            let chi_sq_den = &k_xs + &k_xz;
            let chi_sq = chi_sq_num / &chi_sq_den;
            // Fix division by zero.
            let chi_sq = chi_sq.mapv(|x| if x.is_finite() { x } else { 0. });
            // Compute the chi-squared statistic.
            let chi_sq = chi_sq.sum_axis(Axis(1));
            // For each chi-squared statistic ...
            for (c, d) in chi_sq.into_iter().zip(chi_sq_den.rows()) {
                // Count the non-zero degrees of freedom.
                let dof = d.mapv(|d| (d > 0.) as usize).sum();
                // Check if the degrees of freedom is at least 2.
                let dof = if dof >= 2 { dof } else { 2 };
                // Initialize the chi-squared distribution.
                let n = ChiSquared::new((dof - 1) as f64)
                    .map_err(|e| Error::Probability(e.to_string()))?;
                // Compute the p-value.
                let p_value = n.cdf(c);
                // Check if the p-value is in the alpha range.
                if p_value >= (1. - self.alpha) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}

/// A struct representing the F test.
pub struct FTest<'a, E> {
    estimator: &'a E,
    alpha: f64,
}

impl<'a, E> FTest<'a, E> {
    /// Creates a new `FTest` instance.
    ///
    /// # Arguments
    ///
    /// * `estimator` - A reference to the estimator.
    /// * `alpha` - The significance level.
    ///
    /// # Returns
    ///
    /// A new `FTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Result<Self> {
        // Assert that the significance level is in [0, 1].
        if !(0.0..=1.0).contains(&alpha) {
            return Err(Error::InvalidParameter(
                "alpha".into(),
                "must be in [0, 1]".into(),
            ));
        }

        Ok(Self { estimator, alpha })
    }
}

impl<E> Labelled for FTest<'_, E>
where
    E: Labelled,
{
    #[inline]
    fn labels(&self) -> &Labels {
        self.estimator.labels()
    }
}

impl<E> CITest for FTest<'_, E>
where
    E: CIMEstimator<CatCIM>,
{
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool> {
        // Assert Y contains exactly one label.
        // TODO: Refactor code and remove this assumption.
        if y.len() != 1 {
            return Err(Error::InvalidParameter(
                "y".into(),
                "must contain exactly one label".into(),
            ));
        }

        // Compute the alpha range.
        let alpha = (self.alpha / 2.)..=(1. - self.alpha / 2.);

        // Compute the extended separation set.
        let mut s = z.clone();
        // Get the ordered position of Y in the extended separation set.
        let s_y = match z.binary_search(&y[0]) {
            Ok(_) => return Err(Error::SetsNotDisjoint("Y".into(), "Z".into())),
            Err(i) => i,
        };
        // Insert Y into the extended separation set in sorted order.
        s.shift_insert(s_y, y[0]);

        // Fit the intensity matrices.
        let q_xz = self.estimator.fit(x, z)?;
        let q_xs = self.estimator.fit(x, &s)?;
        // Get the sufficient statistics for the sets.
        let n_xz = q_xz
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .ok_or(Error::MissingSufficientStatistics)?;
        let n_xs = q_xs
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .ok_or(Error::MissingSufficientStatistics)?;

        // Get the shape of the extended separation set.
        let c_s = q_xs.conditioning_shape();
        // Get the shape of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        for j in 0..n_xs.shape()[0] {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the Fisher-Snedecor distribution.
            let r_xz = n_xz.index_axis(Axis(0), i).sum_axis(Axis(1));
            let r_xs = n_xs.index_axis(Axis(0), j).sum_axis(Axis(1));
            // Get the intensity matrices for the separation sets.
            let q_xz = q_xz.parameters().index_axis(Axis(0), i);
            let q_xs = q_xs.parameters().index_axis(Axis(0), j);
            // Perform the F-test.
            let all_passed = Zip::from(&r_xz)
                .and(&r_xs)
                .and(q_xz.diag())
                .and(q_xs.diag())
                .fold(Ok(true), |acc, &r_xz, &r_xs, &q_xz, &q_xs| -> Result<_> {
                    if let Ok(true) = acc {
                        // Initialize the Fisher-Snedecor distribution.
                        let f = FisherSnedecor::new(r_xz, r_xs)
                            .map_err(|e| Error::Probability(e.to_string()))?;
                        // Compute the p-value.
                        let p_value = f.cdf(q_xz / q_xs);
                        // Check if the p-value is in the alpha range.
                        if alpha.contains(&p_value) {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    } else {
                        acc
                    }
                })?;

            if !all_passed {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

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
        // Assert labels of the initial graph and the estimator are the same.
        if initial_graph.labels() != null_time.labels() {
            return Err(Error::LabelMismatch(
                format!("{:?}", initial_graph.labels()),
                format!("{:?}", null_time.labels()),
            ));
        }
        // Assert labels of the initial graph and the estimator are the same.
        if initial_graph.labels() != null_state.labels() {
            return Err(Error::LabelMismatch(
                format!("{:?}", initial_graph.labels()),
                format!("{:?}", null_state.labels()),
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
    /// A mutable reference to the current instance.
    ///
    #[inline]
    pub fn with_prior_knowledge(mut self, prior_knowledge: &'a PK) -> Result<Self> {
        // Assert labels of prior knowledge and initial graph are the same.
        if self.initial_graph.labels() != prior_knowledge.labels() {
            return Err(Error::LabelMismatch(
                format!("{:?}", self.initial_graph.labels()),
                format!("{:?}", prior_knowledge.labels()),
            ));
        }
        // Assert prior knowledge is consistent with initial graph.
        for edge in self.initial_graph.vertices().into_iter().permutations(2) {
            // Get the edge indices.
            let (i, j) = (edge[0], edge[1]);
            // Assert edge must be either present and not forbidden ...
            if self.initial_graph.has_edge(i, j)? {
                if prior_knowledge.is_forbidden(i, j) {
                    return Err(Error::PriorKnowledgeConflict(format!(
                        "Initial graph contains forbidden edge ({i}, {j})."
                    )));
                }
            }
            // ... or absent and not required.
            else if prior_knowledge.is_required(i, j) {
                return Err(Error::PriorKnowledgeConflict(format!(
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

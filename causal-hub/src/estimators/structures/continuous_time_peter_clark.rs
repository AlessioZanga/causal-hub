use itertools::Itertools;
use log::debug;
use ndarray::{Zip, prelude::*};
use rayon::prelude::*;
use statrs::distribution::{ChiSquared, ContinuousCDF, FisherSnedecor};

use crate::{
    estimators::{CIMEstimator, PK},
    models::{CIM, CatCIM, DiGraph, Graph, Labelled},
    set,
    types::{Labels, Set},
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
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool;
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
    /// # Panics
    ///
    /// Panics if the significance level is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `ChiSquaredTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Self {
        // Assert that the significance level is in [0, 1].
        assert!((0.0..=1.0).contains(&alpha), "Alpha must be in [0, 1]");

        Self { estimator, alpha }
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
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool {
        // Assert Y contains exactly one label.
        // TODO: Refactor code and remove this assumption.
        assert_eq!(y.len(), 1, "Y must contain exactly one label.");

        // Compute the extended separation set.
        let mut s = z.clone();
        // Get the ordered position of Y in the extended separation set.
        let s_y = z.binary_search(&y[0]).unwrap_err();
        // Insert Y into the extended separation set in sorted order.
        s.shift_insert(s_y, y[0]);

        // Fit the intensity matrices.
        let q_xz = self.estimator.fit(x, z);
        let q_xs = self.estimator.fit(x, &s);
        // Get the sufficient statistics for the sets.
        let n_xz = q_xz
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .unwrap();
        let n_xs = q_xs
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .unwrap();

        // Get the shape of the extended separation set.
        let c_s = q_xs.conditioning_shape();
        // Get the shape of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        (0..n_xs.shape()[0]).all(|j| {
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
            chi_sq
                .into_iter()
                .zip(chi_sq_den.rows())
                .map(|(c, d)| {
                    // Count the non-zero degrees of freedom.
                    let dof = d.mapv(|d| (d > 0.) as usize).sum();
                    // Check if the degrees of freedom is at least 2.
                    let dof = if dof >= 2 { dof } else { 2 };
                    // Initialize the chi-squared distribution.
                    let n = ChiSquared::new((dof - 1) as f64).unwrap();
                    // Compute the p-value.
                    n.cdf(c)
                })
                // Check if the p-value is in the alpha range.
                .all(|p_value| p_value < (1. - self.alpha))
        })
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
    /// # Panics
    ///
    /// Panics if the significance level is not in [0, 1].
    ///
    /// # Returns
    ///
    /// A new `FTest` instance.
    ///
    #[inline]
    pub fn new(estimator: &'a E, alpha: f64) -> Self {
        // Assert that the significance level is in [0, 1].
        assert!((0.0..=1.0).contains(&alpha), "Alpha must be in [0, 1]");

        Self { estimator, alpha }
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
    fn call(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool {
        // Assert Y contains exactly one label.
        // TODO: Refactor code and remove this assumption.
        assert_eq!(y.len(), 1, "Y must contain exactly one label.");

        // Compute the alpha range.
        let alpha = (self.alpha / 2.)..=(1. - self.alpha / 2.);

        // Compute the extended separation set.
        let mut s = z.clone();
        // Get the ordered position of Y in the extended separation set.
        let s_y = z.binary_search(&y[0]).unwrap_err();
        // Insert Y into the extended separation set in sorted order.
        s.shift_insert(s_y, y[0]);

        // Fit the intensity matrices.
        let q_xz = self.estimator.fit(x, z);
        let q_xs = self.estimator.fit(x, &s);
        // Get the sufficient statistics for the sets.
        let n_xz = q_xz
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .unwrap();
        let n_xs = q_xs
            .sample_statistics()
            .map(|s| s.sample_conditional_counts())
            .unwrap();

        // Get the shape of the extended separation set.
        let c_s = q_xs.conditioning_shape();
        // Get the shape of the parent and the remaining strides.
        let (c_y, c_s) = (c_s[s_y], c_s.slice(s![(s_y + 1)..]).product());

        // For each combination of the extended parent set ...
        (0..n_xs.shape()[0]).all(|j| {
            // Compute the corresponding index for the separation set.
            let i = j % c_s + (j / (c_s * c_y)) * c_s;
            // Get the parameters of the Fisher-Snedecor distribution.
            let r_xz = n_xz.index_axis(Axis(0), i).sum_axis(Axis(1));
            let r_xs = n_xs.index_axis(Axis(0), j).sum_axis(Axis(1));
            // Get the intensity matrices for the separation sets.
            let q_xz = q_xz.parameters().index_axis(Axis(0), i);
            let q_xs = q_xs.parameters().index_axis(Axis(0), j);
            // Perform the F-test.
            Zip::from(&r_xz)
                .and(&r_xs)
                .and(q_xz.diag())
                .and(q_xs.diag())
                .all(|&r_xz, &r_xs, &q_xz, &q_xs| {
                    // Initialize the Fisher-Snedecor distribution.
                    let f = FisherSnedecor::new(r_xz, r_xs).unwrap();
                    // Compute the p-value.
                    let p_value = f.cdf(q_xz / q_xs);
                    // Check if the p-value is in the alpha range.
                    alpha.contains(&p_value)
                })
        })
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
    pub fn new(initial_graph: &'a DiGraph, null_time: &'a T, null_state: &'a S) -> Self {
        // Assert labels of the initial graph and the estimator are the same.
        assert_eq!(
            initial_graph.labels(),
            null_time.labels(),
            "Labels of initial graph and estimator must be the same: \n\
            \t expected:    {:?}, \n\
            \t found:       {:?}.",
            initial_graph.labels(),
            null_time.labels()
        );
        // Assert labels of the initial graph and the estimator are the same.
        assert_eq!(
            initial_graph.labels(),
            null_state.labels(),
            "Labels of initial graph and estimator must be the same: \n\
            \t expected:    {:?}, \n\
            \t found:       {:?}.",
            initial_graph.labels(),
            null_state.labels()
        );

        Self {
            initial_graph,
            null_time,
            null_state,
            prior_knowledge: None,
        }
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
    pub fn with_prior_knowledge(mut self, prior_knowledge: &'a PK) -> Self {
        // Assert labels of prior knowledge and initial graph are the same.
        assert_eq!(
            self.initial_graph.labels(),
            prior_knowledge.labels(),
            "Labels of initial graph and prior knowledge must be the same: \n\
            \t expected:    {:?}, \n\
            \t found:       {:?}.",
            self.initial_graph.labels(),
            prior_knowledge.labels()
        );
        // Assert prior knowledge is consistent with initial graph.
        self.initial_graph
            .vertices()
            .into_iter()
            .permutations(2)
            .for_each(|edge| {
                // Get the edge indices.
                let (i, j) = (edge[0], edge[1]);
                // Assert edge must be either present and not forbidden ...
                if self.initial_graph.has_edge(i, j) {
                    assert!(
                        !prior_knowledge.is_forbidden(i, j),
                        "Initial graph contains forbidden edge ({i}, {j})."
                    );
                // ... or absent and not required.
                } else {
                    assert!(
                        !prior_knowledge.is_required(i, j),
                        "Initial graph does not contain required edge ({i}, {j})."
                    );
                }
            });
        // Set prior knowledge.
        self.prior_knowledge = Some(prior_knowledge);
        self
    }

    /// Execute the CTPC algorithm.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn fit(&self) -> DiGraph {
        // Clone the initial graph.
        let mut graph = self.initial_graph.clone();

        // For each vertex in the graph ...
        for i in graph.vertices() {
            // Get the parents of the vertex.
            let mut pa_i = graph.parents(&set![i]);

            // Initialize the counter.
            let mut k = 0;

            // While the counter is smaller than the number of parents ...
            while k < pa_i.len() {
                // Initialize the set of vertices to remove, to ensure stability.
                let mut not_pa_i = Vec::new();

                // For each parent ...
                for &j in &pa_i {
                    // Check prior knowledge, if available.
                    if let Some(pk) = self.prior_knowledge {
                        // If the edge is required, skip the tests.
                        // NOTE: Since CTPC only removes edges,
                        //  it is sufficient to check for required edges.
                        if pk.is_required(j, i) {
                            // Log the skipped CIT.
                            debug!("CIT for {j} _||_ {i} | [*] ... SKIPPED");
                            continue;
                        }
                    }
                    // Filter out the parent.
                    let pa_i_not_j = pa_i.iter().filter(|&&z| z != j).cloned();
                    // For any combination of size k of Pa(X_i) \ { X_j } ...
                    for s_ij in pa_i_not_j.combinations(k).map(Set::from_iter) {
                        // Log the current combination.
                        debug!("CIT for {i} _||_ {j} | {s_ij:?} ...");
                        // If X_i _||_ X_j | S_{X_i, X_j} ...
                        if self.null_time.call(&set![i], &set![j], &s_ij)
                            && self.null_state.call(&set![i], &set![j], &s_ij)
                        {
                            // Log the result of the CIT.
                            debug!("CIT for {i} _||_ {j} | {s_ij:?} ... PASSED");
                            // Add the parent to the set of vertices to remove.
                            not_pa_i.push(j);
                            // Break the outer loop.
                            break;
                        }
                    }
                }

                // Remove the vertices from the graph.
                for &j in &not_pa_i {
                    // Remove the vertex from the parents.
                    pa_i.retain(|&x| x != j);
                    // Remove the edge from the graph.
                    graph.del_edge(j, i);
                }

                // Increment the counter.
                k += 1;
            }
        }

        // Return the fitted graph.
        graph
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
    pub fn par_fit(&self) -> DiGraph {
        // For each vertex in the graph ...
        let parents: Vec<_> = self
            .initial_graph
            .vertices()
            .into_par_iter()
            .map(|i| {
                // Get the parents of the vertex.
                let mut pa_i = self.initial_graph.parents(&set![i]);

                // Initialize the counter.
                let mut k = 0;

                // While the counter is smaller than the number of parents ...
                while k < pa_i.len() {
                    // Filter the parents in parallel.
                    pa_i = pa_i
                        .par_iter()
                        .filter_map(|&j| {
                            // Check prior knowledge, if available.
                            if let Some(pk) = self.prior_knowledge {
                                // If the edge is required, skip the tests.
                                // NOTE: Since CTPC only removes edges,
                                //  it is sufficient to check for required edges.
                                if pk.is_required(j, i) {
                                    // Log the skipped CIT.
                                    debug!("CIT for {j} _||_ {i} | [*] ... SKIPPED");
                                    return Some(j);
                                }
                            }
                            // Filter out the parent.
                            let pa_i_not_j = pa_i.iter().filter(|&&z| z != j).cloned();
                            // For any combination of size k of Pa(X_i) \ { X_j } ...
                            for s_ij in pa_i_not_j.combinations(k).map(Set::from_iter) {
                                // Log the current combination.
                                debug!("CIT for {i} _||_ {j} | {s_ij:?} ...");
                                // If X_i _||_ X_j | S_{X_i, X_j} ...
                                if self.null_time.call(&set![i], &set![j], &s_ij)
                                    && self.null_state.call(&set![i], &set![j], &s_ij)
                                {
                                    // Log the result of the CIT.
                                    debug!("CIT for {i} _||_ {j} | {s_ij:?} ... PASSED");
                                    // Add the parent to the set of vertices to remove.
                                    return None;
                                }
                            }
                            // Otherwise, keep the parent.
                            Some(j)
                        })
                        .collect();
                    // Increment the counter.
                    k += 1;
                }

                // Return the parents of the vertex.
                pa_i
            })
            .collect();

        // Initialize an empty graph.
        let mut graph = DiGraph::empty(self.initial_graph.labels());

        // Set the parents of each vertex.
        parents.into_iter().enumerate().for_each(|(i, pa_i)| {
            // For each parent ...
            pa_i.into_iter().for_each(|j| {
                // Add the edge to the graph.
                graph.add_edge(j, i);
            })
        });

        // Return the fitted graph.
        graph
    }
}

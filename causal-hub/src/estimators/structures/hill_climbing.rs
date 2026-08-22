use std::cmp::Ordering;

use itertools::iproduct;
use rayon::prelude::*;

use crate::{
    estimators::{PK, ScoringCriterion},
    inference::TopologicalOrder,
    models::{DiGraph, Graph, Labelled},
    set,
    types::{Error, ErrorKind, Result, Set},
};

/// Local edge operation type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    /// Add edge operation.
    Add,
    /// Delete edge operation.
    Del,
    /// Reverse edge operation.
    Rev,
}

/// Local action (operation, edge) type.
type A = (usize, usize, Op);

/// Local edge space type.
type E = Set<(usize, usize)>;

/// Local operations edge space type.
type ES = (
    E, // To-be-added space.
    E, // To-be-deleted space.
    E, // To-be-reversed space.
);

/// The hill climbing algorithm for structure learning in BNs.
#[derive(Clone, Debug)]
pub struct HC<'a, S> {
    score: &'a S,
    initial_graph: Option<&'a DiGraph>,
    max_parents: Option<usize>,
    max_iter: usize,
    prior_knowledge: Option<&'a PK>,
}

impl<'a, S> HC<'a, S>
where
    S: ScoringCriterion + Labelled,
{
    /// Creates a new hill climbing instance.
    ///
    /// # Arguments
    ///
    /// * `score` - The scoring criterion to use.
    ///
    /// # Returns
    ///
    /// A new `HC` instance.
    ///
    /// # Notes
    ///
    /// By default, the search starts from an empty graph over the labels of the
    /// scoring criterion. Use [`HC::with_initial_graph`] to provide a different
    /// starting point.
    ///
    #[inline]
    pub fn new(score: &'a S) -> Self {
        Self {
            initial_graph: None,
            score,
            max_parents: None,
            max_iter: usize::MAX,
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

    /// Sets the maximum number of iterations.
    ///
    /// # Arguments
    ///
    /// * `max_iter` - The maximum number of iterations.
    ///
    /// # Returns
    ///
    /// The modified instance.
    ///
    #[inline]
    pub const fn with_max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Sets the prior knowledge for the algorithm.
    ///
    /// # Arguments
    ///
    /// * `prior_knowledge` - The prior knowledge to use.
    ///
    /// # Errors
    ///
    /// * If the labels of the prior knowledge and the initial graph do not match.
    /// * If the prior knowledge conflicts with the initial graph.
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
        // Check prior knowledge is consistent with initial graph:
        // every edge of the initial graph must not be forbidden.
        // Note that required edges missing from the initial graph are
        // not a conflict, since they are added during initialization.
        for (i, j) in initial_graph.edges() {
            // Check edge must not be forbidden.
            if prior_knowledge.is_forbidden(i, j) {
                return Err(Error::PriorKnowledgeConflict(&format!(
                    "Initial graph contains forbidden edge ({i}, {j})."
                )));
            }
        }
        // Set prior knowledge.
        self.prior_knowledge = Some(prior_knowledge);
        Ok(self)
    }
}

impl<S> HC<'_, S> {
    /// Checks if an edge is forbidden by the prior knowledge, if any.
    #[inline]
    fn is_forbidden(&self, x: usize, y: usize) -> bool {
        self.prior_knowledge.is_some_and(|k| k.is_forbidden(x, y))
    }

    /// Checks if an edge is required by the prior knowledge, if any.
    #[inline]
    fn is_required(&self, x: usize, y: usize) -> bool {
        self.prior_knowledge.is_some_and(|k| k.is_required(x, y))
    }
}

impl<S> HC<'_, S>
where
    S: ScoringCriterion + Labelled,
{
    /// Initializes the search space, the in-degrees and the current solution.
    fn init(&self) -> Result<(ES, Vec<usize>, DiGraph)> {
        // Get the initial graph, or an empty graph over the labels of the scoring criterion.
        let mut g = match self.initial_graph {
            Some(graph) => graph.clone(),
            None => DiGraph::empty(self.score.labels())?,
        };

        // If prior knowledge is set ...
        if let Some(k) = self.prior_knowledge {
            // Check that every edge in the graph is not forbidden.
            for (x, y) in g.edges() {
                if k.is_forbidden(x, y) {
                    return Err(Error::PriorKnowledgeConflict(&format!(
                        "Initial graph contains forbidden edge ({x}, {y})."
                    )));
                }
            }
            // Check that every required edge is in the graph.
            for (x, y) in k.required_edges() {
                if !g.has_edge(x, y)? {
                    g.add_edge(x, y)?;
                }
            }
        }

        // Check acyclicity.
        if g.topological_order().is_none() {
            return Err(Error::NotADag());
        }

        // Get the number of vertices.
        let n = g.vertices().len();
        // Get the columns index.
        let order: Vec<usize> = (0..n).collect();

        // Get the current edge set.
        let e = g.edges();
        // Initialize the potential edges to be added.
        let add: E = iproduct!(order.iter(), order.iter())
            .map(|(&x, &y)| (x, y))
            // Remove any edge (X, Y) s.t. X == Y, is present in the initial graph,
            // or is in the forbidden list.
            .filter(|&(x, y)| x != y && !e.contains(&(x, y)) && !self.is_forbidden(x, y))
            .collect();
        // Initialize the potential edges to be deleted.
        let del: E = e
            .iter()
            // Remove any edge in the required list.
            .filter(|&&(x, y)| !self.is_required(x, y))
            .copied()
            .collect();
        // Initialize the potential edges to be reversed.
        let rev: E = e
            .iter()
            // Remove any reversed edge in the required or forbidden list.
            .filter(|&&(x, y)| !self.is_required(x, y) && !self.is_forbidden(y, x))
            .copied()
            .collect();

        // Compute the current in-degree.
        let in_degree = (0..n)
            .map(|y| g.parents(&set![y]).map(|pa| pa.len()))
            .collect::<Result<_>>()?;

        Ok(((add, del, rev), in_degree, g))
    }
}

impl<S> HC<'_, S> {
    /// Checks if an edge operation is consistent with acyclicity and hyper-parameters.
    fn is_valid(
        &self,
        op: Op,
        in_degree: &[usize],
        g: &DiGraph,
        x: usize,
        y: usize,
    ) -> Result<bool> {
        // Check validity depending on operation.
        match op {
            // |Pa(G, Y)| < max parents, (Y, X) not in G, no path from Y to X.
            Op::Add => {
                let under_max_parents = self.max_parents.is_none_or(|m| in_degree[y] < m);
                let not_present = !g.has_edge(y, x)?;
                let acyclic = !g.descendants(&set![y])?.contains(&x);

                Ok(under_max_parents && not_present && acyclic)
            }
            // Any present edge can be deleted.
            Op::Del => Ok(true),
            // |Pa(G, X)| < max parents, no path from any child of X, other than Y, to Y.
            Op::Rev => {
                let under_max_parents = self.max_parents.is_none_or(|m| in_degree[x] < m);
                // Get the children of X, other than Y.
                let children: Set<usize> = g
                    .children(&set![x])?
                    .into_iter()
                    .filter(|&z| z != y)
                    .collect();
                // The reversal is acyclic iff Y is not reachable from any such child.
                let acyclic = !g.descendants(&children)?.contains(&y);

                Ok(under_max_parents && acyclic)
            }
        }
    }

    /// Applies an edge operation to the given graph and in-degrees.
    fn apply(in_degree: &mut [usize], g: &mut DiGraph, x: usize, y: usize, op: Op) -> Result<()> {
        // Apply operation.
        match op {
            Op::Add => {
                let added = g.add_edge(x, y)?;
                debug_assert!(added);
                in_degree[y] += 1;
            }
            Op::Del => {
                let deleted = g.del_edge(x, y)?;
                debug_assert!(deleted);
                in_degree[y] -= 1;
            }
            Op::Rev => {
                let deleted = g.del_edge(x, y)?;
                debug_assert!(deleted);
                in_degree[y] -= 1;
                let added = g.add_edge(y, x)?;
                debug_assert!(added);
                in_degree[x] += 1;
            }
        }

        Ok(())
    }

    /// Updates the edge spaces for each edge operation.
    fn update((mut add, mut del, mut rev): ES, x: usize, y: usize, op: Op) -> ES {
        // Apply operation.
        match op {
            Op::Add => {
                // Remove performed action.
                let removed = add.shift_remove(&(x, y));
                debug_assert!(removed);
                // Add(X, Y) implies that (X, Y) is not in the
                // required list, therefore Del(X, Y) is valid.
                let inserted = del.insert((x, y));
                debug_assert!(inserted);
                // If Add(Y, X) and Del(X, Y) are valid, then Rev(X, Y) is valid.
                // Since Del(X, Y) is valid by construction, check only Add(Y, X).
                if add.contains(&(y, x)) {
                    let inserted = rev.insert((x, y));
                    debug_assert!(inserted);
                }
            }
            Op::Del => {
                // Del(X, Y) implies that (X, Y) is not in the
                // forbidden list, therefore Add(X, Y) is valid.
                let inserted = add.insert((x, y));
                debug_assert!(inserted);
                // Remove performed action.
                let removed = del.shift_remove(&(x, y));
                debug_assert!(removed);
                // If Add(Y, X) and Del(X, Y) are valid, then Rev(X, Y) is valid.
                // Since Del(X, Y) is valid by construction, check only Add(Y, X).
                if add.contains(&(y, x)) {
                    let removed = rev.shift_remove(&(x, y));
                    debug_assert!(removed);
                }
            }
            Op::Rev => {
                // Remove performed action(s).
                let removed_yx = add.shift_remove(&(y, x));
                let removed_xy_del = del.shift_remove(&(x, y));
                let removed_xy_rev = rev.shift_remove(&(x, y));
                debug_assert!(removed_xy_del && removed_xy_rev);
                // Rev(X, Y) implies that (X, Y) is not in the
                // required list nor in the forbidden list,
                // therefore, Add(X, Y) is valid.
                let inserted_xy = add.insert((x, y));
                debug_assert!(inserted_xy);
                // Rev(X, Y) implies that (Y, X) is not in the
                // required list nor in the forbidden list,
                // therefore, Del(Y, X) is valid.
                let inserted_yx_del = del.insert((y, x));
                debug_assert!(inserted_yx_del);
                // If Rev(X, Y) is valid then Rev(Y, X) is valid.
                let inserted_yx_rev = rev.insert((y, x));
                debug_assert!(inserted_yx_rev);
                // The reversed edge (Y, X) may not have been in the add space.
                let _ = removed_yx;
            }
        }

        (add, del, rev)
    }
}

impl<S> HC<'_, S>
where
    S: ScoringCriterion,
{
    /// Computes the delta score of an edge operation on the given graph.
    fn delta(&self, g: &DiGraph, x: usize, y: usize, op: Op) -> Result<f64> {
        // Compute the delta score depending on operation.
        match op {
            Op::Add => {
                // Get the current parents of Y.
                let pa_y = g.parents(&set![y])?;
                // Compute the current local score of Y.
                let s_y = self.score.call(&set![y], &pa_y)?;
                // Add X in-place, leveraging the parents order.
                let mut pa_star = pa_y.clone();
                let i = match pa_star.binary_search(&x) {
                    Err(i) => i,
                    Ok(_) => {
                        return Err(Error::Unreachable("Edge to be added is already present."));
                    }
                };
                pa_star.shift_insert(i, x);
                // Compute the new local score of Y.
                let s_star = self.score.call(&set![y], &pa_star)?;

                Ok(s_star - s_y)
            }
            Op::Del => {
                // Get the current parents of Y.
                let pa_y = g.parents(&set![y])?;
                // Compute the current local score of Y.
                let s_y = self.score.call(&set![y], &pa_y)?;
                // Remove X in-place, leveraging the parents order.
                let mut pa_star = pa_y.clone();
                let i = match pa_star.binary_search(&x) {
                    Ok(i) => i,
                    Err(_) => return Err(Error::Unreachable("Edge to be deleted is not present.")),
                };
                let removed = pa_star.shift_remove_index(i);
                debug_assert!(removed.is_some());
                // Compute the new local score of Y.
                let s_star = self.score.call(&set![y], &pa_star)?;

                Ok(s_star - s_y)
            }
            Op::Rev => {
                // Get the current parents of X and Y.
                let pa_x = g.parents(&set![x])?;
                let pa_y = g.parents(&set![y])?;
                // Compute the current local scores of X and Y.
                let s_x = self.score.call(&set![x], &pa_x)?;
                let s_y = self.score.call(&set![y], &pa_y)?;

                // Add Y in-place to the parents of X, leveraging the parents order.
                let mut pa_x_star = pa_x.clone();
                let i = match pa_x_star.binary_search(&y) {
                    Err(i) => i,
                    Ok(_) => return Err(Error::Unreachable("Reversed edge is already present.")),
                };
                pa_x_star.shift_insert(i, y);
                // Compute the new local score of X.
                let s_x_star = self.score.call(&set![x], &pa_x_star)?;

                // Remove X in-place from the parents of Y, leveraging the parents order.
                let mut pa_y_star = pa_y.clone();
                let i = match pa_y_star.binary_search(&x) {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(Error::Unreachable("Edge to be reversed is not present."));
                    }
                };
                let removed = pa_y_star.shift_remove_index(i);
                debug_assert!(removed.is_some());
                // Compute the new local score of Y.
                let s_y_star = self.score.call(&set![y], &pa_y_star)?;

                Ok((s_x_star - s_x) + (s_y_star - s_y))
            }
        }
    }

    /// Searches for the best operation given the current graph and edge spaces.
    fn search(
        &self,
        (add, del, rev): (&E, &E, &E),
        in_degree: &[usize],
        g: &DiGraph,
    ) -> Result<Option<(A, f64)>> {
        // Chain the three operation spaces.
        let ops = add
            .iter()
            .map(|&e| (e, Op::Add))
            .chain(del.iter().map(|&e| (e, Op::Del)))
            .chain(rev.iter().map(|&e| (e, Op::Rev)));

        // For each possible operation, check validity and compute the delta score.
        let deltas = ops
            .filter_map(|((x, y), op)| match self.is_valid(op, in_degree, g, x, y) {
                Ok(true) => match self.delta(g, x, y, op) {
                    // Skip operations leading to degenerate (zero-count)
                    // parent configurations, since their score is undefined.
                    Ok(delta) => Some(Ok(((x, y, op), delta))),
                    Err(evidence)
                        if matches!(evidence.kind, ErrorKind::MissingSufficientStatistics) =>
                    {
                        None
                    }
                    Err(evidence) => Some(Err(evidence)),
                },
                Ok(false) => None,
                Err(evidence) => Some(Err(evidence)),
            })
            .collect::<Result<Vec<_>>>()?;

        // Get the operation with highest strictly positive delta score, if any.
        Ok(deltas.into_iter().filter(|(_, delta)| *delta > 0.).max_by(
            |(_, delta), (_, delta_star)| delta.partial_cmp(delta_star).unwrap_or(Ordering::Equal),
        ))
    }
}

impl<S> HC<'_, S>
where
    S: ScoringCriterion + Sync,
{
    /// Searches for the best operation given the current graph and edge spaces, in parallel.
    fn par_search(
        &self,
        (add, del, rev): (&E, &E, &E),
        in_degree: &[usize],
        g: &DiGraph,
    ) -> Result<Option<(A, f64)>> {
        // Chain the three operation spaces.
        let ops: Vec<_> = add
            .iter()
            .map(|&e| (e, Op::Add))
            .chain(del.iter().map(|&e| (e, Op::Del)))
            .chain(rev.iter().map(|&e| (e, Op::Rev)))
            .collect();

        // For each possible operation, check validity and compute the delta score.
        let deltas = ops
            .into_par_iter()
            .filter_map(|((x, y), op)| match self.is_valid(op, in_degree, g, x, y) {
                Ok(true) => match self.delta(g, x, y, op) {
                    // Skip operations leading to degenerate (zero-count)
                    // parent configurations, since their score is undefined.
                    Ok(delta) => Some(Ok(((x, y, op), delta))),
                    Err(evidence)
                        if matches!(evidence.kind, ErrorKind::MissingSufficientStatistics) =>
                    {
                        None
                    }
                    Err(evidence) => Some(Err(evidence)),
                },
                Ok(false) => None,
                Err(evidence) => Some(Err(evidence)),
            })
            .collect::<Result<Vec<_>>>()?;

        // Get the operation with highest strictly positive delta score, if any.
        Ok(deltas.into_iter().filter(|(_, delta)| *delta > 0.).max_by(
            |(_, delta), (_, delta_star)| delta.partial_cmp(delta_star).unwrap_or(Ordering::Equal),
        ))
    }
}

impl<S> HC<'_, S>
where
    S: ScoringCriterion + Labelled,
{
    /// Execute the HC algorithm.
    ///
    /// # Errors
    ///
    /// * If the scoring criterion fails.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn fit(&self) -> Result<DiGraph> {
        // Initialize the search space, the in-degrees and the current solution.
        let ((mut add, mut del, mut rev), mut in_degree, mut g) = self.init()?;

        // Initialize the iterations counter.
        let mut i = 0;

        // While there are iterations left ...
        while i < self.max_iter {
            // Search for the best operation, if any.
            let Some(((x, y, op), _)) = self.search((&add, &del, &rev), &in_degree, &g)? else {
                // If no strictly positive delta score exists, stop the search.
                break;
            };

            // Apply the operation to the current solution.
            Self::apply(&mut in_degree, &mut g, x, y, op)?;
            // Update the search space.
            (add, del, rev) = Self::update((add, del, rev), x, y, op);

            // Increment the iterations counter.
            i += 1;
        }

        // Return the final graph.
        Ok(g)
    }
}

impl<S> HC<'_, S>
where
    S: ScoringCriterion + Sync + Labelled,
{
    /// Execute the HC algorithm in parallel.
    ///
    /// # Errors
    ///
    /// * If the scoring criterion fails.
    ///
    /// # Returns
    ///
    /// The fitted graph.
    ///
    pub fn par_fit(&self) -> Result<DiGraph> {
        // Initialize the search space, the in-degrees and the current solution.
        let ((mut add, mut del, mut rev), mut in_degree, mut g) = self.init()?;

        // Initialize the iterations counter.
        let mut i = 0;

        // While there are iterations left ...
        while i < self.max_iter {
            // Search for the best operation, if any.
            let Some(((x, y, op), _)) = self.par_search((&add, &del, &rev), &in_degree, &g)? else {
                // If no strictly positive delta score exists, stop the search.
                break;
            };

            // Apply the operation to the current solution.
            Self::apply(&mut in_degree, &mut g, x, y, op)?;
            // Update the search space.
            (add, del, rev) = Self::update((add, del, rev), x, y, op);

            // Increment the iterations counter.
            i += 1;
        }

        // Return the final graph.
        Ok(g)
    }
}

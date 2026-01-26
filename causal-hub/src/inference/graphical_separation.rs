use std::collections::VecDeque;

use crate::{
    models::{DiGraph, Graph},
    set,
    types::{Error, Result, Set},
};

/// A trait for graphical separation.
pub trait GraphicalSeparation {
    /// Checks if the `Z` is a separator set for `X` and `Y`.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of vertices representing set `X`.
    /// * `y` - A set of vertices representing set `Y`.
    /// * `z` - A set of vertices representing set `Z`.
    ///
    /// # Errors
    ///
    /// * `IllegalArgument` if any of the vertex in `X`, `Y`, or `Z` are out of bounds.
    /// * `IllegalArgument` if `X`, `Y` or `Z` are not disjoint sets.
    /// * `IllegalArgument` if `X` and `Y` are empty sets.
    ///
    /// # Returns
    ///
    /// `true` if `X` and `Y` are separated by `Z`, `false` otherwise.
    ///
    fn is_separator_set(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool>;

    /// Checks if the `Z` is a minimal separator set for `X` and `Y`.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of vertices representing set `X`.
    /// * `y` - A set of vertices representing set `Y`.
    /// * `z` - A set of vertices representing set `Z`.
    /// * `w` - An optional iterable collection of vertices representing set `W`.
    /// * `v` - An optional iterable collection of vertices representing set `V`.
    ///
    /// # Errors
    ///
    /// * `IllegalArgument` if any of the vertex in `X`, `Y`, `Z`, `W` or `V` are out of bounds.
    /// * `IllegalArgument` if `X`, `Y` or `Z` are not disjoint sets.
    /// * `IllegalArgument` if `X` and `Y` are empty sets.
    /// * `IllegalArgument` if not `W` <= `Z` <= `V`.
    ///
    /// # Returns
    ///
    /// `true` if `Z` is a minimal separator set for `X` and `Y`, `false` otherwise.
    ///
    fn is_minimal_separator_set(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        w: Option<&Set<usize>>,
        v: Option<&Set<usize>>,
    ) -> Result<bool>;

    /// Finds a minimal separator set for the vertex sets `X` and `Y`, if any.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of vertices representing set `X`.
    /// * `y` - A set of vertices representing set `Y`.
    ///
    /// # Errors
    ///
    /// * `IllegalArgument` if any of the vertex in `X`, `Y`, `W` or `V` are out of bounds.
    /// * `IllegalArgument` if `X` and `Y` are not disjoint sets.
    /// * `IllegalArgument` if `X` or `Y` are empty sets.
    /// * `IllegalArgument` if not `W` <= `V`.
    ///
    /// # Returns
    ///
    /// `Some(Set)` containing the minimal separator set, or `None` if no separator set exists.
    ///
    fn find_minimal_separator_set(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        w: Option<&Set<usize>>,
        v: Option<&Set<usize>>,
    ) -> Result<Option<Set<usize>>>;
}

// Implementation of the `GraphicalSeparation` trait for directed graphs.
pub(crate) mod digraph {
    use super::*;
    use crate::inference::TopologicalOrder;

    /// Asserts the validity of the sets and returns them as `Set<usize>`.
    pub(crate) fn _assert(
        g: &DiGraph,
        x: &Set<usize>,
        y: &Set<usize>,
        z: Option<&Set<usize>>,
        w: Option<&Set<usize>>,
        v: Option<&Set<usize>>,
    ) -> Result<()> {
        // Assert the included set is a subset of the restricted set.
        if let (Some(w), Some(v)) = (w.as_ref(), v.as_ref())
            && !w.is_subset(v)
        {
            return Err(Error::SubsetMismatch("W".into(), "V".into()));
        }

        // Convert X to set, while checking for out of bounds.
        x.iter().try_for_each(|&x| {
            if !g.has_vertex(x) {
                return Err(Error::VertexOutOfBounds(x));
            }
            Ok(())
        })?;
        // Convert Y to set, while checking for out of bounds.
        y.iter().try_for_each(|&y| {
            if !g.has_vertex(y) {
                return Err(Error::VertexOutOfBounds(y));
            }
            Ok(())
        })?;
        // Convert Z to set, while checking for out of bounds.
        if let Some(z) = z {
            z.iter().try_for_each(|&z| {
                if !g.has_vertex(z) {
                    return Err(Error::VertexOutOfBounds(z));
                }
                Ok(())
            })?;
        }

        // Assert X is non-empty.
        if x.is_empty() {
            return Err(Error::EmptySet("X".into()));
        }
        // Assert Y is non-empty.
        if y.is_empty() {
            return Err(Error::EmptySet("Y".into()));
        }

        // Assert X and Y are disjoint.
        if !x.is_disjoint(y) {
            return Err(Error::SetsNotDisjoint("X".into(), "Y".into()));
        }

        // If Z is provided, convert it to a set.
        if let Some(z) = &z {
            // Assert X and Z are disjoint.
            if !x.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("X".into(), "Z".into()));
            }
            // Assert Y and Z are disjoint.
            if !y.is_disjoint(z) {
                return Err(Error::SetsNotDisjoint("Y".into(), "Z".into()));
            }
            // Assert Z includes.
            if let Some(w) = w
                && !z.is_superset(w)
            {
                return Err(Error::SubsetMismatch("W".into(), "Z".into()));
            }
            // Assert Z is restricted.
            if let Some(v) = v
                && !z.is_subset(v)
            {
                return Err(Error::SubsetMismatch("Z".into(), "V".into()));
            }
        }
        Ok(())
    }

    fn _reachable(
        g: &DiGraph,
        x: &Set<usize>,
        an_x: &Set<usize>,
        z: &Set<usize>,
    ) -> Result<Set<usize>> {
        // Assert the graph is a DAG.
        if g.topological_order().is_none() {
            return Err(Error::NotADag);
        }

        // Check if the ball passes or not.
        let _pass = |e: bool, v: usize, f: bool, n: usize| {
            let is_element_of_a = an_x.contains(&n);
            let almost_definite_status = true; // NOTE: Always true for DAGs, not so for RCGs.
            let collider_if_in_z = !z.contains(&v) || (e && !f);
            // If the edge is forward, the vertex must be an ancestor or in Z.
            is_element_of_a && collider_if_in_z && almost_definite_status
        };

        // Initialize the queue.
        let mut queue: VecDeque<(bool, usize)> = Default::default();
        // For each vertex in X, add backward/forward edges to the queue.
        for &w in x.iter() {
            // If the vertex has predecessors, add it to the queue as a backward edge.
            if !g.parents(&set![w])?.is_empty() {
                queue.push_back((false, w));
            }
            // If the vertex has successors, add it to the queue as a forward edge.
            if !g.children(&set![w])?.is_empty() {
                queue.push_back((true, w));
            }
        }

        // Initialize the processed set with the queue.
        let mut visited = queue.clone();

        // For each element in the queue ...
        while let Some((e, v)) = queue.pop_front() {
            // Get the predecessors and successors of the vertex.
            let pa_v = g.parents(&set![v])?.into_iter().map(|n| (false, n));
            let ch_v = g.children(&set![v])?.into_iter().map(|n| (true, n));

            // Create pairs of (forward, vertex) for predecessors and successors.
            // Filter and add unvisited pairs that pass the condition.
            for (f, n) in pa_v.chain(ch_v) {
                if !visited.contains(&(f, n)) && _pass(e, v, f, n) {
                    // Add it to the queue and mark it as processed.
                    queue.push_back((f, n));
                    visited.push_back((f, n));
                }
            }
        }

        // Return the set of visited vertices.
        Ok(visited.into_iter().map(|(_, w)| w).collect())
    }

    impl GraphicalSeparation for DiGraph {
        fn is_separator_set(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> Result<bool> {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, Some(z), None::<&Set<_>>, None::<&Set<_>>)?;

            // Initialize the forward and backward deques and visited sets.

            // Contains -> and <-> edges from starting vertex.
            let mut forward_deque: VecDeque<usize> = Default::default();
            let mut forward_visited: Set<usize> = set![];
            // Contains <- and - edges from starting vertex.
            let mut backward_deque: VecDeque<usize> = Default::default();
            let mut backward_visited: Set<usize> = set![];

            // Initialize the backward deque with the vertices in X.
            backward_deque.extend(x.iter().cloned());

            // Compute the ancestors of X and Z.
            let ancestors_or_z = &self.ancestors(z)? | &(z | x);

            // While there are vertices to visit in the forward or backward deques ...
            while !forward_deque.is_empty() || !backward_deque.is_empty() {
                // If there are vertices in the backward deque ...
                if let Some(w) = backward_deque.pop_front() {
                    // Mark the W as visited.
                    backward_visited.insert(w);
                    // If the W is in Y, return false (not separated).
                    if y.contains(&w) {
                        return Ok(false);
                    }
                    // If the W is in Z, continue to the next iteration.
                    if z.contains(&w) {
                        continue;
                    }
                    // Add all predecessors of the W to the backward deque.
                    self.parents(&set![w])?
                        .into_iter()
                        .filter(|pred| !backward_visited.contains(pred))
                        .for_each(|pred| backward_deque.push_back(pred));
                    // Add all successors of the W to the forward deque.
                    self.children(&set![w])?
                        .into_iter()
                        .filter(|succ| !forward_visited.contains(succ))
                        .for_each(|succ| forward_deque.push_back(succ));
                }

                // If there are vertices in the forward deque ...
                if let Some(w) = forward_deque.pop_front() {
                    // Mark the W as visited.
                    forward_visited.insert(w);
                    // If the W is in Y, return false (not separated).
                    if y.contains(&w) {
                        return Ok(false);
                    }
                    // If the W is an ancestor or in Z, add its predecessors to the backward deque.
                    if ancestors_or_z.contains(&w) {
                        self.parents(&set![w])?
                            .into_iter()
                            .filter(|pred| !backward_visited.contains(pred))
                            .for_each(|pred| backward_deque.push_back(pred));
                    }
                    // If the W is not in Z, add its successors to the forward deque.
                    if !z.contains(&w) {
                        self.children(&set![w])?
                            .into_iter()
                            .filter(|succ| !forward_visited.contains(succ))
                            .for_each(|succ| forward_deque.push_back(succ));
                    }
                }
            }

            // Otherwise, return true.
            Ok(true)
        }

        fn is_minimal_separator_set(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            w: Option<&Set<usize>>,
            v: Option<&Set<usize>>,
        ) -> Result<bool> {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, Some(z), w, v)?;

            // Set default values for W if not provided.
            let w = match w {
                Some(w) => w,
                None => &set![],
            };

            // Compute the ancestors of X and Y.
            let x_y_w = &(x | y) | w;
            let an_x_y_w = &self.ancestors(&x_y_w)? | &x_y_w;

            // a) Check that Z is a separator.
            let x_closure = _reachable(self, x, &an_x_y_w, z)?;
            if !x_closure.is_disjoint(y) {
                return Ok(false);
            }

            // b) Check that Z is constrained to An(X, Y).
            if !z.is_subset(&an_x_y_w) {
                return Ok(false);
            }

            // c) Check that Z is minimal.
            let y_closure = _reachable(self, y, &an_x_y_w, z)?;
            if !((z - w).is_subset(&(&x_closure & &y_closure))) {
                return Ok(false);
            }

            // Otherwise, return true.
            Ok(true)
        }

        fn find_minimal_separator_set(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            w: Option<&Set<usize>>,
            v: Option<&Set<usize>>,
        ) -> Result<Option<Set<usize>>> {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, None::<&Set<_>>, w, v)?;

            // Set default values for W and V if not provided.
            let w = match w {
                Some(w) => w,
                None => &set![],
            };
            let v = match v {
                Some(v) => v,
                None => &self.vertices(),
            };

            // Compute the ancestors of X and Y.
            let x_y_w = &(x | y) | w;
            let an_x_y_w = &self.ancestors(&x_y_w)? | &x_y_w;

            // Initialize the restricted set with the intersection of X, Y, and included.
            let z = v & &(&an_x_y_w - &(x | y));

            // Check if Z is a separator.
            let x_closure = _reachable(self, x, &an_x_y_w, &z)?;
            if !x_closure.is_disjoint(y) {
                return Ok(None); // No minimal separator exists.
            }

            // Update Z.
            let z = &z & &(&x_closure | w);

            // Check if Z is a separator.
            let y_closure = _reachable(self, y, &an_x_y_w, &z)?;

            // Return the minimal separator.
            Ok(Some(&z & &(&y_closure | w)))
        }
    }
}

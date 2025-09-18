use std::collections::VecDeque;

use crate::{
    models::{DiGraph, Graph},
    set,
    types::Set,
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
    /// # Panics
    ///
    /// * If any of the vertex in `X`, `Y`, or `Z` are out of bounds.
    /// * If `X`, `Y` or `Z` are not disjoint sets.
    /// * If `X` and `Y` are empty sets.
    ///
    /// # Returns
    ///
    /// `true` if `X` and `Y` are separated by `Z`, `false` otherwise.
    ///
    fn is_separator_set(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool;

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
    /// # Panics
    ///
    /// * If any of the vertex in `X`, `Y`, `Z`, `W` or `V` are out of bounds.
    /// * If `X`, `Y` or `Z` are not disjoint sets.
    /// * If `X` and `Y` are empty sets.
    /// * If not `W` <= `Z` <= `V`.
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
    ) -> bool;

    /// Finds a minimal separator set for the vertex sets `X` and `Y`, if any.
    ///
    /// # Arguments
    ///
    /// * `x` - A set of vertices representing set `X`.
    /// * `y` - A set of vertices representing set `Y`.
    ///
    /// # Panics
    ///
    /// * If any of the vertex in `X`, `Y`, `W` or `V` are out of bounds.
    /// * If `X` and `Y` are not disjoint sets.
    /// * If `X` or `Y` are empty sets.
    /// * If not `W` <= `V`.
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
    ) -> Option<Set<usize>>;
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
    ) {
        // Assert the included set is a subset of the restricted set.
        if let (Some(w), Some(v)) = (w.as_ref(), v.as_ref()) {
            assert!(w.is_subset(v), "Set W must be a subset of set V.");
        }

        // Convert X to set, while checking for out of bounds.
        for &x in x {
            assert!(g.has_vertex(x), "Vertex `{x}` in set X is out of bounds.");
        }
        // Convert Y to set, while checking for out of bounds.
        for &y in y {
            assert!(g.has_vertex(y), "Vertex `{y}` in set Y is out of bounds.");
        }
        // Convert Z to set, while checking for out of bounds.
        if let Some(z) = z {
            for &z in z {
                assert!(g.has_vertex(z), "Vertex `{z}` in set Z is out of bounds.");
            }
        }

        // Assert X is non-empty.
        assert!(!x.is_empty(), "Set X must not be empty.");
        // Assert Y is non-empty.
        assert!(!y.is_empty(), "Set Y must not be empty.");

        // Assert X and Y are disjoint.
        assert!(x.is_disjoint(y), "Sets X and Y must be disjoint.");

        // If Z is provided, convert it to a set.
        if let Some(z) = &z {
            // Assert X and Z are disjoint.
            assert!(x.is_disjoint(z), "Sets X and Z must be disjoint.");
            // Assert Y and Z are disjoint.
            assert!(y.is_disjoint(z), "Sets Y and Z must be disjoint.");
            // Assert Z includes.
            if let Some(w) = w {
                assert!(z.is_superset(w), "Set Z must be a superset of set W.");
            }
            // Assert Z is restricted.
            if let Some(v) = v {
                assert!(z.is_subset(v), "Set Z must be a subset of set V.");
            }
        }
    }

    fn _reachable(g: &DiGraph, x: &Set<usize>, an_x: &Set<usize>, z: &Set<usize>) -> Set<usize> {
        // Assert the graph is a DAG.
        assert!(g.topological_order().is_some(), "Graph must be a DAG.");

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
        // For each vertex in X ...
        for &w in x {
            // If the vertex has predecessors, add it to the queue as a backward edge.
            if !g.parents(&set![w]).is_empty() {
                queue.push_back((false, w));
            }
            // If the vertex has successors, add it to the queue as a forward edge.
            if !g.children(&set![w]).is_empty() {
                queue.push_back((true, w));
            }
        }

        // Initialize the processed set with the queue.
        let mut visited = queue.clone();

        // For each element in the queue ...
        while let Some((e, v)) = queue.pop_front() {
            // Get the predecessors and successors of the vertex.
            let pa_v = g.parents(&set![v]).into_iter().map(|n| (false, n));
            let ch_v = g.children(&set![v]).into_iter().map(|n| (true, n));

            // Create pairs of (forward, vertex) for predecessors and successors.
            let f_n_pairs = pa_v.chain(ch_v);

            // For each pair ...
            for (f, n) in f_n_pairs {
                // If the pair has not been processed and passes the condition ...
                if !visited.contains(&(f, n)) && _pass(e, v, f, n) {
                    // Add it to the queue and mark it as processed.
                    queue.push_back((f, n));
                    visited.push_back((f, n));
                }
            }
        }

        // Return the set of visited vertices.
        visited.into_iter().map(|(_, w)| w).collect()
    }

    impl GraphicalSeparation for DiGraph {
        fn is_separator_set(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, Some(z), None::<&Set<_>>, None::<&Set<_>>);

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
            let ancestors_or_z = &self.ancestors(z) | &(z | x);

            // While there are vertices to visit in the forward or backward deques ...
            while !forward_deque.is_empty() || !backward_deque.is_empty() {
                // If there are vertices in the backward deque ...
                if let Some(w) = backward_deque.pop_front() {
                    // Mark the W as visited.
                    backward_visited.insert(w);
                    // If the W is in Y, return false (not separated).
                    if y.contains(&w) {
                        return false;
                    }
                    // If the W is in Z, continue to the next iteration.
                    if z.contains(&w) {
                        continue;
                    }
                    // Add all predecessors of the W to the backward deque.
                    for pred in self.parents(&set![w]) {
                        if !backward_visited.contains(&pred) {
                            backward_deque.push_back(pred);
                        }
                    }
                    // Add all successors of the W to the forward deque.
                    for succ in self.children(&set![w]) {
                        if !forward_visited.contains(&succ) {
                            forward_deque.push_back(succ);
                        }
                    }
                }

                // If there are vertices in the forward deque ...
                if let Some(w) = forward_deque.pop_front() {
                    // Mark the W as visited.
                    forward_visited.insert(w);
                    // If the W is in Y, return false (not separated).
                    if y.contains(&w) {
                        return false;
                    }
                    // If the W is an ancestor or in Z, add its predecessors to the backward deque.
                    if ancestors_or_z.contains(&w) {
                        for pred in self.parents(&set![w]) {
                            if !backward_visited.contains(&pred) {
                                backward_deque.push_back(pred);
                            }
                        }
                    }
                    // If the W is not in Z, add its successors to the forward deque.
                    if !z.contains(&w) {
                        for succ in self.children(&set![w]) {
                            if !forward_visited.contains(&succ) {
                                forward_deque.push_back(succ);
                            }
                        }
                    }
                }
            }

            // Otherwise, return true.
            true
        }

        fn is_minimal_separator_set(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            w: Option<&Set<usize>>,
            v: Option<&Set<usize>>,
        ) -> bool {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, Some(z), w, v);

            // Set default values for W if not provided.
            let w = match w {
                Some(w) => w,
                None => &set![],
            };

            // Compute the ancestors of X and Y.
            let x_y_w = &(x | y) | w;
            let an_x_y_w = &self.ancestors(&x_y_w) | &x_y_w;

            // a) Check that Z is a separator.
            let x_closure = _reachable(self, x, &an_x_y_w, z);
            if !x_closure.is_disjoint(y) {
                return false;
            }

            // b) Check that Z is constrained to An(X, Y).
            if !z.is_subset(&an_x_y_w) {
                return false;
            }

            // c) Check that Z is minimal.
            let y_closure = _reachable(self, y, &an_x_y_w, z);
            if !((z - w).is_subset(&(&x_closure & &y_closure))) {
                return false;
            }

            // Otherwise, return true.
            true
        }

        fn find_minimal_separator_set(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            w: Option<&Set<usize>>,
            v: Option<&Set<usize>>,
        ) -> Option<Set<usize>> {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, None::<&Set<_>>, w, v);

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
            let an_x_y_w = &self.ancestors(&x_y_w) | &x_y_w;

            // Initialize the restricted set with the intersection of X, Y, and included.
            let z = v & &(&an_x_y_w - &(x | y));

            // Check if Z is a separator.
            let x_closure = _reachable(self, x, &an_x_y_w, &z);
            if !x_closure.is_disjoint(y) {
                return None; // No minimal separator exists.
            }

            // Update Z.
            let z = &z & &(&x_closure | w);

            // Check if Z is a separator.
            let y_closure = _reachable(self, y, &an_x_y_w, &z);

            // Return the minimal separator.
            Some(&z & &(&y_closure | w))
        }
    }
}

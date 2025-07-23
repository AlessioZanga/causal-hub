use std::collections::VecDeque;

use crate::{
    graphs::{DiGraph, Graph},
    types::Set,
};

/// A trait for graphical separation in graphs.
pub trait GraphicalSeparation {
    /// Checks if the vertex set `Z` is a separator for `X` and `Y`.
    ///
    /// # Arguments
    ///
    /// * `x` - An iterable collection of vertex indices representing set `X`.
    /// * `y` - An iterable collection of vertex indices representing set `Y`.
    /// * `z` - An iterable collection of vertex indices representing set `Z`.
    ///
    /// # Panics
    ///
    /// * If any of the vertex indices in `X`, `Y`, or `Z` are out of bounds.
    /// * If `X`, `Y` or `Z` are not disjoint sets.
    /// * If `X` and `Y` are empty sets.
    ///
    /// # Returns
    ///
    /// `true` if `X` and `Y` are separated by `Z`, `false` otherwise.
    ///
    fn is_separator<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>;

    /// Checks if the vertex set `Z` is a minimal separator for `X` and `Y`.
    ///
    /// # Arguments
    ///
    /// * `x` - An iterable collection of vertex indices representing set `X`.
    /// * `y` - An iterable collection of vertex indices representing set `Y`.
    /// * `z` - An iterable collection of vertex indices representing set `Z`.
    ///
    /// # Panics
    ///
    /// * If any of the vertex indices in `X`, `Y`, or `Z` are out of bounds.
    /// * If `X`, `Y` or `Z` are not disjoint sets.
    /// * If `X` and `Y` are empty sets.
    ///
    /// # Returns
    ///
    /// `true` if `Z` is a minimal separator for `X` and `Y`, `false` otherwise.
    ///
    fn is_minimal_separator<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>;

    /// Finds a minimal separator for the vertex sets `X` and `Y`, if any.
    ///
    /// # Arguments
    ///
    /// * `x` - An iterable collection of vertex indices representing set `X`.
    /// * `y` - An iterable collection of vertex indices representing set `Y`.
    ///
    /// # Panics
    ///
    /// * If any of the vertex indices in `X` or `Y` are out of bounds.
    /// * If `X` and `Y` are not disjoint sets.
    /// * If `X` or `Y` are empty sets.
    ///
    /// # Returns
    ///
    /// `Some(Set)` containing the minimal separator, or `None` if no separator exists.
    ///
    fn find_minimal_separator<I, J>(&self, x: I, y: J) -> Option<Set<usize>>
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>;
}

// Implementation of the `GraphicalSeparation` trait for directed graphs.
mod digraph {
    use super::*;
    use crate::{graphs::TopologicalOrder, set};

    impl GraphicalSeparation for DiGraph {
        fn is_separator<I, J, K>(&self, x: I, y: J, z: K) -> bool
        where
            I: IntoIterator<Item = usize>,
            J: IntoIterator<Item = usize>,
            K: IntoIterator<Item = usize>,
        {
            // Convert X to set, while checking for out of bounds.
            let x: Set<usize> = x
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set X is out of bounds."
                    )
                })
                .collect();
            // Convert Y to set, while checking for out of bounds.
            let y: Set<usize> = y
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set Y is out of bounds."
                    )
                })
                .collect();
            // Convert Z to set, while checking for out of bounds.
            let z: Set<usize> = z
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set Z is out of bounds."
                    )
                })
                .collect();

            // Assert X is non-empty.
            assert!(!x.is_empty(), "Set X must not be empty.");
            // Assert Y is non-empty.
            assert!(!y.is_empty(), "Set Y must not be empty.");

            // Assert X and Y are disjoint.
            assert!(x.is_disjoint(&y), "Sets X and Y must be disjoint.");
            // Assert X and Z are disjoint.
            assert!(x.is_disjoint(&z), "Sets X and Z must be disjoint.");
            // Assert Y and Z are disjoint.
            assert!(y.is_disjoint(&z), "Sets Y and Z must be disjoint.");

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
            let ancestors_or_z: Set<usize> = x
                .iter()
                .flat_map(|&x| self.ancestors(x))
                .chain(z.iter().cloned())
                .chain(x.iter().cloned())
                .collect();

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
                    for pred in self.parents(w) {
                        if !backward_visited.contains(&pred) {
                            backward_deque.push_back(pred);
                        }
                    }
                    // Add all successors of the W to the forward deque.
                    for succ in self.children(w) {
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
                        for pred in self.parents(w) {
                            if !backward_visited.contains(&pred) {
                                backward_deque.push_back(pred);
                            }
                        }
                    }
                    // If the W is not in Z, add its successors to the forward deque.
                    if !z.contains(&w) {
                        for succ in self.children(w) {
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

        fn is_minimal_separator<I, J, K>(&self, x: I, y: J, z: K) -> bool
        where
            I: IntoIterator<Item = usize>,
            J: IntoIterator<Item = usize>,
            K: IntoIterator<Item = usize>,
        {
            // TODO: Allocate included and restricted sets, for future use.
            let included: Set<usize> = set![];
            let restricted: Set<usize> = self.vertices();

            // Convert X to set, while checking for out of bounds.
            let x: Set<usize> = x
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set X is out of bounds."
                    )
                })
                .collect();
            // Convert Y to set, while checking for out of bounds.
            let y: Set<usize> = y
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set Y is out of bounds."
                    )
                })
                .collect();
            // Convert Z to set, while checking for out of bounds.
            let z: Set<usize> = z
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set Z is out of bounds."
                    )
                })
                .collect();

            // Assert X is non-empty.
            assert!(!x.is_empty(), "Set X must not be empty.");
            // Assert Y is non-empty.
            assert!(!y.is_empty(), "Set Y must not be empty.");

            // Assert X and Y are disjoint.
            assert!(x.is_disjoint(&y), "Sets X and Y must be disjoint.");
            // Assert X and Z are disjoint.
            assert!(x.is_disjoint(&z), "Sets X and Z must be disjoint.");
            // Assert Y and Z are disjoint.
            assert!(y.is_disjoint(&z), "Sets Y and Z must be disjoint.");

            // Assert the included set is a subset of the restricted set.
            assert!(
                included.is_subset(&restricted),
                "Included set must be a subset of the restricted set."
            );
            // Assert Z includes.
            assert!(
                z.is_superset(&included),
                "Set Z must be a superset of the included set."
            );
            // Assert Z is restricted.
            assert!(
                z.is_subset(&restricted),
                "Set Z must be a subset of the restricted set."
            );

            // Compute the ancestors of X and Y.
            let x_y_in = &(&x | &y) | &included;
            let an_x_y_in: Set<_> = x_y_in.iter().flat_map(|&v| self.ancestors(v)).collect();
            let an_x_y_in = &an_x_y_in | &x_y_in;

            // a) Check that Z is a separator.
            let x_closure = _reachable(self, &x, &an_x_y_in, &z);
            if !x_closure.is_disjoint(&y) {
                return false;
            }

            // b) Check that Z is constrained to An(X, Y).
            if !z.is_subset(&an_x_y_in) {
                return false;
            }

            // c) Check that Z is minimal.
            let y_closure = _reachable(self, &y, &an_x_y_in, &z);
            if !((&z - &included).is_subset(&(&x_closure & &y_closure))) {
                return false;
            }

            // Otherwise, return true.
            true
        }

        fn find_minimal_separator<I, J>(&self, x: I, y: J) -> Option<Set<usize>>
        where
            I: IntoIterator<Item = usize>,
            J: IntoIterator<Item = usize>,
        {
            // TODO: Allocate included and restricted sets, for future use.
            let included: Set<usize> = set![];
            let restricted: Set<usize> = self.vertices();

            // Convert X to set, while checking for out of bounds.
            let x: Set<usize> = x
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set X is out of bounds."
                    )
                })
                .collect();
            // Convert Y to set, while checking for out of bounds.
            let y: Set<usize> = y
                .into_iter()
                .inspect(|&v| {
                    assert!(
                        self.has_vertex(v),
                        "Vertex `{v}` in set Y is out of bounds."
                    )
                })
                .collect();

            // Assert X is non-empty.
            assert!(!x.is_empty(), "Set X must not be empty.");
            // Assert Y is non-empty.
            assert!(!y.is_empty(), "Set Y must not be empty.");

            // Assert X and Y are disjoint.
            assert!(x.is_disjoint(&y), "Sets X and Y must be disjoint.");

            // Assert the included set is a subset of the restricted set.
            assert!(
                included.is_subset(&restricted),
                "Included set must be a subset of the restricted set."
            );

            // Compute the ancestors of X and Y.
            let x_y_in = &(&x | &y) | &included;
            let an_x_y_in: Set<_> = x_y_in.iter().flat_map(|&v| self.ancestors(v)).collect();
            let an_x_y_in = &an_x_y_in | &x_y_in;

            // Initialize the restricted set with the intersection of X, Y, and included.
            let z: Set<_> = &restricted & &(&an_x_y_in - &(&x | &y));

            // Check if Z is a separator.
            let x_closure = _reachable(self, &x, &an_x_y_in, &z);
            if !x_closure.is_disjoint(&y) {
                return None; // No minimal separator exists.
            }

            // Update Z.
            let z = &z & &(&x_closure | &included);

            // Check if Z is a separator.
            let y_closure = _reachable(self, &y, &an_x_y_in, &z);

            // Return the minimal separator.
            Some(&z & &(&y_closure | &included))
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
            if !g.parents(w).is_empty() {
                queue.push_back((false, w));
            }
            // If the vertex has successors, add it to the queue as a forward edge.
            if !g.children(w).is_empty() {
                queue.push_back((true, w));
            }
        }

        // Initialize the processed set with the queue.
        let mut visited = queue.clone();

        // For each element in the queue ...
        while let Some((e, v)) = queue.pop_front() {
            // Get the predecessors and successors of the vertex.
            let pa_v = g.parents(v).into_iter().map(|n| (false, n));
            let ch_v = g.children(v).into_iter().map(|n| (true, n));

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
}

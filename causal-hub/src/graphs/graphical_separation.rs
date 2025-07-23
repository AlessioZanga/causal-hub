use std::collections::VecDeque;

use crate::{
    graphs::{DiGraph, Graph},
    types::FxIndexSet,
};

/// A trait for graphical separation in graphs.
pub trait GraphicalSeparation {
    /// Checks if the vertex set `X` is separated from `Y` given `Z`.
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
}

impl GraphicalSeparation for DiGraph {
    fn is_separator<I, J, K>(&self, x: I, y: J, z: K) -> bool
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
        K: IntoIterator<Item = usize>,
    {
        // Convert X to set, while checking for out of bounds.
        let x: FxIndexSet<usize> = x
            .into_iter()
            .inspect(|&v| {
                assert!(
                    self.has_vertex(v),
                    "Vertex `{v}` in set X is out of bounds."
                )
            })
            .collect();
        // Convert Y to set, while checking for out of bounds.
        let y: FxIndexSet<usize> = y
            .into_iter()
            .inspect(|&v| {
                assert!(
                    self.has_vertex(v),
                    "Vertex `{v}` in set Y is out of bounds."
                )
            })
            .collect();
        // Convert Z to set, while checking for out of bounds.
        let z: FxIndexSet<usize> = z
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
        let mut forward_visited: FxIndexSet<usize> = Default::default();
        // Contains <- and - edges from starting vertex.
        let mut backward_deque: VecDeque<usize> = Default::default();
        let mut backward_visited: FxIndexSet<usize> = Default::default();

        // Initialize the backward deque with the vertices in X.
        backward_deque.extend(x.iter().cloned());

        // Compute the ancestors of X and Z.
        let ancestors_or_z: FxIndexSet<usize> = x
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
}

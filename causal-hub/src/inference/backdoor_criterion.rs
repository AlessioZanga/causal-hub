use crate::{
    models::{DiGraph, Graph},
    set,
    types::Set,
};

/// A trait for backdoor adjustment criterion.
pub trait BackdoorCriterion {
    /// Checks if the `Z` is a backdoor adjustment set for `X` and `Y`.
    ///
    /// # Arguments
    ///
    /// * `x` - A set vertices representing set `X`.
    /// * `y` - A set vertices representing set `Y`.
    /// * `z` - A set vertices representing set `Z`.
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
    fn is_backdoor_set(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool;

    /// Checks if the `Z` is a minimal backdoor adjustment set for `X` and `Y`.
    ///
    /// # Arguments
    ///
    /// * `x` - A set vertices representing set `X`.
    /// * `y` - A set vertices representing set `Y`.
    /// * `z` - A set vertices representing set `Z`.
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
    /// `true` if `Z` is a minimal backdoor adjustment set for `X` and `Y`, `false` otherwise.
    ///
    fn is_minimal_backdoor_set(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        z: &Set<usize>,
        w: Option<&Set<usize>>,
        v: Option<&Set<usize>>,
    ) -> bool;

    /// Finds a minimal backdoor adjustment set for the vertex sets `X` and `Y`, if any.
    ///
    /// # Arguments
    ///
    /// * `x` - A set vertices representing set `X`.
    /// * `y` - A set vertices representing set `Y`.
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
    /// `Some(Set)` containing the minimal backdoor adjustment set,
    ///  or `None` if no backdoor adjustment set exists.
    ///
    fn find_minimal_backdoor_set(
        &self,
        x: &Set<usize>,
        y: &Set<usize>,
        w: Option<&Set<usize>>,
        v: Option<&Set<usize>>,
    ) -> Option<Set<usize>>;
}

mod digraph {
    use super::*;
    use crate::inference::{GraphicalSeparation, digraph::_assert};

    // Returns the set of vertices:
    //
    //     PCP(X, Y) = { W \in (V \ X) | W is on a *proper possible causal path* from X to Y }
    //
    // where:
    //
    //     * possible causal path in a directed graph is a directed path from X to Y,
    //     * proper path is a directed path from X to Y that does not contain any vertex in X.
    //
    fn _proper_causal_path(g: &DiGraph, x: &Set<usize>, y: &Set<usize>) -> Set<usize> {
        // Initialize the PCP set.
        let mut pcp = set![];

        // Perform a visit starting from each vertex in X.
        for &x_i in x {
            // Initialize stack and visited set.
            let mut stack = vec![x_i];
            let mut visited = set![x_i];

            // While there are vertices to visit ...
            while let Some(z) = stack.pop() {
                // For each child of the current node ...
                for w in g.children(&set![z]) {
                    // Skip if W is in X or already visited.
                    if x.contains(&w) || visited.contains(&w) {
                        continue;
                    }
                    // Set W as visited.
                    visited.insert(w);

                    // Skip if W is in PCP, continue search.
                    if pcp.contains(&w) {
                        continue;
                    }
                    // Add W to the PCP set.
                    pcp.insert(w);

                    // Skip if W is in Y, continue search.
                    if y.contains(&w) {
                        continue;
                    }
                    // Add W to the stack for further exploration.
                    stack.push(w);
                }
            }
        }

        pcp
    }

    // Returns the proper backdoor graph:
    //
    //     G^PDB = G \ { X -> PCP(X, Y) }
    //
    fn _proper_backdoor_graph(g: &DiGraph, x: &Set<usize>, pcp: &Set<usize>) -> DiGraph {
        // Clone the graph.
        let mut g_pdb = g.clone();
        // Remove all the edge from X to PCP(X, Y).
        for &i in x {
            for &j in pcp {
                g_pdb.del_edge(i, j);
            }
        }
        // Return the modified graph.
        g_pdb
    }

    impl BackdoorCriterion for DiGraph {
        fn is_backdoor_set(&self, x: &Set<usize>, y: &Set<usize>, z: &Set<usize>) -> bool {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, Some(z), None::<&Set<_>>, None::<&Set<_>>);

            // Constructive backdoor criterion:
            //
            // Z is a backdoor set for X and Y if and only if:
            //
            //  a) Z <= V \ pDe(PCP(X, Y)), and
            //  b) Z separates X from Y in G^PDB.
            //

            // Compute the proper causal path.
            let pcp = _proper_causal_path(self, x, y);
            // Compute the descendants of the proper causal path.
            let pde = self.descendants(&pcp);
            // a) Check if Z is a subset of V \ pDe(PCP(X, Y)).
            if !z.is_subset(&(&self.vertices() - &pde)) {
                return false;
            }

            // Compute the proper backdoor graph.
            let g_pdb = _proper_backdoor_graph(self, x, &pcp);
            // b) Check if Z separates X from Y in G^PDB.
            if !g_pdb.is_separator_set(x, y, z) {
                return false;
            }

            // Otherwise, return true.
            true
        }

        fn is_minimal_backdoor_set(
            &self,
            x: &Set<usize>,
            y: &Set<usize>,
            z: &Set<usize>,
            w: Option<&Set<usize>>,
            v: Option<&Set<usize>>,
        ) -> bool {
            // Perform sanity checks and convert sets.
            _assert(self, x, y, Some(z), w, v);

            // Set default values for W and V if not provided.
            let w = match w {
                Some(w) => w,
                None => &set![],
            };
            let v = match v {
                Some(v) => v,
                None => &self.vertices(),
            };

            // Every minimal backdoor adjustment set is a
            // minimal separator in the proper backdoor graph
            // G^PDB under the constraint V' = V \ pDe(PCP(X, Y)).

            // Compute the proper causal path.
            let pcp = _proper_causal_path(self, x, y);
            // Compute the descendants of the proper causal path.
            let pde = self.descendants(&pcp);
            // Constraint the restricted vertices.
            let v_prime = &(v - &pde);

            // Compute the proper backdoor graph.
            let g_pdb = _proper_backdoor_graph(self, x, &pcp);

            // Check if Z is a minimal separator in G^PDB under the constraint V'.
            g_pdb.is_minimal_separator_set(x, y, z, Some(w), Some(v_prime))
        }

        fn find_minimal_backdoor_set(
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

            // Every minimal backdoor adjustment set is a
            // minimal separator in the proper backdoor graph
            // G^PDB under the constraint V' = V \ pDe(PCP(X, Y)).

            // Compute the proper causal path.
            let pcp = _proper_causal_path(self, x, y);
            // Compute the descendants of the proper causal path.
            let pde = self.descendants(&pcp);
            // Constraint the restricted vertices.
            let v_prime = &(v - &pde);

            // Compute the proper backdoor graph.
            let g_pdb = _proper_backdoor_graph(self, x, &pcp);

            // Find a minimal separator in G^PDB under the constraint V'.
            g_pdb.find_minimal_separator_set(x, y, Some(w), Some(v_prime))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn proper_causal_path() {
            let mut graph = DiGraph::empty(vec!["A", "B", "C", "D", "E"]);
            graph.add_edge(0, 1);
            graph.add_edge(0, 2);
            graph.add_edge(1, 2);
            graph.add_edge(1, 3);
            graph.add_edge(2, 3);
            graph.add_edge(3, 4);

            assert_eq!(
                _proper_causal_path(&graph, &set![0], &set![3]),
                set![1, 2, 3]
            );
        }
    }
}

use itertools::Itertools;

use crate::{
    models::{DiGraph, Graph},
    set,
    types::Result,
};

/// A trait for v-structures.
pub trait VStructures {
    /// Returns the v-structures of the graph.
    ///
    /// # Returns
    ///
    /// An iterator over the v-structures of the graph.
    ///
    fn v_structures(&self) -> Result<Vec<(usize, usize, usize)>>;
}

impl VStructures for DiGraph {
    fn v_structures(&self) -> Result<Vec<(usize, usize, usize)>> {
        // Initialize the v-structures list.
        let mut v_structs = Vec::new();

        // For each vertex z in the graph ...
        for &z in &self.vertices() {
            // ... get its parents.
            let pa_z = self.parents(&set![z])?;
            // If the vertex has at least two parents ...
            if pa_z.len() >= 2 {
                // ... for each pair of parents (x, y) ...
                for x_y in pa_z.iter().copied().combinations(2) {
                    let (x, y) = (x_y[0], x_y[1]);
                    // ... if x and y are not connected ...
                    if !self.has_edge(x, y)? && !self.has_edge(y, x)? {
                        // ... then (x, z, y) is a v-structure.
                        v_structs.push((x, z, y));
                    }
                }
            }
        }

        Ok(v_structs)
    }
}

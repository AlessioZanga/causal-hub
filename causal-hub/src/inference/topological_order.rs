use std::collections::VecDeque;

use ndarray::prelude::*;

use crate::{
    models::{DiGraph, Graph},
    set,
};

/// Topological sort trait.
pub trait TopologicalOrder {
    /// Returns the topological sort of the graph.
    ///
    /// # Returns
    ///
    /// A vector of vertex indices in topological order,
    /// or `None` if the order does not exists.
    ///
    fn topological_order(&self) -> Option<Vec<usize>>;
}

impl TopologicalOrder for DiGraph {
    fn topological_order(&self) -> Option<Vec<usize>> {
        // Compute the in-degrees of the vertices.
        let mut in_degree = self
            .to_adjacency_matrix()
            .mapv(|x| x as usize)
            .sum_axis(Axis(0));
        // Initialize queue with vertices having in-degree 0
        let mut to_be_visited: VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| if d == 0 { Some(i) } else { None })
            .collect();

        // Initialize the order vector.
        let mut order = Vec::with_capacity(in_degree.len());
        // While there are vertices to be visited ...
        while let Some(i) = to_be_visited.pop_front() {
            // Add the vertex to the order.
            order.push(i);
            // For each neighbor, reduce its in-degree.
            self.children(&set![i]).into_iter().for_each(|y| {
                // Decrement the in-degree of the child.
                in_degree[y] -= 1;
                // If the in-degree becomes 0, add it to the queue.
                if in_degree[y] == 0 {
                    to_be_visited.push_back(y);
                }
            });
        }

        // Check if the order contains all vertices.
        if in_degree.len() == order.len() {
            Some(order)
        } else {
            None // The graph is not a DAG.
        }
    }
}

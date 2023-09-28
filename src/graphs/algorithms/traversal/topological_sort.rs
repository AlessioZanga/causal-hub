use std::collections::{HashMap, VecDeque};

use crate::{graphs::DirectedGraph, Ch, V};

pub struct TopologicalSort<'a, G>
where
    G: DirectedGraph,
{
    g: &'a G,
    // To-be-visited queue.
    queue: VecDeque<usize>,
    // Visit map with vertices in-degrees.
    visit: HashMap<usize, usize>,
}

impl<'a, G> TopologicalSort<'a, G>
where
    G: DirectedGraph,
{
    pub fn new(g: &'a G) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            g,
            // Initialize the to-be-visited queue with the source vertex.
            queue: Default::default(),
            // Initialize the visit map.
            visit: Default::default(),
        };
        // For each vertex in the graph.
        for x in V!(search.g) {
            // Compute its in-degree.
            match search.g.in_degree(x) {
                // If the in-degree is zero, then add it to the queue.
                0 => search.queue.push_back(x),
                // Otherwise, add it to the map.
                y => {
                    search.visit.insert(x, y);
                }
            }
        }

        search
    }
}

impl<'a, G> Iterator for TopologicalSort<'a, G>
where
    G: DirectedGraph,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // While there are still vertices with zero in-degree.
        if let Some(x) = self.queue.pop_front() {
            // For each child of the selected vertex.
            for y in Ch!(self.g, x) {
                // If it was not visited before.
                if let Some(z) = self.visit.get(&y) {
                    // Update its in-degree.
                    match z - 1 {
                        // If the in-degree is zero ...
                        0 => {
                            // ... then add it to the queue ...
                            self.queue.push_back(y);
                            // ... and remove it from the visit map.
                            self.visit.remove(&y);
                        }
                        // Otherwise, update its in-degree into the map.
                        z => {
                            self.visit.insert(y, z);
                        }
                    }
                }
            }
            // Return current vertex.
            return Some(x);
        }

        // If there are still vertices with non-zero in-degree ...
        if !self.visit.is_empty() {
            // ... no topological sort is defined, i.e. cyclic graph.
            panic!("No topological sort is defined, i.e. cyclic graph");
        }

        None
    }
}

impl<'a, G> From<&'a G> for TopologicalSort<'a, G>
where
    G: DirectedGraph,
{
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}

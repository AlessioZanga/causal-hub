use std::collections::BTreeSet;

use crate::{
    graphs::{directions, BaseGraph, UndirectedGraph},
    prelude::BFS,
    V,
};

/// Connected components structure.
pub struct ConnectedComponents<'a, G>
where
    G: BaseGraph<Direction = directions::Undirected> + UndirectedGraph,
{
    g: &'a G,
    queue: BTreeSet<usize>,
}

impl<'a, G> ConnectedComponents<'a, G>
where
    G: BaseGraph<Direction = directions::Undirected> + UndirectedGraph,
{
    /// Build a CC iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// // FIXME:
    /// ```
    ///
    pub fn new(g: &'a G) -> Self {
        // Initialize to-be-visited queue.
        let queue = V!(g).collect();

        Self { g, queue }
    }
}

impl<'a, G> Iterator for ConnectedComponents<'a, G>
where
    G: BaseGraph<Direction = directions::Undirected> + UndirectedGraph,
{
    type Item = BTreeSet<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if there is still a vertex to be visited.
        self.queue.pop_first().map(|x| {
            // Perform BFS Tree visit starting from the vertex.
            let component = BFS::from((self.g, x)).collect();
            // Remove visited vertices from the to-be-visited set.
            self.queue = &self.queue - &component;

            component
        })
    }
}

impl<'a, G> From<&'a G> for ConnectedComponents<'a, G>
where
    G: BaseGraph<Direction = directions::Undirected> + UndirectedGraph,
{
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}

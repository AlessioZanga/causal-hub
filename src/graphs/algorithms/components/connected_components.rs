use std::collections::VecDeque;

use crate::{
    graphs::{directions, UndirectedGraph},
    prelude::BFS,
    V,
};

pub struct ConnectedComponents<'a, G>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    g: &'a G,
    queue: VecDeque<usize>,
}

impl<'a, G> ConnectedComponents<'a, G>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    pub fn new(g: &'a G) -> Self {
        // Initialize to-be-visited queue.
        let queue = V!(g).collect();

        Self { g, queue }
    }
}

impl<'a, G> Iterator for ConnectedComponents<'a, G>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if there is still a vertex to be visited.
        self.queue.pop_front().map(|x| {
            // Perform BFS Tree visit starting from the vertex.
            let component = BFS::from((self.g, x)).collect();
            // Remove visited vertices from the to-be-visited set.
            self.queue = iter_set::difference(&self.queue, &component)
                .cloned()
                .collect();

            component
        })
    }
}

impl<'a, G> From<&'a G> for ConnectedComponents<'a, G>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    fn from(g: &'a G) -> Self {
        Self::new(g)
    }
}

pub type CC<'a, G> = ConnectedComponents<'a, G>;

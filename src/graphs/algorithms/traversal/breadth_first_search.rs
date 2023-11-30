use std::{collections::VecDeque, iter::FusedIterator};

use super::Traversal;
use crate::{
    graphs::{
        Directed, DirectedGraph, Graph, PartiallyDirected, PartiallyDirectedGraph, Undirected,
        UndirectedGraph,
    },
    Ch, Ne, V,
};

pub struct BreadthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
    g: &'a G,

    vertices: VecDeque<usize>,

    queue: VecDeque<usize>,

    pub distance: Vec<usize>,

    pub predecessor: Vec<usize>,
}

impl<'a, G, D> BreadthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
    pub fn new(g: &'a G, x: Option<usize>, m: Traversal) -> Self {
        // Get graph order.
        let order = g.order();
        // Initialize the [`Forest`] to-be-visited queue.
        let mut vertices = VecDeque::default();
        // Initialize the to-be-visited queue with the source vertex.
        let mut queue = VecDeque::with_capacity(order);
        // Initialize the distance map.
        let mut distance = vec![usize::MAX; order];
        // Initialize the predecessor map.
        let predecessor = vec![usize::MAX; order];

        // If visit variant is [`Forest`] ...
        if matches!(m, Traversal::Forest) {
            // ... fill the vertices queue.
            vertices.extend(V!(g));
        }

        // If no source vertex is given, choose the first in the vertex set.
        if let Some(x) = x.or_else(|| V!(g).next()) {
            // ... assert that source vertex is in graph.
            assert!(g.has_vertex(x));
            // Push the source vertex into the queue.
            queue.push_front(x);
            // Set its distance to zero.
            distance[x] = 0;
        };

        Self {
            g,
            vertices,
            queue,
            distance,
            predecessor,
        }
    }
}

impl<'a, G, D> From<&'a G> for BreadthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
    #[inline]
    fn from(g: &'a G) -> Self {
        Self::new(g, None, Traversal::Tree)
    }
}

impl<'a, G, D> From<(&'a G, usize)> for BreadthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
    #[inline]
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x), Traversal::Tree)
    }
}

impl<'a, G> Iterator for BreadthFirstSearch<'a, G, Undirected>
where
    G: UndirectedGraph<Direction = Undirected>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current algorithm is set to Forest
        // and there are no more vertices in the queue ...
        if self.queue.is_empty() {
            // ... but there is at least one vertex ...
            while let Some(x) = self.vertices.pop_front() {
                // ... that was not visited.
                if self.distance[x] == usize::MAX {
                    // Push such vertex into the visiting queue.
                    self.queue.push_front(x);
                    // Set the distance of new root.
                    self.distance[x] = 0;
                    // Continue search visit.
                    break;
                }
            }
        }
        // If there are still vertices to be visited.
        self.queue.pop_front().map(|x| {
            // Get predecessor distance.
            let distance_x = self.distance[x];
            // Iterate over the reachable vertices of the popped vertex.
            for y in Ne!(self.g, x) {
                // If the vertex was never seen before.
                if self.distance[y] == usize::MAX {
                    // Push it into the to-be-visited queue.
                    self.queue.push_back(y);
                    // Compute the distance from its predecessor.
                    self.distance[y] = distance_x + 1;
                    // Set its predecessor.
                    self.predecessor[y] = x;
                }
            }

            x
        })
    }
}

impl<'a, G> FusedIterator for BreadthFirstSearch<'a, G, Undirected> where
    G: UndirectedGraph<Direction = Undirected>
{
}

impl<'a, G> Iterator for BreadthFirstSearch<'a, G, Directed>
where
    G: DirectedGraph<Direction = Directed>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current algorithm is set to Forest
        // and there are no more vertices in the queue ...
        if self.queue.is_empty() {
            // ... but there is at least one vertex ...
            while let Some(x) = self.vertices.pop_front() {
                // ... that was not visited.
                if self.distance[x] == usize::MAX {
                    // Push such vertex into the visiting queue.
                    self.queue.push_front(x);
                    // Set the distance of new root.
                    self.distance[x] = 0;
                    // Continue search visit.
                    break;
                }
            }
        }
        // If there are still vertices to be visited.
        self.queue.pop_front().map(|x| {
            // Get predecessor distance.
            let distance_x = self.distance[x];
            // Iterate over the reachable vertices of the popped vertex.
            for y in Ch!(self.g, x) {
                // If the vertex was never seen before.
                if self.distance[y] == usize::MAX {
                    // Push it into the to-be-visited queue.
                    self.queue.push_back(y);
                    // Compute the distance from its predecessor.
                    self.distance[y] = distance_x + 1;
                    // Set its predecessor.
                    self.predecessor[y] = x;
                }
            }

            x
        })
    }
}

impl<'a, G> FusedIterator for BreadthFirstSearch<'a, G, Directed> where
    G: DirectedGraph<Direction = Directed>
{
}

impl<'a, G> Iterator for BreadthFirstSearch<'a, G, PartiallyDirected>
where
    G: PartiallyDirectedGraph<Direction = PartiallyDirected>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current algorithm is set to Forest
        // and there are no more vertices in the queue ...
        if self.queue.is_empty() {
            // ... but there is at least one vertex ...
            while let Some(x) = self.vertices.pop_front() {
                // ... that was not visited.
                if self.distance[x] == usize::MAX {
                    // Push such vertex into the visiting queue.
                    self.queue.push_front(x);
                    // Set the distance of new root.
                    self.distance[x] = 0;
                    // Continue search visit.
                    break;
                }
            }
        }
        // If there are still vertices to be visited.
        self.queue.pop_front().map(|x| {
            // Get predecessor distance.
            let distance_x = self.distance[x];
            // Iterate over the reachable vertices of the popped vertex.
            for y in iter_set::union(Ne!(self.g, x), Ch!(self.g, x)) {
                // If the vertex was never seen before.
                if self.distance[y] == usize::MAX {
                    // Push it into the to-be-visited queue.
                    self.queue.push_back(y);
                    // Compute the distance from its predecessor.
                    self.distance[y] = distance_x + 1;
                    // Set its predecessor.
                    self.predecessor[y] = x;
                }
            }

            x
        })
    }
}

impl<'a, G> FusedIterator for BreadthFirstSearch<'a, G, PartiallyDirected> where
    G: PartiallyDirectedGraph<Direction = PartiallyDirected>
{
}

pub type BFS<'a, G, D> = BreadthFirstSearch<'a, G, D>;

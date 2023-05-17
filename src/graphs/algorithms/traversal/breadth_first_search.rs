use std::{collections::VecDeque, iter::FusedIterator};

use super::Traversal;
use crate::{
    graphs::{directions, BaseGraph, DirectedGraph, PartiallyDirectedGraph, UndirectedGraph},
    Ch, Ne, V,
};

/// Breadth-first search structure.
///
/// This structure contains the `distance` and `predecessor` maps.
///
/// If the algorithm is set to the [`Forest`](super::Traversal) variant, a vertex with distance
/// [`usize::MAX`] means that such vertex is not reachable from the given
/// source vertex (i.e. the graph is disconnected).
///
pub struct BreadthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// To-be-visited queue for the [`Forest`](super::Traversal) variant.
    vertices: VecDeque<usize>,
    /// To-be-visited queue with the source vertex.
    queue: VecDeque<usize>,
    /// Distance from the source vertex.
    pub distance: Vec<usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: Vec<usize>,
}

impl<'a, G, D> BreadthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Build a BFS iterator.
    ///
    /// Build a BFS iterator for a given graph. This will execute the [`Tree`](super::Traversal)
    /// variant of the algorithm, if not specified otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the (optional) source vertex is not in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use causal_hub::prelude::*;
    ///
    /// // Define edge set.
    /// let e = EdgeList::from([
    ///     ("A", "B"), ("B", "C"), ("A", "D"), ("A", "E"), ("E", "F")
    /// ]);
    ///
    /// // Build a new graph.
    /// let mut g = DiGraph::from(e);
    ///
    /// // Build the search object over graph with `0` as source vertex.
    /// let mut search = BFS::from((&g, 0));
    ///
    /// // Collect visit order by reference, preserving search structure.
    /// let order: Vec<_> = search.by_ref().collect();
    /// // The visit returns a pre-order.
    /// assert_eq!(order, [0, 1, 3, 4, 2, 5]);
    ///
    /// // The source vertex has distance zero from itself ...
    /// assert_eq!(search.distance[0], 0);
    /// // ... and no predecessor by definition.
    /// assert_eq!(search.predecessor[0], usize::MAX);
    ///
    /// // For example, vertex `5` has distance two from `0` ...
    /// assert_eq!(search.distance[5], 2);
    /// // ... and its predecessor is `4`.
    /// assert_eq!(search.predecessor[5], 4);
    /// ```
    ///
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
            assert!(g.has_vertex_by_index(x));
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
    G: BaseGraph<Direction = D>,
{
    #[inline]
    fn from(g: &'a G) -> Self {
        Self::new(g, None, Traversal::Tree)
    }
}

impl<'a, G, D> From<(&'a G, usize)> for BreadthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    #[inline]
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x), Traversal::Tree)
    }
}

impl<'a, G> Iterator for BreadthFirstSearch<'a, G, directions::Undirected>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
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

impl<'a, G> FusedIterator for BreadthFirstSearch<'a, G, directions::Undirected> where
    G: UndirectedGraph<Direction = directions::Undirected>
{
}

impl<'a, G> Iterator for BreadthFirstSearch<'a, G, directions::Directed>
where
    G: DirectedGraph<Direction = directions::Directed>,
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

impl<'a, G> FusedIterator for BreadthFirstSearch<'a, G, directions::Directed> where
    G: DirectedGraph<Direction = directions::Directed>
{
}

impl<'a, G> Iterator for BreadthFirstSearch<'a, G, directions::PartiallyDirected>
where
    G: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
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

impl<'a, G> FusedIterator for BreadthFirstSearch<'a, G, directions::PartiallyDirected> where
    G: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>
{
}

/// Alias for breadth-first search.
pub type BFS<'a, G, D> = BreadthFirstSearch<'a, G, D>;

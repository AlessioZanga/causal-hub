use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    vec::Vec,
};

use super::Traversal;
use crate::{
    graphs::{directions, BaseGraph, DirectedGraph, PartiallyDirectedGraph, UndirectedGraph},
    Ch, Ne, V,
};

/// Depth-first search structure.
///
/// This structure contains the `discovery_time`, `finish_time` and `predecessor` maps.
///
pub struct DepthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// The visit stack.
    stack: Vec<usize>,
    /// Global time counter.
    pub time: usize,
    /// Discovery time of each discovered vertex.
    pub discovery_time: HashMap<usize, usize>,
    /// Finish time of each discovered vertex.
    pub finish_time: HashMap<usize, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<usize, usize>,
}

impl<'a, G, D> DepthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Build a DFS iterator.
    ///
    /// Build a DFS iterator for a given graph. This will execute the [`Tree`](super::Traversal)
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
    /// // Build the search object over said graph with `0` as source vertex.
    /// let mut search = DFS::from((&g, 0));
    ///
    /// // Collect visit order by reference, preserving search structure.
    /// let order: Vec<_> = search.by_ref().collect();
    /// // The visit returns a pre-order.
    /// assert_eq!(order, [0, 1, 2, 3, 4, 5]);
    ///
    /// // The source vertex has discovery-time equals to zero ...
    /// assert_eq!(search.discovery_time[&0], 0);
    /// // ... finish-time equals to two times the number of discovered vertices minus one ...
    /// assert_eq!(search.finish_time[&0], 2 * search.discovery_time.len() - 1);
    /// // ... and no predecessor by definition.
    /// assert_eq!(search.predecessor.contains_key(&0), false);
    ///
    /// // For example, vertex `5` has discovery-time equals to eight ...
    /// assert_eq!(search.discovery_time[&5], 8);
    /// // ... finish-time equals to nine ...
    /// assert_eq!(search.finish_time[&5], 9);
    /// // ... and its predecessor is `4`.
    /// assert_eq!(search.predecessor[&5], 4);
    /// ```
    ///
    pub fn new(g: &'a G, x: Option<usize>, m: Traversal) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            g,
            // Initialize the to-be-visited queue with the source vertex.
            stack: Default::default(),
            // Initialize the global clock.
            time: Default::default(),
            // Initialize the discovery-time map.
            discovery_time: Default::default(),
            // Initialize the finish-time map.
            finish_time: Default::default(),
            // Initialize the predecessor map.
            predecessor: Default::default(),
        };
        // If the graph is null.
        if g.order() == 0 {
            // Assert source vertex is none.
            assert!(x.is_none());
            // Then, return the default search object.
            return search;
        }
        // Get source vertex, if any.
        let x = match x {
            // If no source vertex is given, choose the first one in the vertex set.
            None => V!(g).next().unwrap(),
            // Otherwise ...
            Some(x) => {
                // ... assert that source vertex is in graph.
                assert!(g.has_vertex_by_index(x));
                // Return given source vertex.
                x
            }
        };
        // If visit variant is Forest.
        if matches!(m, Traversal::Forest) {
            // Add vertices to the visit stack in reverse to preserve order.
            let mut queue = VecDeque::<usize>::with_capacity(g.order());
            queue.extend(V!(g).filter(|&y| y != x));
            search.stack.extend(queue.iter().rev());
        }
        // Push source vertex onto the stack.
        search.stack.push(x);
        // Return search object.
        search
    }
}

impl<'a, G> Iterator for DepthFirstSearch<'a, G, directions::Undirected>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&x) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if let Entry::Vacant(e) = self.discovery_time.entry(x) {
                // Set its discover time (as GRAY).
                e.insert(self.time);
                // Increment time.
                self.time += 1;
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for y in Ne!(self.g, x) {
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(&y) {
                        // Set predecessor.
                        self.predecessor.insert(y, x);
                        // Add to queue.
                        queue.push_front(y);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);
                // Return vertex in pre-order.
                return Some(x);
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if it is GRAY (not BLACK).
                if let Entry::Vacant(e) = self.finish_time.entry(x) {
                    // Set its finish time (as BLACK).
                    e.insert(self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
        }

        None
    }
}

impl<'a, G> Iterator for DepthFirstSearch<'a, G, directions::Directed>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&x) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if let Entry::Vacant(e) = self.discovery_time.entry(x) {
                // Set its discover time (as GRAY).
                e.insert(self.time);
                // Increment time.
                self.time += 1;
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for y in Ch!(self.g, x) {
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(&y) {
                        // Set predecessor.
                        self.predecessor.insert(y, x);
                        // Add to queue.
                        queue.push_front(y);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);
                // Return vertex in pre-order.
                return Some(x);
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if it is GRAY (not BLACK).
                if let Entry::Vacant(e) = self.finish_time.entry(x) {
                    // Set its finish time (as BLACK).
                    e.insert(self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
        }

        None
    }
}

impl<'a, G> Iterator for DepthFirstSearch<'a, G, directions::PartiallyDirected>
where
    G: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&x) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if let Entry::Vacant(e) = self.discovery_time.entry(x) {
                // Set its discover time (as GRAY).
                e.insert(self.time);
                // Increment time.
                self.time += 1;
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for y in iter_set::union(Ne!(self.g, x), Ch!(self.g, x)) {
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(&y) {
                        // Set predecessor.
                        self.predecessor.insert(y, x);
                        // Add to queue.
                        queue.push_front(y);
                    }
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);
                // Return vertex in pre-order.
                return Some(x);
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if it is GRAY (not BLACK).
                if let Entry::Vacant(e) = self.finish_time.entry(x) {
                    // Set its finish time (as BLACK).
                    e.insert(self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
        }

        None
    }
}

impl<'a, G, D> From<&'a G> for DepthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    fn from(g: &'a G) -> Self {
        Self::new(g, None, Traversal::Tree)
    }
}

impl<'a, G, D> From<(&'a G, usize)> for DepthFirstSearch<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x), Traversal::Tree)
    }
}

/// Alias for depth-first search.
pub type DFS<'a, G, D> = DepthFirstSearch<'a, G, D>;

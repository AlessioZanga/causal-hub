use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    vec::Vec,
};

use super::Traversal;
use crate::{
    graphs::{
        Directed, DirectedGraph, Graph, PartiallyDirected, PartiallyDirectedGraph, Undirected,
        UndirectedGraph,
    },
    Ch, Ne, V,
};

pub struct DepthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
    g: &'a G,

    stack: Vec<usize>,

    pub time: usize,

    pub discovery_time: HashMap<usize, usize>,

    pub finish_time: HashMap<usize, usize>,

    pub predecessor: HashMap<usize, usize>,
}

impl<'a, G, D> DepthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
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
                assert!(g.has_vertex(x));
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

impl<'a, G> Iterator for DepthFirstSearch<'a, G, Undirected>
where
    G: UndirectedGraph<Direction = Undirected>,
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

impl<'a, G> Iterator for DepthFirstSearch<'a, G, Directed>
where
    G: DirectedGraph<Direction = Directed>,
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

impl<'a, G> Iterator for DepthFirstSearch<'a, G, PartiallyDirected>
where
    G: PartiallyDirectedGraph<Direction = PartiallyDirected>,
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
    G: Graph<Direction = D>,
{
    fn from(g: &'a G) -> Self {
        Self::new(g, None, Traversal::Tree)
    }
}

impl<'a, G, D> From<(&'a G, usize)> for DepthFirstSearch<'a, G, D>
where
    G: Graph<Direction = D>,
{
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x), Traversal::Tree)
    }
}

pub type DFS<'a, G, D> = DepthFirstSearch<'a, G, D>;

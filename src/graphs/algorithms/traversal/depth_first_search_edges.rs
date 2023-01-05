use std::collections::{hash_map::Entry, HashMap, VecDeque};

use super::Traversal;
use crate::{
    graphs::{directions, BaseGraph, DirectedGraph, UndirectedGraph},
    Ch, Ne, V,
};

/// Edge classification performed by the [depth first search edges](`DepthFirstSearchEdges`) algorithm.
#[derive(Debug, PartialEq, Eq)]
pub enum DFSEdge {
    /// From a vertex to an ancestor.
    Back(usize, usize),
    /// From a vertex to another, which is not an ancestor nor a descendant.
    Cross(usize, usize),
    /// From a vertex to a descendant, which is not a child.
    Forward(usize, usize),
    /// From a vertex to a child.
    Tree(usize, usize),
}

/// Depth-first search-edges structure.
///
/// This structure contains the `discovery_time`, `finish_time` and `predecessor` maps.
///
pub struct DepthFirstSearchEdges<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Given graph reference.
    g: &'a G,
    /// The visit stack.
    stack: Vec<(usize, usize)>,
    /// Global time counter.
    pub time: usize,
    /// Discovery time of each discovered vertex.
    pub discovery_time: HashMap<usize, usize>,
    /// Finish time of each discovered vertex.
    pub finish_time: HashMap<usize, usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: HashMap<usize, usize>,
}

impl<'a, G, D> DepthFirstSearchEdges<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    /// Build a DFS-Edges iterator.
    ///
    /// Build a DFS-Edges iterator for a given graph. This will execute the [`Tree`](super::Traversal)
    /// variant of the algorithm, if not specified otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the (optional) source vertex is not in the graph.
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
                assert!(g.has_vertex(x));
                // Return given source vertex.
                x
            }
        };
        // If visit variant is Forest.
        if matches!(m, Traversal::Forest) {
            // Add vertices to the visit stack in reverse to preserve order.
            let mut queue = VecDeque::with_capacity(g.order());
            queue.extend(V!(g).filter(|&y| y != x).map(|y| (y, y)));
            search.stack.extend(queue.iter().rev());
        }
        // Push source vertex onto the stack.
        search.stack.push((x, x));
        // Return search object.
        search
    }
}

impl<'a, G> Iterator for DepthFirstSearchEdges<'a, G, directions::Undirected>
where
    G: BaseGraph<Direction = directions::Undirected> + UndirectedGraph,
{
    type Item = DFSEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&(x, y)) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if let Entry::Vacant(e) = self.discovery_time.entry(y) {
                // Set its discover time (as GRAY).
                e.insert(self.time);
                // Increment time.
                self.time += 1;
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for z in Ne!(self.g, y) {
                    // Filter incoming edge. TODO: Simplify this.
                    if self.predecessor.get(&y) == Some(&z) {
                        continue;
                    }
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(&z) {
                        // Set predecessor.
                        self.predecessor.insert(z, y);
                    }
                    // Add to queue.
                    queue.push_front((y, z));
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);
                // Filter the base case. TODO: Simplify this. Base case-related.
                if x != y && self.predecessor.contains_key(&y) {
                    // discovery[x] < discovery[y] && finish[x] < finish[y].
                    return Some(DFSEdge::Tree(x, y));
                }
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if current vertex can be GRAY. TODO: Simplify this. Base case-related.
                let flag = self.predecessor.get(&y);
                // Check if it is GRAY (not BLACK). TODO: Simplify this. Base case-related.
                if (flag.is_none() || flag == Some(&x)) && !self.finish_time.contains_key(&y) {
                    // Set its finish time (as BLACK).
                    self.finish_time.insert(y, self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
            // Classify the incoming edge w.r.t. the timings.
            // NOTE: Given that the graph is undirected, there are no forward nor cross edges.
            if self.discovery_time[&x] > self.discovery_time[&y]
                && self.discovery_time[&x] < *self.finish_time.get(&y).unwrap_or(&0)
            {
                // discovery[x] > discovery[y] && discovery[x] < finish[y].
                return Some(DFSEdge::Back(x, y));
            }
        }

        None
    }
}

impl<'a, G> Iterator for DepthFirstSearchEdges<'a, G, directions::Directed>
where
    G: BaseGraph<Direction = directions::Directed> + DirectedGraph,
{
    type Item = DFSEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&(x, y)) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if let Entry::Vacant(e) = self.discovery_time.entry(y) {
                // Set its discover time (as GRAY).
                e.insert(self.time);
                // Increment time.
                self.time += 1;
                // Initialize visiting queue.
                let mut queue = VecDeque::new();
                // Iterate over reachable vertices.
                for z in Ch!(self.g, y) {
                    // Filter already visited vertices (as GRAY).
                    if !self.discovery_time.contains_key(&z) {
                        // Set predecessor.
                        self.predecessor.insert(z, y);
                    }
                    // Add to queue.
                    queue.push_front((y, z));
                }
                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);
                // Filter the base case. TODO: Simplify this. Base case-related.
                if x != y && self.predecessor.contains_key(&y) {
                    // discovery[x] < discovery[y] && discovery[x] > finish[y].
                    return Some(DFSEdge::Tree(x, y));
                }
            // If the vertex is NOT WHITE.
            } else {
                // Remove it from stack.
                self.stack.pop();
                // Check if current vertex can be GRAY. TODO: Simplify this. Base case-related.
                let flag = self.predecessor.get(&y);
                // Check if it is GRAY (not BLACK). TODO: Simplify this. Base case-related.
                if (flag.is_none() || flag == Some(&x)) && !self.finish_time.contains_key(&y) {
                    // Set its finish time (as BLACK).
                    self.finish_time.insert(y, self.time);
                    // Increment time.
                    self.time += 1;
                }
            }
            // Classify the incoming edge w.r.t. the timings.
            if self.discovery_time[&x] > self.discovery_time[&y] {
                if self.discovery_time[&x] < *self.finish_time.get(&y).unwrap_or(&0) {
                    // discovery[x] > discovery[y] && discovery[x] < finish[y], or ...
                    return Some(DFSEdge::Back(x, y));
                } else {
                    // discovery[x] > discovery[y] && discovery[x] > finish[y], or ...
                    return Some(DFSEdge::Cross(x, y));
                }
            } else {
                // Finally ... TODO: Simplify this. Partially base case-related.
                let flag = self.predecessor.get(&y);
                // ... if it is not a Tree edge ...
                if x != y && flag.is_some() && flag != Some(&x) {
                    // ... then it is a Forward edge.
                    return Some(DFSEdge::Forward(x, y));
                }
            }
        }

        None
    }
}

impl<'a, G, D> From<&'a G> for DepthFirstSearchEdges<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    fn from(g: &'a G) -> Self {
        Self::new(g, None, Traversal::Tree)
    }
}

impl<'a, G, D> From<(&'a G, usize)> for DepthFirstSearchEdges<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x), Traversal::Tree)
    }
}

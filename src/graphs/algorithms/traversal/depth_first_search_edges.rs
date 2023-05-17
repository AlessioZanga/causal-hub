use std::{collections::VecDeque, iter::FusedIterator};

use super::Traversal;
use crate::{
    graphs::{directions, BaseGraph, DirectedGraph, PartiallyDirectedGraph, UndirectedGraph},
    Ch, Ne, V,
};

/// Edge classification performed by the [depth first search edges](`DepthFirstSearchEdges`) algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub discovery_time: Vec<usize>,
    /// Finish time of each discovered vertex.
    pub finish_time: Vec<usize>,
    /// Predecessor of each discovered vertex (except the source vertex).
    pub predecessor: Vec<usize>,
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
    #[inline]
    pub fn new(g: &'a G, x: Option<usize>, m: Traversal) -> Self {
        // Get graph order.
        let order = g.order();
        // Initialize the to-be-visited queue with the source vertex.
        let mut stack = Vec::default();
        // Initialize the global clock.
        let time = 0;
        // Initialize the discovery-time map.
        let discovery_time = vec![usize::MAX; order];
        // Initialize the finish-time map.
        let finish_time = vec![usize::MAX; order];
        // Initialize the predecessor map.
        let predecessor = vec![usize::MAX; order];

        // If visit variant is Forest.
        if matches!(m, Traversal::Forest) {
            // Add vertices to the visit stack...
            stack.extend(V!(g).map(|x| (usize::MAX, x)));
            // ... in reverse to preserve order.
            stack.reverse();
        }

        // If no source vertex is given, choose the first in the vertex set.
        if let Some(x) = x.or_else(|| V!(g).next()) {
            // ... assert that source vertex is in graph.
            assert!(g.has_vertex_by_index(x));
            // Push source vertex onto the stack.
            stack.push((usize::MAX, x));
        };

        Self {
            g,
            stack,
            time,
            discovery_time,
            finish_time,
            predecessor,
        }
    }
}

impl<'a, G, D> From<&'a G> for DepthFirstSearchEdges<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    #[inline]
    fn from(g: &'a G) -> Self {
        Self::new(g, None, Traversal::Tree)
    }
}

impl<'a, G, D> From<(&'a G, usize)> for DepthFirstSearchEdges<'a, G, D>
where
    G: BaseGraph<Direction = D>,
{
    #[inline]
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x), Traversal::Tree)
    }
}

impl<'a, G> Iterator for DepthFirstSearchEdges<'a, G, directions::Undirected>
where
    G: UndirectedGraph<Direction = directions::Undirected>,
{
    type Item = DFSEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&(x, y)) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if self.discovery_time[y] == usize::MAX {
                // Set its discover time (as GRAY).
                self.discovery_time[y] = self.time;
                // Increment time.
                self.time += 1;

                // Initialize visiting queue.
                let mut queue = VecDeque::new();

                // Iterate over reachable vertices.
                for z in Ne!(self.g, y) {
                    // Filter incoming edge.
                    if self.predecessor[y] != z {
                        // Add to queue.
                        queue.push_front((y, z));
                    }
                    // Filter already visited vertices (as GRAY).
                    if self.discovery_time[z] == usize::MAX {
                        // Set predecessor.
                        self.predecessor[z] = y;
                    }
                }

                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);

                // Filter the base case (root node).
                if x == usize::MAX {
                    continue;
                }

                // discovery[x] < discovery[y] < finish[y] < finish[x].
                return Some(DFSEdge::Tree(x, y));
            }

            // If the vertex is NOT WHITE, remove it from stack.
            self.stack.pop();
            // Get Y predecessor.
            let predecessor_y = self.predecessor[y];
            // Check if it is GRAY (not BLACK).
            if self.finish_time[y] == usize::MAX
                && (predecessor_y == usize::MAX || predecessor_y == x)
            {
                // Set its finish time (as BLACK).
                self.finish_time[y] = self.time;
                // Increment time.
                self.time += 1;
            }

            // Filter the base case.
            if x == usize::MAX {
                continue;
            }

            // Get X discovery time.
            let discovery_x = self.discovery_time[x];
            // Get Y discovery time.
            let discovery_y = self.discovery_time[y];
            // Get Y finish time.
            let finish_y = self.finish_time[y];

            // NOTE: Given that the graph is undirected, there are no forward nor cross edges.
            if discovery_x >= discovery_y && discovery_x < finish_y && finish_y != usize::MAX {
                // discovery[x] > discovery[y] && discovery[x] < finish[y].
                return Some(DFSEdge::Back(x, y));
            }
        }

        None
    }
}

impl<'a, G> FusedIterator for DepthFirstSearchEdges<'a, G, directions::Undirected> where
    G: UndirectedGraph<Direction = directions::Undirected>
{
}

impl<'a, G> Iterator for DepthFirstSearchEdges<'a, G, directions::Directed>
where
    G: DirectedGraph<Direction = directions::Directed>,
{
    type Item = DFSEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&(x, y)) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if self.discovery_time[y] == usize::MAX {
                // Set its discover time (as GRAY).
                self.discovery_time[y] = self.time;
                // Increment time.
                self.time += 1;

                // Initialize visiting queue.
                let mut queue = VecDeque::new();

                // Iterate over reachable vertices.
                for z in Ch!(self.g, y) {
                    // Add to queue.
                    queue.push_front((y, z));
                    // Filter already visited vertices (as GRAY).
                    if self.discovery_time[z] == usize::MAX {
                        // Set predecessor.
                        self.predecessor[z] = y;
                    }
                }

                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);

                // Filter the base case.
                if x == usize::MAX {
                    continue;
                }

                // discovery[x] < discovery[y] && finish[x] < finish[y].
                return Some(DFSEdge::Tree(x, y));
            }

            // If the vertex is NOT WHITE, remove it from stack.
            self.stack.pop();
            // Get Y predecessor.
            let predecessor_y = self.predecessor[y];
            // Check if it is GRAY (not BLACK).
            if self.finish_time[y] == usize::MAX
                && (predecessor_y == usize::MAX || predecessor_y == x)
            {
                // Set its finish time (as BLACK).
                self.finish_time[y] = self.time;
                // Increment time.
                self.time += 1;
            }

            // Filter the base case.
            if x == usize::MAX {
                continue;
            }

            // Get X discovery time.
            let discovery_x = self.discovery_time[x];
            // Get Y discovery time.
            let discovery_y = self.discovery_time[y];
            // Get Y finish time.
            let finish_y = self.finish_time[y];
            // Get Y predecessor.
            let predecessor_y = self.predecessor[y];

            // discovery[x] > discovery[y] ...
            if discovery_x >= discovery_y {
                // ... && discovery[x] < finish[y], or ...
                if discovery_x < finish_y {
                    return Some(DFSEdge::Back(x, y));
                }
                // ... && discovery[x] > finish[y], or ...
                return Some(DFSEdge::Cross(x, y));
            }
            // ... it is a forward edge.
            if predecessor_y != x && predecessor_y != usize::MAX {
                return Some(DFSEdge::Forward(x, y));
            }
        }

        None
    }
}

impl<'a, G> FusedIterator for DepthFirstSearchEdges<'a, G, directions::Directed> where
    G: DirectedGraph<Direction = directions::Directed>
{
}

impl<'a, G> Iterator for DepthFirstSearchEdges<'a, G, directions::PartiallyDirected>
where
    G: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>,
{
    type Item = DFSEdge;

    fn next(&mut self) -> Option<Self::Item> {
        // If there are still vertices to be visited.
        while let Some(&(x, y)) = self.stack.last() {
            // Check if vertex is WHITE (i.e. was not seen before).
            if self.discovery_time[y] == usize::MAX {
                // Set its discover time (as GRAY).
                self.discovery_time[y] = self.time;
                // Increment time.
                self.time += 1;

                // Initialize visiting queue.
                let mut queue = VecDeque::new();

                // Iterate over reachable vertices.
                for z in iter_set::union(Ne!(self.g, y), Ch!(self.g, y)) {
                    // Filter incoming edge.
                    if self.predecessor[y] != z {
                        // Add to queue.
                        queue.push_front((y, z));
                    }
                    // Filter already visited vertices (as GRAY).
                    if self.discovery_time[z] == usize::MAX {
                        // Set predecessor.
                        self.predecessor[z] = y;
                    }
                }

                // Push vertices onto the stack in reverse order, this makes
                // traversal order and neighborhood order the same.
                self.stack.extend(queue);

                // Filter the base case.
                if x == usize::MAX {
                    continue;
                }

                // discovery[x] < discovery[y] && finish[x] < finish[y].
                return Some(DFSEdge::Tree(x, y));
            }

            // If the vertex is NOT WHITE, remove it from stack.
            self.stack.pop();
            // Get Y predecessor.
            let predecessor_y = self.predecessor[y];
            // Check if it is GRAY (not BLACK).
            if self.finish_time[y] == usize::MAX
                && (predecessor_y == usize::MAX || predecessor_y == x)
            {
                // Set its finish time (as BLACK).
                self.finish_time[y] = self.time;
                // Increment time.
                self.time += 1;
            }

            // Filter the base case.
            if x == usize::MAX {
                continue;
            }

            // Get X discovery time.
            let discovery_x = self.discovery_time[x];
            // Get Y discovery time.
            let discovery_y = self.discovery_time[y];
            // Get Y finish time.
            let finish_y = self.finish_time[y];
            // Get Y predecessor.
            let predecessor_y = self.predecessor[y];

            // discovery[x] > discovery[y] ...
            if discovery_x >= discovery_y {
                // ... && discovery[x] < finish[y], or ...
                if discovery_x < finish_y {
                    return Some(DFSEdge::Back(x, y));
                }
                // ... && discovery[x] > finish[y], or ...
                return Some(DFSEdge::Cross(x, y));
            }
            // ... it is a forward edge.
            if predecessor_y != x && predecessor_y != usize::MAX {
                return Some(DFSEdge::Forward(x, y));
            }
        }

        None
    }
}

impl<'a, G> FusedIterator for DepthFirstSearchEdges<'a, G, directions::PartiallyDirected> where
    G: PartiallyDirectedGraph<Direction = directions::PartiallyDirected>
{
}

/// Alias for depth-first search.
pub type DFSEdges<'a, G, D> = DepthFirstSearchEdges<'a, G, D>;

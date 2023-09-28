use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use crate::{graphs::UndirectedGraph, Ne, V};

pub struct LexicographicDepthFirstSearch<'a, G>
where
    G: UndirectedGraph,
{
    g: &'a G,

    index: usize,

    queue: HashMap<usize, VecDeque<usize>>,

    pub predecessor: HashMap<usize, usize>,
}

impl<'a, G> LexicographicDepthFirstSearch<'a, G>
where
    G: UndirectedGraph,
{
    pub fn new(g: &'a G, x: Option<usize>) -> Self {
        // Initialize default search object.
        let mut search = Self {
            // Set target graph.
            g,
            // Initialize index.
            index: Default::default(),
            // Initialize the to-be-visited queue with labels.
            queue: HashMap::with_capacity(g.order()),
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
        // Add any vertex except the source vertex.
        search.queue.extend(V!(g).map(|x| (x, VecDeque::default())));
        // Push source vertex in front.
        search.queue.get_mut(&x).unwrap().push_front(search.index);
        // Return search object.
        search
    }
}

impl<'a, G> Iterator for LexicographicDepthFirstSearch<'a, G>
where
    G: UndirectedGraph,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // While the queue is non-empty.
        if !self.queue.is_empty() {
            // Select next vertex.
            let x = self
                .queue
                .iter()
                // Get min vertex with max label.
                .max_by(|(x, x_label), (y, y_label)| match x_label.cmp(y_label) {
                    // If labels are equal, then prefer min vertex.
                    Ordering::Equal => y.cmp(x),
                    // Otherwise, return ordering result.
                    ordering => ordering,
                })
                .map(|(&x, _)| x)
                .unwrap();
            // Remove selected vertex from the visit queue.
            self.queue.remove(&x);
            // Iterate over vertex neighbors.
            for y in Ne!(self.g, x) {
                // If neighbor has not been visited yet.
                if let Some(y_label) = self.queue.get_mut(&y) {
                    // Set its predecessor.
                    self.predecessor.insert(y, x);
                    // Update neighbor label.
                    y_label.push_front(self.index);
                }
            }
            // Increase current index.
            self.index += 1;
            // Return lexicographic order.
            return Some(x);
        }

        None
    }
}

impl<'a, G> From<&'a G> for LexicographicDepthFirstSearch<'a, G>
where
    G: UndirectedGraph,
{
    fn from(g: &'a G) -> Self {
        Self::new(g, None)
    }
}

impl<'a, G> From<(&'a G, usize)> for LexicographicDepthFirstSearch<'a, G>
where
    G: UndirectedGraph,
{
    fn from((g, x): (&'a G, usize)) -> Self {
        Self::new(g, Some(x))
    }
}

pub type LexDFS<'a, G> = LexicographicDepthFirstSearch<'a, G>;

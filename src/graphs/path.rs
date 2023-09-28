use super::{
    algorithms::traversal::{DFSEdge, DFSEdges, Traversal},
    Graph,
};
use crate::{
    graphs::{DirectedDenseAdjacencyMatrix, UndirectedDenseAdjacencyMatrix},
    prelude::BFS,
};

pub trait PathGraph {
    fn has_path(&self, x: usize, y: usize) -> bool;

    fn is_acyclic(&self) -> bool;
}

/* Implement PathGraph */
impl PathGraph for UndirectedDenseAdjacencyMatrix {
    #[inline]
    fn has_path(&self, x: usize, y: usize) -> bool {
        self.has_edge(x, y) || BFS::from((self, x)).skip(1).any(|z| z == y)
    }

    #[inline]
    fn is_acyclic(&self) -> bool {
        !DFSEdges::new(self, None, Traversal::Forest).any(|e| matches!(e, DFSEdge::Back(_, _)))
    }
}

/* Implement PathGraph */
impl PathGraph for DirectedDenseAdjacencyMatrix {
    #[inline]
    fn has_path(&self, x: usize, y: usize) -> bool {
        self.has_edge(x, y) || BFS::from((self, x)).skip(1).any(|z| z == y)
    }

    #[inline]
    fn is_acyclic(&self) -> bool {
        !DFSEdges::new(self, None, Traversal::Forest).any(|e| matches!(e, DFSEdge::Back(_, _)))
    }
}

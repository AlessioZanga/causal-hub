use crate::graphs::DirectedGraph;

pub trait MoralGraph: DirectedGraph {
    fn to_moral(&self) -> Self::UndirectedGraph;
}

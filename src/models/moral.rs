use crate::graphs::{directions, UndirectedGraph};

pub trait MoralGraph {
    type MoralGraph: UndirectedGraph<Direction = directions::Undirected>;

    fn moral(&self) -> Self::MoralGraph;
}

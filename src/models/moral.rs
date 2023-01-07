use crate::{
    graphs::{directions, IntoUndirectedGraph},
    prelude::{BaseGraph, UndirectedGraph},
};

pub trait MoralGraph: IntoUndirectedGraph {
    type MoralGraph: BaseGraph<Direction = directions::Undirected> + UndirectedGraph;

    fn moral(&self) -> Self::MoralGraph;
}

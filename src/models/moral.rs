use crate::{
    graphs::{directions, IntoUndirectedGraph},
    prelude::{BaseGraph, UndirectedGraph},
};

pub trait IntoMoralGraph: IntoUndirectedGraph {
    type MoralGraph: BaseGraph<Direction = directions::Undirected> + UndirectedGraph;

    fn into_moral(self) -> Self::MoralGraph;
}

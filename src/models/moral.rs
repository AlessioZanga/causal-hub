use crate::{
    graphs::directions,
    prelude::{BaseGraph, UndirectedGraph},
};

pub trait IntoMoralGraph {
    type MoralGraph: BaseGraph<Direction = directions::Undirected> + UndirectedGraph;

    fn into_moral(self) -> Self::MoralGraph;
}

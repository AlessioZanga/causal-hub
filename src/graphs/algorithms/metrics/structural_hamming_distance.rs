use crate::{graphs::BaseGraph, types::FxIndexSet, E};

/// Structural Hamming Distance (SHD).
pub fn shd<G, H>(true_graph: &G, pred_graph: &H) -> f64
where
    G: BaseGraph,
    H: BaseGraph,
{
    // Accumulate edges set.
    let true_graph: FxIndexSet<_> = E!(true_graph).collect();
    let pred_graph: FxIndexSet<_> = E!(pred_graph).collect();

    // Compute missing and extra edges as symmetric difference.
    let shd = &true_graph ^ &pred_graph;
    // Symmetrize edges to count reversed just once.
    let shd: FxIndexSet<_> = shd
        .into_iter()
        .map(|(i, j)| match i <= j {
            true => (i, j),
            false => (j, i),
        })
        .collect();

    shd.len() as f64
}

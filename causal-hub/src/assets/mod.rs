use dry::macro_for;
use ndarray::prelude::*;
use paste::paste;

use crate::{
    distributions::CategoricalCIM,
    graphs::{DiGraph, Graph},
    io::BifReader,
    models::{CTBN, CategoricalBN, CategoricalCTBN},
};

macro_for!(
    $bn in [
        alarm, andes, asia, barley, cancer, child, diabetes, earthquake,
        hailfinder, hepar2, insurance, link, mildew, munin1, pathfinder,
        pigs, sachs, survey, water, win95pts
    ] {
    paste! {
        #[doc = "Load the `" $bn:upper "` BN from the assets."]
        pub fn [<load_ $bn>]() -> CategoricalBN {
            BifReader::read(include_str!(concat!(stringify!($bn), ".bif")))
        }
    }
});

/// Load the EATING CTBN.
///
/// See:
///     U. Nodelman, C.R. Shelton, and D. Koller (2003). "Learning Continuous Time Bayesian Networks."
///     Proc. Nineteenth Conference on Uncertainty in Artificial Intelligence (UAI) (pp. 451-458).
///
pub fn load_eating() -> CategoricalCTBN {
    // Initialize the graph.
    let mut graph = DiGraph::empty(vec!["Hungry", "Eating", "FullStomach"]);
    graph.add_edge(0, 1); // Hungry -> Eating
    graph.add_edge(1, 2); // Eating -> FullStomach
    graph.add_edge(2, 0); // FullStomach -> Hungry

    // Initialize the distributions.
    let cims = vec![
        CategoricalCIM::new(
            // P(Hungry | FullStomach)
            ("Hungry", vec!["no", "yes"]),
            [("FullStomach", vec!["no", "yes"])],
            array![
                [
                    [-0.1, 0.1], //
                    [10., -10.]  //
                ],
                [
                    [-2., 2.],   //
                    [0.1, -0.1]  //
                ],
            ],
        ),
        CategoricalCIM::new(
            // P(Eating | Hungry)
            ("Eating", vec!["no", "yes"]),
            [("Hungry", vec!["no", "yes"])],
            array![
                [
                    [-0.1, 0.1], //
                    [10., -10.]  //
                ],
                [
                    [-2., 2.],   //
                    [0.1, -0.1]  //
                ],
            ],
        ),
        CategoricalCIM::new(
            // P(FullStomach | Eating)
            ("FullStomach", vec!["no", "yes"]),
            [("Eating", vec!["no", "yes"])],
            array![
                [
                    [-0.1, 0.1], //
                    [10., -10.]  //
                ],
                [
                    [-2., 2.],   //
                    [0.1, -0.1]  //
                ],
            ],
        ),
    ];

    // Initialize the model.
    CategoricalCTBN::new(graph, cims)
}

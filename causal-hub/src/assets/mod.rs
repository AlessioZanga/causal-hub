mod bif;
pub use bif::*;

mod json_schema;
pub(crate) use json_schema::*;

mod ctbn {
    use log::debug;
    use ndarray::prelude::*;

    use crate::{
        distributions::CatCIM,
        graphs::{DiGraph, Graph},
        map,
        models::{CTBN, CatCTBN},
        set,
    };

    /// Load the `EATING` CTBN from assets.
    ///
    /// See:
    ///     U. Nodelman, C.R. Shelton, and D. Koller (2003). "Learning Continuous Time Bayesian Networks."
    ///     Proc. Nineteenth Conference on Uncertainty in Artificial Intelligence (UAI) (pp. 451-458).
    ///
    pub fn load_eating() -> CatCTBN {
        // Log the loading of the EATING CTBN.
        debug!("Loading the 'EATING' CTBN from assets.");
        // Initialize the graph.
        let mut graph = DiGraph::empty(vec!["Hungry", "Eating", "FullStomach"]);
        graph.add_edge(0, 1); // Hungry -> Eating
        graph.add_edge(1, 2); // Eating -> FullStomach
        graph.add_edge(2, 0); // FullStomach -> Hungry

        // Set the states of the variables.
        let states = set!["no".to_string(), "yes".to_string()];

        // Initialize the distributions.
        let cims = vec![
            CatCIM::new(
                // P(Hungry | FullStomach)
                map![("Hungry".to_string(), states.clone())],
                map![("FullStomach".to_string(), states.clone())],
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
            CatCIM::new(
                // P(Eating | Hungry)
                map![("Eating".to_string(), states.clone())],
                map![("Hungry".to_string(), states.clone())],
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
            CatCIM::new(
                // P(FullStomach | Eating)
                map![("FullStomach".to_string(), states.clone())],
                map![("Eating".to_string(), states.clone())],
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
        CatCTBN::new(graph, cims)
    }
}

pub use ctbn::*;

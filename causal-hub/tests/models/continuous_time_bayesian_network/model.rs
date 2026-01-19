#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_eating,
        labels,
        models::{BN, CIM, CPD, CTBN, Graph, Labelled},
        states,
    };
    use ndarray::prelude::*;

    #[test]
    fn new() {
        // Initialize the model.
        let model = load_eating();

        // Check the labels.
        assert_eq!(model.labels(), &labels!["Eating", "FullStomach", "Hungry"]);
        // Check the graph structure.
        assert_eq!(model.graph().vertices().len(), 3);
        assert!(model.graph().has_edge(0, 1));
        assert!(model.graph().has_edge(1, 2));
        assert!(model.graph().has_edge(2, 0));
        // Check the distributions.
        assert_eq!(model.cims().len(), 3);
        assert_eq!(model.cims()[0].labels(), &labels!["Eating"]);
        assert_eq!(model.cims()[1].labels(), &labels!["FullStomach"]);
        assert_eq!(model.cims()[2].labels(), &labels!["Hungry"]);
        assert_eq!(model.cims()[0].conditioning_labels(), &labels!["Hungry"]);
        assert_eq!(model.cims()[1].conditioning_labels(), &labels!["Eating"]);
        assert_eq!(
            model.cims()[2].conditioning_labels(),
            &labels!["FullStomach"]
        );

        // Check the parameters.
        assert_eq!(
            model.cims()[0].parameters(),
            &array![
                [
                    [-0.1, 0.1], //
                    [10., -10.]  //
                ],
                [
                    [-2., 2.],   //
                    [0.1, -0.1]  //
                ],
            ]
        );
        assert_eq!(
            model.cims()[1].parameters(),
            &array![
                [
                    [-0.1, 0.1], //
                    [10., -10.]  //
                ],
                [
                    [-2., 2.],   //
                    [0.1, -0.1]  //
                ],
            ]
        );
        assert_eq!(
            model.cims()[2].parameters(),
            &array![
                [
                    [-0.1, 0.1], //
                    [10., -10.]  //
                ],
                [
                    [-2., 2.],   //
                    [0.1, -0.1]  //
                ],
            ]
        );
        // Check the states.
        assert_eq!(
            model.cims()[0].states(),
            &states![("Eating", ["no", "yes"])]
        );
        assert_eq!(
            model.cims()[1].states(),
            &states![("FullStomach", ["no", "yes"])]
        );
        assert_eq!(
            model.cims()[2].states(),
            &states![("Hungry", ["no", "yes"])]
        );
        // Check the parameters size.
        assert_eq!(model.parameters_size(), 15);

        // Check the initial distribution.
        let initial_distribution = model.initial_distribution();

        // Check the labels.
        assert_eq!(initial_distribution.labels(), model.labels());
        // Check the graph structure.
        assert_eq!(initial_distribution.graph().vertices().len(), 3);
        // Check the distributions.
        assert_eq!(initial_distribution.cpds().len(), 3);
        assert_eq!(initial_distribution.cpds()[0].labels(), &labels!["Eating"]);
        assert_eq!(
            initial_distribution.cpds()[1].labels(),
            &labels!["FullStomach"]
        );
        assert_eq!(initial_distribution.cpds()[2].labels(), &labels!["Hungry"]);
        assert_eq!(
            initial_distribution.cpds()[0].conditioning_labels(),
            &labels![]
        );
        assert_eq!(
            initial_distribution.cpds()[1].conditioning_labels(),
            &labels![]
        );
        assert_eq!(
            initial_distribution.cpds()[2].conditioning_labels(),
            &labels![]
        );
        // Check the parameters.
        assert_eq!(
            initial_distribution.cpds()[0].parameters(),
            &array![[0.5, 0.5]] //
        );
        assert_eq!(
            initial_distribution.cpds()[1].parameters(),
            &array![[0.5, 0.5]] //
        );
        assert_eq!(
            initial_distribution.cpds()[2].parameters(),
            &array![[0.5, 0.5]] //
        );
        // Check the states.
        assert_eq!(
            &states![("Eating", ["no", "yes"])],
            initial_distribution.cpds()[0].states()
        );
        assert_eq!(
            &states![("FullStomach", ["no", "yes"])],
            initial_distribution.cpds()[1].states()
        );
        assert_eq!(
            &states![("Hungry", ["no", "yes"])],
            initial_distribution.cpds()[2].states()
        );
        // Check the parameters size.
        assert_eq!(initial_distribution.parameters_size(), 3);
        // Check the topological order.
        assert_eq!(initial_distribution.topological_order(), &[0, 1, 2]);
    }
}

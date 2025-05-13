#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_eating,
        distributions::CPD,
        graphs::Graph,
        models::{BN, CTBN},
    };
    use ndarray::prelude::*;

    #[test]
    fn test_new() {
        // Initialize the model.
        let ctbn = load_eating();

        // Check the labels.
        assert!(ctbn.labels().iter().eq(["Eating", "FullStomach", "Hungry"]));
        // Check the graph structure.
        assert_eq!(ctbn.graph().vertices().len(), 3);
        assert!(ctbn.graph().has_edge(0, 1));
        assert!(ctbn.graph().has_edge(1, 2));
        assert!(ctbn.graph().has_edge(2, 0));
        // Check the distributions.
        assert_eq!(ctbn.cims().len(), 3);
        assert_eq!(ctbn.cims()[0].label(), "Eating");
        assert_eq!(ctbn.cims()[1].label(), "FullStomach");
        assert_eq!(ctbn.cims()[2].label(), "Hungry");
        assert!(ctbn.cims()[0].conditioning_labels().iter().eq(["Hungry"]));
        assert!(ctbn.cims()[1].conditioning_labels().iter().eq(["Eating"]));
        assert!(
            ctbn.cims()[2]
                .conditioning_labels()
                .iter()
                .eq(["FullStomach"])
        );

        // Check the parameters.
        assert_eq!(
            ctbn.cims()[0].parameters(),
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
            ctbn.cims()[1].parameters(),
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
            ctbn.cims()[2].parameters(),
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
        assert!(ctbn.cims()[0].states().iter().eq(["no", "yes"]));
        assert!(ctbn.cims()[1].states().iter().eq(["no", "yes"]));
        assert!(ctbn.cims()[2].states().iter().eq(["no", "yes"]));
        // Check the parameters size.
        assert_eq!(ctbn.parameters_size(), 15);

        // Check the initial distribution.
        let initial_distribution = ctbn.initial_distribution();

        // Check the labels.
        assert_eq!(initial_distribution.labels(), ctbn.labels());
        // Check the graph structure.
        assert_eq!(initial_distribution.graph().vertices().len(), 3);
        // Check the distributions.
        assert_eq!(initial_distribution.cpds().len(), 3);
        assert_eq!(initial_distribution.cpds()[0].label(), "Eating");
        assert_eq!(initial_distribution.cpds()[1].label(), "FullStomach");
        assert_eq!(initial_distribution.cpds()[2].label(), "Hungry");
        assert!(
            initial_distribution.cpds()[0]
                .conditioning_labels()
                .iter()
                .eq(Vec::<&str>::new())
        );
        assert!(
            initial_distribution.cpds()[1]
                .conditioning_labels()
                .iter()
                .eq(Vec::<&str>::new())
        );
        assert!(
            initial_distribution.cpds()[2]
                .conditioning_labels()
                .iter()
                .eq(Vec::<&str>::new())
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
        assert!(
            initial_distribution.cpds()[0]
                .states()
                .iter()
                .eq(["no", "yes"])
        );
        assert!(
            initial_distribution.cpds()[1]
                .states()
                .iter()
                .eq(["no", "yes"])
        );
        assert!(
            initial_distribution.cpds()[2]
                .states()
                .iter()
                .eq(["no", "yes"])
        );
        // Check the parameters size.
        assert_eq!(initial_distribution.parameters_size(), 3);
        // Check the topological order.
        assert_eq!(initial_distribution.topological_order(), &[0, 1, 2]);
    }
}

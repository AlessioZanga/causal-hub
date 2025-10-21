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
        let ctbn = load_eating();

        // Check the labels.
        assert_eq!(&labels!["Eating", "FullStomach", "Hungry"], ctbn.labels());
        // Check the graph structure.
        assert_eq!(ctbn.graph().vertices().len(), 3);
        assert!(ctbn.graph().has_edge(0, 1));
        assert!(ctbn.graph().has_edge(1, 2));
        assert!(ctbn.graph().has_edge(2, 0));
        // Check the distributions.
        assert_eq!(ctbn.cims().len(), 3);
        assert_eq!(&labels!["Eating"], ctbn.cims()[0].labels());
        assert_eq!(&labels!["FullStomach"], ctbn.cims()[1].labels());
        assert_eq!(&labels!["Hungry"], ctbn.cims()[2].labels());
        assert_eq!(&labels!["Hungry"], ctbn.cims()[0].conditioning_labels());
        assert_eq!(&labels!["Eating"], ctbn.cims()[1].conditioning_labels());
        assert_eq!(
            &labels!["FullStomach"],
            ctbn.cims()[2].conditioning_labels()
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
        assert_eq!(&states![("Eating", ["no", "yes"])], ctbn.cims()[0].states());
        assert_eq!(
            &states![("FullStomach", ["no", "yes"])],
            ctbn.cims()[1].states()
        );
        assert_eq!(&states![("Hungry", ["no", "yes"])], ctbn.cims()[2].states());
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
        assert_eq!(&labels!["Eating"], initial_distribution.cpds()[0].labels());
        assert_eq!(
            &labels!["FullStomach"],
            initial_distribution.cpds()[1].labels()
        );
        assert_eq!(&labels!["Hungry"], initial_distribution.cpds()[2].labels());
        assert_eq!(
            &labels![],
            initial_distribution.cpds()[0].conditioning_labels()
        );
        assert_eq!(
            &labels![],
            initial_distribution.cpds()[1].conditioning_labels()
        );
        assert_eq!(
            &labels![],
            initial_distribution.cpds()[2].conditioning_labels()
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

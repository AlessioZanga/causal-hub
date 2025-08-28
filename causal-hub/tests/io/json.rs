#[cfg(test)]
mod tests {
    mod categorical {
        mod continuous_time_bayesian_network {
            use causal_hub::{assets::load_eating, io::JsonIO, models::CatCTBN};

            #[test]
            fn test_from_json() {
                // Load model.
                let true_model = load_eating();
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Deserialize model from JSON.
                let pred_model = CatCTBN::from_json(json.as_str());
                // Assert the models are equal.
                assert_eq!(true_model, pred_model);
            }

            #[test]
            fn test_to_json() {
                // Load model.
                let true_model = load_eating();
                // Serialize model to JSON.
                let json = true_model.to_json();
                // Assert the JSON string is correct.
                assert_eq!(
                    json,
                    r#"{"initial_distribution":{"states":{"Eating":["no","yes"],"FullStomach":["no","yes"],"Hungry":["no","yes"]},"graph":{"labels":["Eating","FullStomach","Hungry"],"adjacency_matrix":{"v":1,"dim":[3,3],"data":[false,false,false,false,false,false,false,false,false]}},"cpds":{"Eating":{"labels":["Eating"],"states":{"Eating":["no","yes"]},"cardinality":{"v":1,"dim":[1],"data":[2]},"multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"conditioning_labels":[],"conditioning_states":{},"conditioning_cardinality":{"v":1,"dim":[0],"data":[]},"conditioning_multi_index":{"cardinality":{"v":1,"dim":[0],"data":[]},"strides":{"v":1,"dim":[0],"data":[]}},"parameters":{"v":1,"dim":[1,2],"data":[0.5,0.5]},"parameters_size":1,"sample_size":null,"sample_log_likelihood":null},"FullStomach":{"labels":["FullStomach"],"states":{"FullStomach":["no","yes"]},"cardinality":{"v":1,"dim":[1],"data":[2]},"multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"conditioning_labels":[],"conditioning_states":{},"conditioning_cardinality":{"v":1,"dim":[0],"data":[]},"conditioning_multi_index":{"cardinality":{"v":1,"dim":[0],"data":[]},"strides":{"v":1,"dim":[0],"data":[]}},"parameters":{"v":1,"dim":[1,2],"data":[0.5,0.5]},"parameters_size":1,"sample_size":null,"sample_log_likelihood":null},"Hungry":{"labels":["Hungry"],"states":{"Hungry":["no","yes"]},"cardinality":{"v":1,"dim":[1],"data":[2]},"multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"conditioning_labels":[],"conditioning_states":{},"conditioning_cardinality":{"v":1,"dim":[0],"data":[]},"conditioning_multi_index":{"cardinality":{"v":1,"dim":[0],"data":[]},"strides":{"v":1,"dim":[0],"data":[]}},"parameters":{"v":1,"dim":[1,2],"data":[0.5,0.5]},"parameters_size":1,"sample_size":null,"sample_log_likelihood":null}},"topological_order":[0,1,2]},"graph":{"labels":["Eating","FullStomach","Hungry"],"adjacency_matrix":{"v":1,"dim":[3,3],"data":[false,true,false,false,false,true,true,false,false]}},"cims":{"Eating":{"labels":["Eating"],"states":{"Eating":["no","yes"]},"cardinality":{"v":1,"dim":[1],"data":[2]},"multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"conditioning_labels":["Hungry"],"conditioning_states":{"Hungry":["no","yes"]},"conditioning_cardinality":{"v":1,"dim":[1],"data":[2]},"conditioning_multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"parameters":{"v":1,"dim":[2,2,2],"data":[-0.1,0.1,10.0,-10.0,-2.0,2.0,0.1,-0.1]},"parameters_size":4,"sample_size":null,"sample_log_likelihood":null},"FullStomach":{"labels":["FullStomach"],"states":{"FullStomach":["no","yes"]},"cardinality":{"v":1,"dim":[1],"data":[2]},"multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"conditioning_labels":["Eating"],"conditioning_states":{"Eating":["no","yes"]},"conditioning_cardinality":{"v":1,"dim":[1],"data":[2]},"conditioning_multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"parameters":{"v":1,"dim":[2,2,2],"data":[-0.1,0.1,10.0,-10.0,-2.0,2.0,0.1,-0.1]},"parameters_size":4,"sample_size":null,"sample_log_likelihood":null},"Hungry":{"labels":["Hungry"],"states":{"Hungry":["no","yes"]},"cardinality":{"v":1,"dim":[1],"data":[2]},"multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"conditioning_labels":["FullStomach"],"conditioning_states":{"FullStomach":["no","yes"]},"conditioning_cardinality":{"v":1,"dim":[1],"data":[2]},"conditioning_multi_index":{"cardinality":{"v":1,"dim":[1],"data":[2]},"strides":{"v":1,"dim":[1],"data":[1]}},"parameters":{"v":1,"dim":[2,2,2],"data":[-0.1,0.1,10.0,-10.0,-2.0,2.0,0.1,-0.1]},"parameters_size":4,"sample_size":null,"sample_log_likelihood":null}}}"#
                );
            }
        }
    }
}

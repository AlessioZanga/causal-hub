use std::{io::Error as IOError, path::PathBuf};

use itertools::Itertools;
use ndarray::prelude::*;
use pest::{error::Error as ParserError, iterators::Pairs, Parser};
use pest_derive::Parser;

use crate::{
    io::File,
    models::DiscreteCPD,
    prelude::{DiscreteBayesianNetwork, Factor, FxIndexMap, FxIndexSet},
};

#[derive(Clone, Debug, Default, Parser)]
#[grammar = "io/bif/grammar.pest"]
pub struct BIF {
    /// Parameters. TODO: Generalize to the continuous case.
    pub theta: Vec<DiscreteCPD>,
}

impl<'a> From<Pairs<'a, Rule>> for BIF {
    fn from(pairs: Pairs<'a, Rule>) -> Self {
        // Initialize scope map. TODO: Generalize to the continuous case.
        let mut scope: FxIndexMap<String, FxIndexSet<String>> = Default::default();
        // Initialize CPDs tables vector. TODO: Generalize to the continuous case.
        let mut tables: Vec<(Vec<String>, Array1<f64>)> = Default::default();

        // Match inner rules.
        let mut inner = pairs;

        // Assert rule match. TODO: Parse network properties.
        let _network = inner.next().unwrap();
        assert!(matches!(_network.as_rule(), Rule::network_declaration));

        // Assert rule match.
        for variable_probability in inner {
            match variable_probability.as_rule() {
                Rule::variable_declaration => {
                    // Match inner rules.
                    let mut i = variable_probability.into_inner();

                    // Assert rule match.
                    let name = i.next().unwrap();
                    assert!(matches!(name.as_rule(), Rule::variable_name));
                    // Get variable name.
                    let name = name.as_str().into();

                    // Assert rule match.
                    let content = i.next().unwrap();
                    assert!(matches!(content.as_rule(), Rule::variable_content));
                    // Match inner rules. TODO: Generalize to the continuous case.
                    let mut i = content.into_inner();

                    // Assert rule match.
                    let discrete = i.next().unwrap();
                    assert!(matches!(discrete.as_rule(), Rule::variable_discrete));
                    // Match inner rules.
                    let mut i = discrete.into_inner();

                    // Assert rule match.
                    let states = i.next().unwrap();
                    assert!(matches!(states.as_rule(), Rule::variable_states_list));
                    // Collect states.
                    let states = states.into_inner().map(|s| s.as_str().into()).collect();

                    // Insert variable with states into scope.
                    scope.insert(name, states);
                }
                Rule::probability_declaration => {
                    // Match inner rules.
                    let mut i = variable_probability.into_inner();

                    // Assert rule match.
                    let variables = i.next().unwrap();
                    assert!(matches!(
                        variables.as_rule(),
                        Rule::probability_variables_list
                    ));
                    // Get variables names.
                    let variables = variables
                        .into_inner()
                        .map(|x| x.as_str().to_owned())
                        .collect();

                    // Assert rule match.
                    let content = i.next().unwrap();
                    assert!(matches!(content.as_rule(), Rule::probability_content));

                    // Initialize parameters values. TODO: Generalize to the continuous case.
                    let mut values: Vec<f64> = Default::default();
                    // Match into inner rules.
                    for entry in content.into_inner() {
                        match entry.as_rule() {
                            Rule::probability_default_entry | Rule::probability_table => values
                                .extend(
                                    entry
                                        .into_inner()
                                        .map(|x| x.as_str())
                                        .map(|x| x.parse::<f64>().unwrap()),
                                ),
                            Rule::probability_entry => values.extend(
                                entry
                                    .into_inner()
                                    .skip(1) // Skip states declaration.
                                    .map(|x| x.as_str())
                                    .map(|x| x.parse::<f64>().unwrap()),
                            ),
                            _ => unreachable!(),
                        }
                    }
                    // Convert vector to array.
                    let values = Array1::from_vec(values);

                    // Append to parsed results.
                    tables.push((variables, values));
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        // Construct parameters from scopes, variables and tables. TODO: Generalize to the continuous case.
        let theta = tables
            .into_iter()
            .map(|(variables, values)| {
                // Consume variables iterator.
                let mut variables = variables.into_iter();
                // Get target variable X scope.
                let x = variables.next().expect("Failed to get target variable");
                let (x, y) = (x.clone(), scope[&x].clone());
                // Get conditioning variables Z scopes.
                let z = variables.map(|z| (z.clone(), scope[&z].clone()));
                // Compute values shape as (\Prod_i |Z_i|, |X|).
                let shape = (values.len() / y.len(), y.len());
                // Reshape values.
                let values = values.into_shape(shape).expect("Failed to reshape values");
                // Normalized values.
                let values = &values / values.sum_axis(Axis(1)).insert_axis(Axis(1));
                // Construct associated parameter.
                DiscreteCPD::new((x, y), z, values)
            })
            .collect();

        Self { theta }
    }
}

impl From<BIF> for String {
    fn from(value: BIF) -> Self {
        // Allocate output string.
        let mut bif = String::new();

        // Write network declaration.
        bif += "network unknown {\n}\n";

        // Write variables declaration.
        for phi in value.theta.iter() {
            // Get associated target.
            let x = phi.target();
            // Get associated states.
            let s = &phi.states()[x];
            // Get cardinality of associated states.
            let c = s.len();
            // Collect associated states.
            let s = s.iter().join(", ");
            // Format variable declaration.
            bif += &format!("variable {x} {{\n  type discrete [ {c} ] {{ {s} }};\n}}\n");
        }

        // Write variables probability.
        for phi in value.theta {
            // Get associated target.
            let x = phi.target();
            // Match probability declaration with states.
            match phi.states().len() > 1 {
                // Format P(X | Z).
                true => {
                    // Get associated states.
                    let s = phi.states();
                    // Get conditioning variables.
                    let z = s.keys().filter(|&z| z != x).join(", ");
                    // Get target index.
                    let i = s
                        .get_index_of(x)
                        .expect("Failed to get index of target variable");
                    // Construct iterator over states levels.
                    let s = s
                        .iter()
                        .filter_map(|(s, t)| match !s.eq(&x) {
                            true => Some(t),
                            false => None,
                        })
                        .multi_cartesian_product();
                    // Construct iterator over values.
                    let mut v = phi
                        .values()
                        .axis_iter(Axis(i))
                        .map(|x| x.into_iter())
                        .collect_vec();
                    // Format probability values with conditioning states.
                    let v = s
                        .map(|s| {
                            // Format conditioning states.
                            let s = s.into_iter().join(", ");
                            // Format conditioned values.
                            let v = v
                                .iter_mut()
                                .map(|x| x.next().unwrap().to_string())
                                .join(", ");
                            // Joint states and values.
                            format!("  ({s}) {v};")
                        })
                        .join("\n");
                    // Format probability declaration.
                    bif += &format!("probability ( {x} | {z} ) {{\n{v}\n}}\n");
                }
                // Format P(X).
                false => {
                    // Format probability values.
                    let v = phi.values().into_iter().map(|x| x.to_string()).join(", ");
                    // Format probability declaration.
                    bif += &format!("probability ( {x} ) {{\n  table {v};\n}}\n")
                }
            }
        }

        bif
    }
}

impl TryFrom<String> for BIF {
    type Error = ParserError<Rule>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Parse the given string.
        let out = Self::parse(Rule::compilation_unit, string.trim())?;
        // Match inner rules.
        let out: Self = out.into();

        Ok(out)
    }
}

impl File for BIF {
    type ReadError = ParserError<Rule>;

    type WriteError = IOError;

    fn read<P>(path: P) -> Result<Self, Self::ReadError>
    where
        P: Into<PathBuf>,
    {
        // Get path.
        let path = path.into();
        // Read file to string.
        let out = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| format!("Failed to read file: \"{}\"", path.display()));
        // Parse string.
        Self::try_from(out)
    }

    fn write<P>(self, path: P) -> Result<(), Self::WriteError>
    where
        P: Into<PathBuf>,
    {
        // Format to string.
        let out = String::from(self);
        // Write string to file.
        std::fs::write(path.into(), out)
    }
}

impl From<DiscreteBayesianNetwork> for BIF {
    fn from(b: DiscreteBayesianNetwork) -> Self {
        // Get parameters.
        let (_, theta) = b.into();
        // Map to vector of parameters.
        let theta = theta.into_values().collect();

        Self { theta }
    }
}

use std::{io::Error as IOError, path::PathBuf};

use itertools::Itertools;
use ndarray::Array2;
use pest::{
    error::Error as ParserError,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{
    io::File,
    models::DiscreteCPD,
    prelude::{FxIndexMap, FxIndexSet},
};

#[derive(Clone, Debug, Default, Parser)]
#[grammar = "io/bif/grammar.pest"]
pub struct BayesianInterchangeFormat {
    /// CPDs set. TODO: Generalize to the continuous case.
    pub cpds: Vec<DiscreteCPD>,
}

impl<'a> From<Pairs<'a, Rule>> for BayesianInterchangeFormat {
    fn from(pairs: Pairs<'a, Rule>) -> Self {
        // Initialize CPDs vector.
        let mut tables: Vec<DiscreteCPD> = Default::default();
        // Initialize scopes map.
        let mut scopes: FxIndexMap<String, FxIndexSet<String>> = Default::default();

        // Match inner rules.
        let mut inner = pairs;

        // Assert rule match. TODO: Parse network properties.
        let network = inner.next().unwrap();
        assert!(matches!(network.as_rule(), Rule::network_declaration));

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
                    scopes.insert(name, states);
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
                    // Get variable name.
                    let variables: Vec<_> = variables
                        .into_inner()
                        .map(|x| x.as_str().to_owned())
                        .collect();

                    // Assert rule match.
                    let content = i.next().unwrap();
                    assert!(matches!(content.as_rule(), Rule::probability_content));

                    // Initialize CPDs values. TODO: Generalize to the continuous case.
                    let mut values: Vec<Vec<f64>> = Default::default();
                    // Match into inner rules.
                    for entry in content.into_inner() {
                        match entry.as_rule() {
                            Rule::probability_default_entry | Rule::probability_table => values
                                .push(
                                    entry
                                        .into_inner()
                                        .map(|x| x.as_str().parse().unwrap())
                                        .collect(),
                                ),
                            Rule::probability_entry => {}
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::EOI => {}
                _ => unreachable!(),
            }
        }

        Default::default() // FIXME:
    }
}

impl From<BayesianInterchangeFormat> for String {
    fn from(value: BayesianInterchangeFormat) -> Self {
        todo!() // FIXME:
    }
}

impl TryFrom<String> for BayesianInterchangeFormat {
    type Error = ParserError<Rule>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Parse the given string.
        let out = Self::parse(Rule::compilation_unit, string.trim())?;
        // Match inner rules.
        let out: Self = out.into();

        Ok(out)
    }
}

impl File for BayesianInterchangeFormat {
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

use itertools::Itertools;
use ndarray::prelude::*;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{
    models::{BN, CPD, CatBN, CatCPD, DiGraph, Graph, Labelled},
    types::{Error, Map, Result, States},
};

#[derive(Debug)]
struct Network {
    pub name: String,
    pub properties: Vec<Property>,
    pub variables: Vec<Variable>,
    pub probabilities: Vec<Probability>,
}

#[derive(Debug)]
struct Property {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
struct Variable {
    pub label: String,
    pub states: Vec<String>,
    pub _properties: Vec<Property>,
}

#[derive(Debug)]
struct Probability {
    pub label: String,
    pub parents: Vec<String>,
    pub table: Option<Vec<f64>>,                       // For flat CPTs
    pub entries: Option<Vec<(Vec<String>, Vec<f64>)>>, // For conditional CPTs
}

/// BIF parser for parsing Bayesian Interchange Format (BIF) files.
#[derive(Parser)]
#[grammar = "src/io/bif/bif.pest"]
pub struct BifParser;

impl BifParser {
    /// Read a BIF string and returns a `Network` object.
    pub fn parse_str(bif: &str) -> Result<CatBN> {
        let mut pairs = Self::parse(Rule::file, bif)
            .map_err(|e| Error::Parsing(format!("Failed to parse BIF file: {}", e)))?;

        let network_pair = pairs
            .next()
            .ok_or_else(|| Error::Parsing("Empty BIF file".into()))?;
        let network = build_ast(network_pair)?;

        // Get network properties.
        let properties: Map<_, _> = network
            .properties
            .into_iter()
            .map(|p| (p.key, p.value))
            .collect();

        // Get network name and description.
        let name = Some(network.name);
        let description = properties.get("description").cloned();

        // Construct states.
        let states: States = network
            .variables
            .into_iter()
            .map(|v| (v.label, v.states.into_iter().collect()))
            .collect();

        // Construct CPDs.
        let cpds: Result<Vec<_>> = network
            .probabilities
            .into_iter()
            .map(|p| {
                // Get the variable of the CPD.
                let variable = States::from_iter([(
                    p.label.clone(),
                    states
                        .get(&p.label)
                        .ok_or_else(|| Error::Parsing(format!("Failed to get states for variable '{}'.", p.label)))?
                        .clone(),
                )]);
                // Get the conditioning variables of the CPD.
                let conditioning_variables: States = p
                    .parents
                    .iter()
                    .map(|x| {
                        let states = states.get(x).ok_or_else(|| Error::Parsing(format!("Failed to get states for variable '{}'.", x)))?;
                        Ok((x.to_string(), states.iter().cloned().collect()))
                    })
                    .collect::<Result<_>>()?;

                // Map the probability values.
                let parameters = match (p.table, p.entries) {
                    (Some(table), None) => Array1::from_vec(table).insert_axis(Axis(0)),
                    (None, Some(entries)) => {
                        // Align the probability values with the states.
                        let entries: Map<_, _> = entries.into_iter().collect();
                        // Align the entries with the states.
                        let entries: Vec<_> = conditioning_variables
                            .iter()
                            .map(|(_, states)| states)
                            .cloned()
                            .multi_cartesian_product()
                            .map(|states| {
                                entries.get(&states).ok_or_else(|| Error::Parsing(format!(
                                    "Missing probability entry for configuration {:?}",
                                    states
                                )))
                            })
                            .collect::<Result<Vec<_>>>()?;
                        // Get the shape of the parameters.
                        let shape = (entries.len(), entries[0].len());
                        // Collect the parameters.
                        let parameters: Array1<_> =
                            entries.into_iter().flatten().copied().collect();
                        // Reshape the parameters.
                        parameters
                            .into_shape_with_order(shape)
                            .map_err(Error::NdarrayShape)?
                    }
                    _ => return Err(Error::Parsing("Invalid probability definition: must have either table or entries, not both or none.".into())),
                };

                // Normalize the parameters so that they sum exactly to 1 by row.
                let parameters = &parameters / parameters.sum_axis(Axis(1)).insert_axis(Axis(1));

                // Create the CPD.
                CatCPD::new(variable, conditioning_variables, parameters)
            })
            .collect();
        let cpds = cpds?;

        // Construct the graph.
        let mut graph = DiGraph::empty(states.keys());
        for p in &cpds {
            // Assert the CPD has a single variable in the BIF file.
            if p.labels().len() != 1 {
                return Err(Error::Parsing(format!(
                    "CPD for '{}' must have exactly one target variable.",
                    p.labels().iter().next().unwrap_or(&String::from("?"))
                )));
            }
            // Get child index.
            let x = &p.labels()[0];
            let x_idx = graph
                .labels()
                .get_index_of(x)
                .ok_or_else(|| Error::Parsing(format!("Failed to get index of label '{x}'.")))?;

            // Get parent indices.
            for z in p.conditioning_labels() {
                // Get parent index.
                let z_idx = graph.labels().get_index_of(z).ok_or_else(|| {
                    Error::Parsing(format!("Failed to get index of label '{z}'."))
                })?;
                // Add edge from parent to child.
                graph.add_edge(z_idx, x_idx);
            }
        }

        // Construct the Bayesian network.
        Ok(CatBN::with_optionals(name, description, graph, cpds)?)
    }
}

fn build_ast(pair: Pair<Rule>) -> Result<Network> {
    if pair.as_rule() != Rule::file {
        return Err(Error::Parsing(format!(
            "Expected rule 'file', found '{:?}'",
            pair.as_rule()
        )));
    }

    let mut pair = pair.into_inner();
    let network_pair = pair
        .next()
        .ok_or_else(|| Error::Parsing("Expected 'network' definition".into()))?;

    // Network definitions
    let mut inner = network_pair.into_inner();
    let name = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected network name".into()))?
        .as_str()
        .to_string();
    let mut properties = vec![];
    for p in inner {
        if p.as_rule() == Rule::property {
            properties.push(parse_property(p)?);
        }
    }

    let mut variables = vec![];
    let mut probabilities = vec![];

    for item in pair {
        match item.as_rule() {
            Rule::variable => variables.push(parse_variable(item)?),
            Rule::probability => probabilities.push(parse_probability(item)?),
            _ => {}
        }
    }

    Ok(Network {
        name,
        properties,
        variables,
        probabilities,
    })
}

fn parse_property(pair: Pair<Rule>) -> Result<Property> {
    let mut inner = pair.into_inner();
    let key = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected property key".into()))?
        .as_str()
        .to_string();
    let value = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected property value".into()))?
        .as_str()
        .to_string();

    Ok(Property { key, value })
}

fn parse_variable(pair: Pair<Rule>) -> Result<Variable> {
    let mut inner = pair.into_inner();
    let label = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected variable label".into()))?
        .as_str()
        .to_string();

    // Skip 'type discrete [n] { values } ;'
    let _n = inner.next(); // n
    let values_pair = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected values block".into()))?;
    let states = values_pair
        .into_inner()
        .map(|v| v.as_str().to_string())
        .collect();

    let _semicolon = inner.next(); // ';'

    let properties = inner
        .filter(|p| p.as_rule() == Rule::property)
        .map(parse_property)
        .collect::<Result<_>>()?;

    Ok(Variable {
        label,
        states,
        _properties: properties,
    })
}

fn parse_probability(pair: Pair<Rule>) -> Result<Probability> {
    let mut inner = pair.into_inner();
    let label = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected probability label".into()))?
        .as_str()
        .to_string();

    let mut parents = vec![];
    let mut table = None;
    let mut entries = vec![];

    let mut next = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected parents or content".into()))?;
    if next.as_rule() == Rule::parents {
        parents = next
            .into_inner()
            .next()
            .ok_or_else(|| Error::Parsing("Expected parent list".into()))?
            .into_inner()
            .map(|p| p.as_str().to_string())
            .collect();
        next = inner
            .next()
            .ok_or_else(|| Error::Parsing("Expected probability content".into()))?;
    }

    match next.as_rule() {
        Rule::number_list => {
            table = Some(parse_number_list(next)?);
        }
        Rule::entry => {
            entries.push(parse_entry(next)?);
            for entry in inner {
                if entry.as_rule() == Rule::entry {
                    entries.push(parse_entry(entry)?);
                }
            }
        }
        _ => {}
    }

    let entries = if entries.is_empty() {
        None
    } else {
        Some(entries)
    };

    Ok(Probability {
        label,
        parents,
        table,
        entries,
    })
}

fn parse_entry(pair: Pair<Rule>) -> Result<(Vec<String>, Vec<f64>)> {
    let mut inner = pair.into_inner();
    let values = inner
        .next()
        .ok_or_else(|| Error::Parsing("Expected entry values".into()))?
        .into_inner()
        .map(|v| v.as_str().to_string())
        .collect();
    let probs = parse_number_list(
        inner
            .next()
            .ok_or_else(|| Error::Parsing("Expected entry probabilities".into()))?,
    )?;
    Ok((values, probs))
}

fn parse_number_list(pair: Pair<Rule>) -> Result<Vec<f64>> {
    pair.into_inner()
        .map(|n| {
            n.as_str()
                .parse::<f64>()
                .map_err(|e| Error::Parsing(format!("Failed to parse number: {}", e)))
        })
        .collect()
}

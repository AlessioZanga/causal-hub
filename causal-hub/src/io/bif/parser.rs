use itertools::Itertools;
use ndarray::prelude::*;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{
    models::{BN, CPD, CatBN, CatCPD, DiGraph, Graph, Labelled},
    types::{Map, States},
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
    pub fn parse_str(bif: &str) -> CatBN {
        let network = Self::parse(Rule::file, bif)
            .expect("Failed to parse BIF file.")
            .map(build_ast)
            .next()
            .expect("Failed to parse BIF file.");
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
        let cpds: Vec<_> = network
            .probabilities
            .into_iter()
            .map(|p| {
                // Get the variable of the CPD.
                let variable = States::from_iter([(
                    p.label.clone(),
                    states
                        .get(&p.label)
                        .expect("Failed to get variable states.")
                        .clone(),
                )]);
                // Get the conditioning variables of the CPD.
                let conditioning_variables: States = p
                    .parents
                    .iter()
                    .map(|x| {
                        let states = states.get(x).expect("Failed to get variable states.");
                        (x.to_string(), states.iter().cloned().collect())
                    })
                    .collect();
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
                            .map(|states| &entries[&states])
                            .collect();
                        // Get the shape of the parameters.
                        let shape = (entries.len(), entries[0].len());
                        // Collect the parameters.
                        let parameters: Array1<_> =
                            entries.into_iter().flatten().copied().collect();
                        // Reshape the parameters.
                        parameters
                            .into_shape_with_order(shape)
                            .expect("Failed to reshape parameters.")
                    }
                    _ => unreachable!(),
                };
                // Normalize the parameters so that they sum exactly to 1 by row.
                let parameters = &parameters / parameters.sum_axis(Axis(1)).insert_axis(Axis(1));
                // Construct the CPD.
                CatCPD::new(variable, conditioning_variables, parameters)
            })
            .collect();

        // Construct the graph.
        let mut graph = DiGraph::empty(states.keys());
        cpds.iter().for_each(|p| {
            // Assert the CPD has a single variable in the BIF file.
            assert_eq!(p.labels().len(), 1);
            // Get child index.
            let x = &p.labels()[0];
            let x = graph
                .labels()
                .get_index_of(x)
                .unwrap_or_else(|| panic!("Failed to get index of label '{x}'."));
            // Get parent indices.
            p.conditioning_labels().into_iter().for_each(|z| {
                // Get parent index.
                let z = graph
                    .labels()
                    .get_index_of(z)
                    .unwrap_or_else(|| panic!("Failed to get index of label '{z}'."));
                // Add edge from parent to child.
                graph.add_edge(z, x);
            });
        });

        // Construct the Bayesian network.
        CatBN::with_optionals(name, description, graph, cpds)
    }
}

fn build_ast(pair: Pair<Rule>) -> Network {
    assert_eq!(pair.as_rule(), Rule::file);

    let mut name = String::new();
    let mut properties = vec![];
    let mut variables = vec![];
    let mut probabilities = vec![];

    for item in pair.into_inner() {
        match item.as_rule() {
            Rule::network => {
                let mut inner = item.into_inner();
                name = inner.next().unwrap().as_str().to_string();
                for p in inner {
                    if p.as_rule() == Rule::property {
                        properties.push(parse_property(p));
                    }
                }
            }
            Rule::variable => variables.push(parse_variable(item)),
            Rule::probability => probabilities.push(parse_probability(item)),
            _ => {}
        }
    }

    Network {
        name,
        properties,
        variables,
        probabilities,
    }
}

fn parse_property(pair: Pair<Rule>) -> Property {
    let mut inner = pair.into_inner();
    let key = inner.next().unwrap().as_str().to_string();
    let value = inner.next().unwrap().as_str().to_string();

    Property { key, value }
}

fn parse_variable(pair: Pair<Rule>) -> Variable {
    let mut inner = pair.into_inner();
    let label = inner.next().unwrap().as_str().to_string();

    // Skip 'type discrete [n] { values } ;'
    inner.next(); // n
    let values_pair = inner.next().unwrap(); // { values }
    let states = values_pair
        .into_inner()
        .map(|v| v.as_str().to_string())
        .collect();

    inner.next(); // ';'

    let properties = inner
        .filter(|p| p.as_rule() == Rule::property)
        .map(parse_property)
        .collect();

    Variable {
        label,
        states,
        _properties: properties,
    }
}

fn parse_probability(pair: Pair<Rule>) -> Probability {
    let mut inner = pair.into_inner();
    let label = inner.next().unwrap().as_str().to_string();

    let mut parents = vec![];
    let mut table = None;
    let mut entries = vec![];

    let mut next = inner.next().unwrap();
    if next.as_rule() == Rule::parents {
        parents = next
            .into_inner()
            .next()
            .unwrap()
            .into_inner()
            .map(|p| p.as_str().to_string())
            .collect();
        next = inner.next().unwrap(); // move to table or entry
    }

    match next.as_rule() {
        Rule::number_list => {
            table = Some(parse_number_list(next));
        }
        Rule::entry => {
            entries.push(parse_entry(next));
            for entry in inner {
                if entry.as_rule() == Rule::entry {
                    entries.push(parse_entry(entry));
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

    Probability {
        label,
        parents,
        table,
        entries,
    }
}

fn parse_entry(pair: Pair<Rule>) -> (Vec<String>, Vec<f64>) {
    let mut inner = pair.into_inner();
    let values = inner
        .next()
        .unwrap()
        .into_inner()
        .map(|v| v.as_str().to_string())
        .collect();
    let probs = parse_number_list(inner.next().unwrap());
    (values, probs)
}

fn parse_number_list(pair: Pair<Rule>) -> Vec<f64> {
    pair.into_inner()
        .map(|n| n.as_str().parse::<f64>().unwrap())
        .collect()
}

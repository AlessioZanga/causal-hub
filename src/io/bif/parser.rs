use pest_derive::Parser;

#[derive(Clone, Debug, Default, Parser)]
#[grammar = "io/bif/grammar.pest"]
pub struct BayesianInterchangeFormat {}

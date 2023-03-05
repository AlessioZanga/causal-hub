use std::{io::Error as IOError, path::PathBuf};

use pest::{
    error::Error as ParserError,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::io::File;

#[derive(Clone, Debug, Default, Parser)]
#[grammar = "io/bif/grammar.pest"]
pub struct BayesianInterchangeFormat {}

impl<'a> From<Pair<'a, Rule>> for BayesianInterchangeFormat {
    fn from(pair: Pair<'a, Rule>) -> Self {
        todo!() // FIXME:
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
        let out: Self = Self {}; // FIXME: out.map(|x| x.into()).next().unwrap();

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

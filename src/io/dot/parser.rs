use pest::error::Error as ParserError;
use pest::Parser;
use pest_derive::Parser;

use std::path::Path;

/// DOT parser.
///
/// Implements a [DOT language](https://graphviz.org/doc/info/lang.html) parser.
///
#[derive(Parser)]
#[grammar = "io/dot/grammar.pest"]
pub struct DOT {}

impl DOT {
    pub fn read(path: &Path) -> Result<Self, ParserError<Rule>> {
        // Read file to string.
        let string = std::fs::read_to_string(path)
            .unwrap_or_else(|_| format!("Failed to read file: \"{}\"", path.display()));

        Self::try_from(string)
    }
}

impl TryFrom<String> for DOT {
    type Error = ParserError<Rule>;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Parse the given dot string.
        DOT::parse(Rule::file, string.trim())?;

        Ok(Self {}) // FIXME:
    }
}

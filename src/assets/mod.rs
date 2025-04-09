use crate::{io::BifReader, model::CategoricalBN};

// Asia BIF.
const ASIA: &str = include_str!("asia.bif");

/// Loads the Asia BIF file and returns a `CategoricalBN` struct.
pub fn load_asia() -> CategoricalBN {
    BifReader::read(ASIA)
}

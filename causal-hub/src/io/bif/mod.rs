mod parser;
pub use parser::BifParser;

pub trait BifIO {
    fn from_bif(bif: &str) -> Self;
    fn to_bif(&self) -> String;
    fn read_bif(path: &str) -> Self;
    fn write_bif(&self, path: &str);
}

use std::path::PathBuf;

pub trait Plot {
    type Success;

    type Error;

    fn plot<P>(self, path: P) -> Result<Self::Success, Self::Error>
    where
        P: Into<PathBuf>;
}

use std::path::PathBuf;
use crate::Cargo;

pub struct External;

impl External {
    /// The external content directory
    pub fn dir() -> PathBuf {
        let mut path = Cargo::crate_dir();
        path.push(".external");

        path
    }

    /// Checks that external content is present and valid 
    pub fn is_valid() -> bool {
        Self::dir().exists()
    }
}

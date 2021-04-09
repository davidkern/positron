use std::env;
use std::path::PathBuf;

pub struct Cargo;

impl Cargo {
    /// Get the output directory
    pub fn out_dir() -> PathBuf {
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set"))
    }

    /// Get the directory of the crate currently being built
    pub fn crate_dir() -> PathBuf {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set"))
    }

    /// Get the directory of the workspace
    pub fn workspace_dir() -> PathBuf {
        PathBuf::from(Self::crate_dir().parent().unwrap())
    }

    /// Get the build profile
    pub fn profile() -> String {
        env::var("PROFILE").expect("PROFILE should be set")
    }

    /// Get the target output directory
    pub fn target_dir() -> PathBuf {
        let mut path = Self::workspace_dir();
        path.push("target");
        path.push(Self::profile());

        path
    }
    
    /// Print a warning message
    pub fn warn<S: AsRef<str>>(msg: S) {
        println!("cargo:warning={}", msg.as_ref());
    }
}

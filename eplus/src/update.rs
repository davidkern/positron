use clap::{App, SubCommand};
use crate::Tool;

pub fn tool() -> Tool {
    Tool::new()
        .name("update")
        .help("Updates runtime components")
        .func(|t| {
            println!("run {}", t.name);
        })
}

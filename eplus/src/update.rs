use clap::{App, SubCommand};
use crate::ToolBuilder;

pub fn tool() -> ToolBuilder {
    ToolBuilder::new()
        .name("update")
        .help("Updates runtime components")
        .func(|t| {
            println!("run {}", t.name);
        })
}

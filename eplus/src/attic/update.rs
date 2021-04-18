use clap::{App, SubCommand};
use crate::Tool;

pub struct Update;

impl Tool for Update {
    type Parent = crate::Root;
    fn name(&self) -> &'static str { "update" }
    fn help(&self) -> &'static str { "Updates runtime components" }

    fn run(&mut self, _parent: &Self::Parent) {
        println!("run {}", self.name())
    }
}

mod cli;
mod tool;
mod update;

pub use tool::Tool;
pub use cli::CommandLineInterface;

pub struct Root;

impl Tool for Root {
    type Parent = ();

    fn name(&self) -> &'static str { "e+" }
    fn help(&self) -> &'static str { "Positron Project CLI" }

    fn run(&mut self, parent: &()) {
        println!("run {}", self.name());
    }
}

fn main() {
    let mut root = Root {};

    CommandLineInterface::run(&mut root)
}

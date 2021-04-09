mod cli;
mod tool;
mod update;

pub use tool::Tool;
pub use cli::CommandLineInterface;

fn main() {
    let tool = Tool::new()
        .name("e+")
        .help("Positron Project CLI")
        .tool(crate::update::tool());

    CommandLineInterface::main(tool)
}

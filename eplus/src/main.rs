mod cli;
mod tool;
mod update;

pub use tool::ToolBuilder;
pub use cli::CommandLineInterface;

fn main() {
    let tool = ToolBuilder::new()
        .name("e+")
        .help("Positron Project CLI")
        .subtool(crate::update::tool())
        .func(|t| {
            println!("run {}", t.name);
        })
        .build();

    CommandLineInterface::run(&tool)
}

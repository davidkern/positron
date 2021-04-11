use crate::tool::Tool;

use clap::{App, ArgMatches};

pub struct CommandLineInterface;

impl CommandLineInterface {
    pub fn run(tool: &Tool) {
        let app = Self::build_app(tool);
        let matches = app.get_matches();

        Self::run_tool(&matches, tool);
    }

    fn build_app<'a, 'b>(tool: &'b Tool) -> App<'a, 'b> {
        let mut app = App::new(tool.name.as_str())
            .about(tool.help.as_str());

        app = tool.subtools.iter().fold(app, |app, tool| app.subcommand(Self::build_app(&tool)));

        app
    }

    fn run_tool(matches: &ArgMatches, tool: &Tool) {
        (tool.func)(tool);

        for subtool in tool.subtools.iter() {
            if let Some(submatches) = matches.subcommand_matches(subtool.name.as_str()) {
                Self::run_tool(submatches, subtool);
            }
        }
    }
}

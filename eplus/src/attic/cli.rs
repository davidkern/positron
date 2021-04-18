use crate::tool::Tool;

use clap::{App, ArgMatches};

pub struct CommandLineInterface;

impl CommandLineInterface {
    pub fn run(tool: &mut dyn Tool<Parent=()>) {
        let app = Self::build_app(tool);
        let matches = app.get_matches();

        Self::run_tool(&matches, tool, &());
    }

    fn build_app<'a, 'b, TParent: Tool>(tool: &'b dyn Tool<Parent=TParent>) -> App<'a, 'b> {
        let mut app = App::new(tool.name())
            .about(tool.help());

        //app = tool.subtools.iter().fold(app, |app, tool| app.subcommand(Self::build_app(&tool)));

        app
    }

    fn run_tool<TParent: Tool>(matches: &ArgMatches, tool: &mut dyn Tool<Parent=TParent>, parent: &TParent) {
        tool.run(parent);

        // for subtool in tool.subtools.iter_mut() {
        //     if let Some(submatches) = matches.subcommand_matches(subtool.name.as_str()) {
        //         Self::run_tool(submatches, subtool);
        //     }
        // }
    }
}

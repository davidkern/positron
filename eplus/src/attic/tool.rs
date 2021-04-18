pub struct ToolInstance<'a, TTool: Tool> {
    pub parent: &'a TTool::Parent,
    pub children: Box<[Box<dyn Tool<Parent=TTool>>]>,
    pub tool: TTool,
}

impl<'a, TTool: Tool> ToolInstance<'a, TTool> {
    pub fn run(&mut self, parent: &TTool::Parent) {
        self.tool.run(parent);
    }
}

pub struct ToolInterface {
    name: &'static str,
    help: &'static str,
}

pub trait Tool {
    type Parent: Tool;
    fn interface(&self) -> &'static ToolInterface;
    fn run(&mut self, instance: &mut ToolInstance<'a, Self>);
}

/// Used for the parent of the root Tool so all Tools always have a parent to reference.

impl Tool for () {
    type Parent = ();

    fn interface(&self) -> &'static ToolInterface {
        &ToolInterface {
            name: "",
            help: "",
        }
    }

    fn run(&mut self, parent: &Self::Parent) { }
}

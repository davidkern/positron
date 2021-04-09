pub struct Tool {
    name: &'static str,
    help: &'static str,
    tools: Vec<Self>,
}

impl Tool {
    pub fn new() -> Self {
        Self {
            name: "",
            help: "",
            tools: Vec::new(),
        }
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    pub fn help(mut self, help: &'static str) -> Self {
        self.help = help;
        self
    }

    pub fn tool(mut self, child: Self) -> Self {
        self.tools.push(child);
        self
    }
}

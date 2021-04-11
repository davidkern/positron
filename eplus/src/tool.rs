pub struct ToolBuilder {
    pub name: String,
    pub help: String,
    pub subtools: Vec<ToolBuilder>,
    pub func: Box<dyn Fn(&Tool)>,
}

impl ToolBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            help: String::new(),
            subtools: Vec::new(),
            func: Box::new(|_| {}),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    pub fn help(mut self, help: &str) -> Self {
        self.help = String::from(help);
        self
    }

    pub fn subtool(mut self, tool: Self) -> Self {
        self.subtools.push(tool);
        self
    }

    pub fn func<F: Fn(&Tool) + 'static>(mut self, func: F) -> Self {
        self.func = Box::new(func);
        self
    }

    pub fn build(mut self) -> Tool {
        let mut tool = Tool {
            name: self.name,
            help: self.help,
            subtools: Vec::with_capacity(self.subtools.len()),
            func: self.func,
        };

        for subtool in self.subtools.drain(..) {
           tool.subtools.push(subtool.build());
        }

        tool
    }
}

pub struct Tool {
    pub name: String,
    pub help: String,
    pub subtools: Vec<Self>,
    pub func: Box<dyn Fn(&Self)>,
}

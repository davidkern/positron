use std::cell::RefCell;
use std::marker::PhantomData;
use indextree::Arena;


pub struct Tool {
    tool: Box<dyn Tooling>,
}

pub trait Tooling {
    fn name(&self) -> &'static str;
}

pub struct ToolStore<'store> {
    arena: RefCell<Arena<Tool>>,
    marker: PhantomData<&'store ToolRef<'store>>,
}

impl<'store> ToolStore<'store> {
    pub fn new() -> Self {
        ToolStore {
            arena: RefCell::new(Arena::new()),
            marker: PhantomData,
        }
    }

    pub fn new_tool<T: Tooling + 'static>(&'store self, tool: T) -> ToolRef<'store> {
        let tool = Tool {
            tool: Box::<T>::new(tool),
        };
        
        ToolRef {
            store: self,
            node: self.arena.borrow_mut().new_node(tool),
        }
    }

    pub fn append<T: Tooling + 'static>(
        &'store self,
        parent: &'store ToolRef<'store>,
        child_tool: T) -> &'store Self
    {
        let tool = Tool {
            tool: Box::<T>::new(child_tool),
        };

        self.with_arena(|arena| {
            // TODO: fixme
            // let tool = arena.new_node(tool);
            // parent.node.append(tool, arena);
        });

        self
    }

    pub fn fold_children<F: Fn(R, &ToolRef<'store>) -> R, R>(&'store self, parent: &'store ToolRef<'store>, f: F, initial: R) -> R {
        let arena = self.arena.borrow();
        let mut result = initial;

        for child in parent.node.children(&arena) {
            let tool = ToolRef {
                store: &self,
                node: child,
            };
            result = (f)(result, &tool);
        }

        result
    }

    fn with_arena<F: Fn(&mut Arena<Tool>) -> R, R>(&'store self, f: F) -> R {
        let mut arena = self.arena.borrow_mut();
        (f)(&mut arena)
    }
}

#[derive(Copy, Clone)]
pub struct ToolRef<'a> {
    store: &'a ToolStore<'a>,
    node: indextree::NodeId,
}

impl<'a> ToolRef<'a> {
    pub fn append<T: Tooling + 'static>(&'a self, tool: T) -> &'a Self {
        self.store.append(self, tool);

        self
    }

    // pub fn with_tool<T, F: Fn(&Box<dyn Tooling>) -> T>(self, toolbox: &mut ToolBox, f: F) -> T {
    //     (f)(&toolbox.tools.get(self.0).unwrap().get())
    // }

    // pub fn with_tool_mut<T, F: Fn(&mut Box<dyn Tooling>) -> T>(self, toolbox: &mut ToolBox, f: F) -> T {
    //     (f)(&mut toolbox.tools.get(self.0).unwrap().get_mut())
    // }

    // pub fn with_children<T, F: Fn(&mut Box<dyn Tooling>) -> T>(self, toolbox: &mut ToolBox, f: F) -> T {

    // }
}

#[derive(Default)]
pub struct Root {}

impl Tooling for Root {
    fn name(&self) -> &'static str { "root" }
}

#[derive(Default)]
pub struct Update {}

impl Tooling for Update {
    fn name(&self) -> &'static str { "update" }
}

pub struct ShellInterface {
    tool: Tool,
}

impl ShellInterface {
    pub fn new(tool: Tool) -> Self {
        Self {
            tool,
        }
    }

    pub fn run(&mut self, tool: ToolRef) {
    
    }

    // fn app(&mut self, tool: ToolRef) -> clap::App<'a, 'a> {
    //     unimplemented!()
    // }
}

fn main() {
    let store = ToolStore::new();

    let root = store.new_tool::<Root>(Default::default())
        .append::<Update>(Default::default());
}

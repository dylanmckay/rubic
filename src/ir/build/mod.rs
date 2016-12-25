use {ir, ast};

pub struct Error;

pub fn from_ast(program: ast::Program) -> Result<ir::Program, Error> {
    Context::new().build(program)
}

struct Context
{
    pub classes: Vec<ir::Class>,
}

impl Context
{
    pub fn new() -> Self {
        Context {
            classes: Vec::new(),
        }
    }

    pub fn build(mut self, program: ast::Program)
        -> Result<ir::Program, Error> {
        for item in program.items {
            self.item(item)?;
        }

        unimplemented!();
    }

    fn item(&mut self, item: ast::Item) -> Result<(), Error> {
        match item {
            ast::Item::Class(c) => self.class(c),
            _ => unimplemented!(),
        }
    }

    fn class(&mut self, class: ast::Class) -> Result<(), Error> {
        let ir = ir::Class {
            id: ir::Id::new(),
            name: class.name,
            items: unimplemented!(),
            superclass: unimplemented!(),
        };
    }
}

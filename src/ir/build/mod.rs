pub use self::scope::{Scope, ScopeContext};
pub mod scope;

use {ir, ast};

pub struct Error;

pub fn from_ast(program: ast::Program) -> Result<ir::Program, Error> {
    Context::new().build(program)
}

struct Context
{
    pub classes: Vec<ir::Class>,
    pub scopes: ScopeContext,
}

impl Context
{
    pub fn new() -> Self {
        Context {
            classes: Vec::new(),
            scopes: ScopeContext::new(),
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
        let superclass = class.superclass.map(|class_path| {
            self.scopes.resolve(&class_path).unwrap().expect_class()
        });

        let ir = ir::Class {
            id: ir::Id::new(),
            name: class.name.clone(),
            superclass: superclass,
        };

        self.scopes.current_mut().insert_class(&ir);

        self.scopes.begin(class.name.clone(), ir::ItemId::Class(ir.id.clone()));
        for item in class.items { self.item(item)?; }
        self.classes.push(ir);

        self.scopes.end();
        Ok(())
    }
}

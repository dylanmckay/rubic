pub use self::scope::{Scope, ScopeContext};
pub mod scope;

use {ir, ast};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Error;

pub fn from_ast(program: ast::Program) -> Result<ir::Program, Error> {
    Context::new().build(program)
}

struct Context
{
    pub modules: Vec<ir::Module>,
    pub classes: Vec<ir::Class>,
    pub functions: Vec<ir::Function>,
    pub scopes: ScopeContext,
}

impl Context
{
    pub fn new() -> Self {
        Context {
            modules: Vec::new(),
            classes: Vec::new(),
            functions: Vec::new(),
            scopes: ScopeContext::new(),
        }
    }

    pub fn build(mut self, program: ast::Program)
        -> Result<ir::Program, Error> {
        for item in program.items {
            self.item(item)?;
        }

        let modules: HashMap<_, _> = self.modules.into_iter().map(|a| (a.id.clone(), a)).collect();
        let classes: HashMap<_, _> = self.classes.into_iter().map(|a| (a.id.clone(), a)).collect();
        let functions: HashMap<_, _> = self.functions.into_iter().map(|a| (a.id.clone(), a)).collect();

        Ok(ir::Program {
            modules: modules,
            classes: classes,
            functions: functions,
        })
    }

    fn item(&mut self, item: ast::Item) -> Result<(), Error> {
        match item {
            ast::Item::Module(m) => self.module(m),
            ast::Item::Class(c) => self.class(c),
            ast::Item::Function(f) => self.function(f),
            _ => unimplemented!(),
        }
    }

    fn module(&mut self, module: ast::Module) -> Result<(), Error> {
        let ir = ir::Module {
            id: ir::ModuleId::new(),
            name: module.name.clone(),
        };

        self.scopes.current_mut().insert_module(&ir);

        self.scopes.begin(module.name.clone(), ir::ItemId::Module(ir.id.clone()));
        for item in module.items { self.item(item)?; }
        self.modules.push(ir);

        self.scopes.end();
        Ok(())
    }

    fn class(&mut self, class: ast::Class) -> Result<(), Error> {
        let superclass = class.superclass.map(|class_path| {
            self.scopes.resolve(&class_path).unwrap().expect_class()
        });

        let ir = ir::Class {
            id: ir::ClassId::new(),
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

    fn function(&mut self, function: ast::Function) -> Result<(), Error> {
        let ir = ir::Function {
            id: ir::FunctionId::new(),
            name: function.name.clone(),
            statements: Vec::new(),
        };

        self.scopes.current_mut().insert_function(&ir);
        self.functions.push(ir);

        Ok(())
    }
}

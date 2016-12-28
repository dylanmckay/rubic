pub use self::id::Id;
pub use self::expr::*;
pub use self::stmt::*;

pub mod id;
pub mod expr;
pub mod stmt;
pub mod build;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModuleId(pub Id);
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClassId(pub Id);
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId(pub Id);

impl ModuleId {
    pub fn new() -> Self { ModuleId(Id::new()) }
}

impl ClassId {
    pub fn new() -> Self { ClassId(Id::new()) }
}

impl FunctionId {
    pub fn new() -> Self { FunctionId(Id::new()) }
}

use std::collections::HashMap;

/// A Ruby program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program
{
    pub modules: HashMap<ModuleId, Module>,
    pub classes: HashMap<ClassId, Class>,
    pub functions: HashMap<FunctionId, Function>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemId
{
    Module(ModuleId),
    Class(ClassId),
    Function(FunctionId),
}

/// A module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module
{
    pub id: ModuleId,
    /// The name of the module.
    pub name: String,
}

/// A class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class
{
    pub id: ClassId,
    /// The name of the class.
    pub name: String,
    /// The parent class.
    pub superclass: Option<ClassId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function
{
    pub id: FunctionId,
    /// The name of the function.
    pub name: String,
    /// The statements in the function.
    pub statements: Vec<Stmt>,
}

/// An identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier(pub String);

/// A constant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constant(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Argument
{
    /// A standard positional argument.
    Positional(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter
{
    pub name: String,
    pub default: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lvalue
{
    Variable { name: String },
    Field { name: String },
}

impl Program
{
    pub fn new() -> Self {
        Program {
            modules: HashMap::new(),
            classes: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

impl Module
{
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Module { id: ModuleId::new(), name: name.into() }
    }
}

impl Class
{
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Class { id: ClassId::new(), name: name.into(), superclass: None }
    }
}

impl ItemId
{
    pub fn expect_class(&self) -> ClassId {
        if let ItemId::Class(ref id) = *self {
            id.clone()
        } else {
            panic!("not a class");
        }
    }
}

pub use self::id::Id;
pub use self::expr::*;
pub use self::stmt::*;

pub mod id;
pub mod expr;
pub mod stmt;

pub type ModuleId = Id<Module>;
pub type ClassId = Id<Class>;
pub type FunctionId = Id<Function>;

/// A Ruby program.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program
{
    pub items: Vec<Item>,
}

/// An item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item
{
    Module(Module),
    Class(Class),
    Function(Function),
    Stmt(Stmt),
}

/// A module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module
{
    pub id: ModuleId,
    /// The name of the module.
    pub name: String,
    /// The items contained in the module.
    pub items: Vec<Item>,
}

/// A class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class
{
    pub id: ClassId,
    /// The name of the class.
    pub name: String,
    /// The items contained in the class.
    pub items: Vec<Item>,
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
        Program { items: Vec::new() }
    }
}

impl Module
{
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Module { id: Id::new(), name: name.into(), items: Vec::new() }
    }
}

impl Class
{
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Class { id: Id::new(), name: name.into(), items: Vec::new(), superclass: None }
    }
}

impl Into<Item> for Class { fn into(self) -> Item { Item::Class(self) } }
impl Into<Item> for Module { fn into(self) -> Item { Item::Module(self) } }
impl Into<Item> for Function { fn into(self) -> Item { Item::Function(self) } }
impl Into<Item> for Stmt { fn into(self) -> Item { Item::Stmt(self) } }

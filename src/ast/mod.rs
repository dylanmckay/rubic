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
}

/// A module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module
{
    /// The name of the module.
    pub name: String,
    /// The items contained in the module.
    pub items: Vec<Item>,
}

/// A class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class
{
    /// The name of the class.
    pub name: String,
    /// The items contained in the class.
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function
{
    /// The name of the function.
    pub name: String,
    /// The statements in the function.
    pub statements: Vec<Statement>,
}

/// Unimplemented.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement;

impl Program
{
    pub fn new() -> Self {
        Program { items: Vec::new() }
    }
}

impl Module
{
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Module { name: name.into(), items: Vec::new() }
    }
}

impl Class
{
    pub fn new<S>(name: S) -> Self where S: Into<String> {
        Class { name: name.into(), items: Vec::new() }
    }
}

impl Into<Item> for Class { fn into(self) -> Item { Item::Class(self) } }
impl Into<Item> for Module { fn into(self) -> Item { Item::Module(self) } }
impl Into<Item> for Function { fn into(self) -> Item { Item::Function(self) } }

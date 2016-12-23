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
}

/// A module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module
{
    /// The name of the module.
    pub name: String,
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

impl Program
{
    pub fn new() -> Self {
        Program { items: Vec::new() }
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

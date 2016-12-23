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
}

impl Program
{
    pub fn new() -> Self {
        Program { items: Vec::new() }
    }
}

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token
{
    /// A standalone work.
    Word(String),
    /// A symbol.
    Symbol(&'static str),
    /// A string in quotes.
    String(String),
    /// An integer.
    Integer(i64),
    EndOfLine,
    EndOfFile,
}

impl Token
{
    /// The `class` keyword.
    pub fn class() -> Token { Token::Word("class".to_owned()) }
    pub fn module() -> Token { Token::Word("module".to_owned()) }
    pub fn def() -> Token { Token::Word("def".to_owned()) }
    pub fn end() -> Token { Token::Word("end".to_owned()) }

    pub fn left_paren() -> Token { Token::Symbol("(") }
    pub fn right_paren() -> Token { Token::Symbol(")") }

    pub fn less_than() -> Token { Token::Symbol("<") }
    pub fn greater_than() -> Token { Token::Symbol(">") }

    pub fn colon() -> Token { Token::Symbol(":") }
    pub fn comma() -> Token { Token::Symbol(",") }
    pub fn negate() -> Token { Token::Symbol("-") }

    pub fn rocket() -> Token { Token::Symbol("=>") }
    pub fn equals() -> Token { Token::Symbol("=") }

    pub fn is_terminator(&self) -> bool {
        match *self {
            Token::EndOfLine |
                Token::Symbol(";") => true,
            _ => false,
        }
    }
}

impl fmt::Display for Token
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Word(ref word) => word.fmt(fmt),
            Token::Symbol(sym) => sym.fmt(fmt),
            Token::String(ref s) => write!(fmt, "\"{}\"", s),
            Token::Integer(i) => i.fmt(fmt),
            Token::EndOfLine => "end-of-line".fmt(fmt),
            Token::EndOfFile => "end-of-file".fmt(fmt),
        }
    }
}

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
    EndOfLine,
    EndOfFile,
}

impl Token
{
    /// The `class` keyword.
    pub fn class() -> Token { Token::Word("class".to_owned()) }
    pub fn end() -> Token { Token::Word("end".to_owned()) }
}

impl fmt::Display for Token
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Word(ref word) => word.fmt(fmt),
            Token::Symbol(sym) => sym.fmt(fmt),
            Token::String(ref s) => write!(fmt, "\"{}\"", s),
            Token::EndOfLine => "end-of-line".fmt(fmt),
            Token::EndOfFile => "end-of-file".fmt(fmt),
        }
    }
}

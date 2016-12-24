use ast;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr
{
    Assignment(AssignmentExpr),
    Paren(ParenExpr),
    Call(CallExpr),
    StringLiteral(StringLiteral),
    IntegerLiteral(IntegerLiteral),
    Symbol(SymbolExpr),
}

/// A `a = b` expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssignmentExpr
{
    /// The LHS
    pub assignee: ast::Path,
    /// The new value.
    pub value: Box<Expr>,
}

/// A parenthesized expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParenExpr
{
    /// The expression in the parentheses.
    pub inner: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallExpr
{
    pub callee: ast::Path,
    pub arguments: Vec<ast::Argument>
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringLiteral
{
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerLiteral
{
    pub value: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolExpr
{
    pub name: String,
}

macro_rules! expr_boilerplate {
    ($ty:ty => $shortname:ident) => {
        impl Into<Expr> for $ty {
            fn into(self) -> Expr {
                Expr::$shortname(self)
            }
        }
    }
}

expr_boilerplate!(AssignmentExpr => Assignment);
expr_boilerplate!(ParenExpr => Paren);
expr_boilerplate!(CallExpr => Call);
expr_boilerplate!(StringLiteral => StringLiteral);
expr_boilerplate!(IntegerLiteral => IntegerLiteral);
expr_boilerplate!(SymbolExpr => Symbol);

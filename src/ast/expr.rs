use ast;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr
{
    Assignment(AssignmentExpr),
    Call(CallExpr),
    StringLiteral(StringLiteral),
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
expr_boilerplate!(CallExpr => Call);
expr_boilerplate!(StringLiteral => StringLiteral);

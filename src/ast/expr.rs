use ast;

/// An expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr
{
    Assignment(AssignmentExpr),
    Call(CallExpr),
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
    // FIXME: implement arguments.
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

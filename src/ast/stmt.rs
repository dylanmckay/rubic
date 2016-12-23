use ast;

/// A statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt
{
    Expr(ast::Expr),
}

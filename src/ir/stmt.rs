use ir;

/// A statement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt
{
    Expr(ir::Expr),
}

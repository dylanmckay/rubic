use parse;
use ast;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        UnexpectedToken(got: parse::Token, expected: Vec<parse::Token>) {
            description("unexpected token")
            display("unexpected token: got '{:?}' but expected '{:?}'",
                    got, expected)
        }
        UnexpectedExpr(got: ast::Expr, expected: String) {
            description("unexpected expression")
            display("unexpected expression: got '{:?}' but expected {}", got, expected)
        }
    }
}

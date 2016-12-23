use parse;

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
    }
}

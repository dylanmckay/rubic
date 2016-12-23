use parse;

error_chain! {
    links {
        Parse(parse::Error, parse::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
    }
}

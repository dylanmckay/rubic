use parse::Token;
use std::iter::Peekable;

/// A list of symbols.
const SYMBOLS: &'static [&'static str] = &[
    ".", ";", "{", "}"
];

/// A tokenizer.
pub struct Tokenizer<I: Iterator<Item=char>>
{
    chars: Peekable<I>,
    /// We generate a fake new line at the end of every program.
    sent_last_new_line: bool,
}

impl<I> Tokenizer<I> where I: Iterator<Item=char>
{
    /// Creates a new tokenizer.
    pub fn new(characters: I) -> Self {
        Tokenizer { chars: characters.peekable(), sent_last_new_line: false }
    }

    fn read_token(&mut self) -> Option<Token> {
        self.eat_whitespace();

        let peeked_char = if let Some(&c) = self.chars.peek() { c } else { return None };

        if peeked_char.is_alphabetic() {
            Some(self.read_word())
        } else if peeked_char == '\n' {
            self.chars.next(); // Eat new line
            Some(Token::EndOfLine)
        } else if peeked_char == '"' || peeked_char == '\'' {
            Some(self.read_string())
        } else if SYMBOLS.iter().any(|sym| sym.starts_with(peeked_char)) {
            let matches: Vec<_> = SYMBOLS.iter().filter(|sym| sym.starts_with(peeked_char)).collect();

            if matches.iter().any(|sym| sym.len() > 1) {
                // Multi-char symbols are unimplemented.
                unimplemented!();
            } else { // We matched with a single-char symbol.
                debug_assert_eq!(matches.len(), 1, "matched with multiple symbols");

                self.chars.next(); // Eat the single-char symbol.
                Some(Token::Symbol(matches[0]))
            }
        } else {
            unimplemented!();
        }
    }

    fn eat_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c != '\n' && c.is_whitespace() {
                self.chars.next(); // Eat the character.
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> Token {
        let mut chars = Vec::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '-' {
                self.chars.next(); // Eat the char
                chars.push(c)
            } else {
                break;
            }
        }

        Token::Word(chars.into_iter().collect())
    }

    fn read_string(&mut self) -> Token {
        self.chars.next(); // Eat the quote.

        let mut chars = Vec::new();

        while let Some(&c) = self.chars.peek() {
            if c != '"' && c != '\'' {
                self.chars.next(); // Eat the char
                chars.push(c)
            } else {
                self.chars.next(); // Eat the quote.
                break;
            }
        }

        Token::String(chars.into_iter().collect())
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<I>
{
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.read_token() {
            Some(token)
        } else {
            if self.sent_last_new_line {
                None
            } else {
                self.sent_last_new_line = true;
                Some(Token::EndOfLine)
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use parse::Token;

    fn tokenize(s: &str) -> Vec<Token> {
        let t = Tokenizer::new(s.chars());
        t.collect()
    }

    #[test]
    fn can_read_simple_word() {
        assert_eq!(tokenize("abcdef"), vec![Token::Word("abcdef".to_owned()),
                                            Token::EndOfLine]);
    }

    #[test]
    fn can_handle_whitespace_at_start_of_word() {
        assert_eq!(tokenize("     abcdef"), vec![Token::Word("abcdef".to_owned()),
                                                 Token::EndOfLine]);
    }

    #[test]
    fn can_read_multiple_words() {
        assert_eq!(tokenize("\tabcdef hg"), vec![Token::Word("abcdef".to_owned()),
                                                 Token::Word("hg".to_owned()),
                                                 Token::EndOfLine]);
    }

    #[test]
    fn considers_dashes_a_part_of_words() {
        assert_eq!(tokenize("\tabcdef-hg"), vec![Token::Word("abcdef-hg".to_owned()), Token::EndOfLine]);
    }

    #[test]
    fn can_read_single_dot() {
        assert_eq!(tokenize("."), vec![Token::Symbol("."), Token::EndOfLine]);
    }

    #[test]
    fn can_read_multiple_dots() {
        assert_eq!(tokenize("..."), vec![Token::Symbol("."),
                                         Token::Symbol("."),
                                         Token::Symbol("."),
                                         Token::EndOfLine]);
    }

    #[test]
    fn can_read_new_line() {
        assert_eq!(tokenize(" \nb"), vec![Token::EndOfLine, Token::Word("b".to_owned()), Token::EndOfLine]);
    }

    #[test]
    fn can_read_string() {
        assert_eq!(tokenize("\"hello\""), vec![Token::String("hello".to_owned()),
                                               Token::EndOfLine]);
    }
}

use std::str::Chars;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
}

#[derive(Debug)]
pub enum Token<'a> {
    Operator(Operator),
    Number(i32),
    Error(&'a str),
}

pub struct Lexer<'a> {
    program: &'a str,
    program_iter: Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    // TODO: look into these lifetime parameters
    type Item = Token<'a>;

    // TODO: why no 'a on the mut on the lhs?
    fn next(&mut self) -> Option<Token<'a>> {
        // TODO: is this clone bad? for peeking...
        // TODO: better way to write this?
        while let Some(chr) = self.program_iter.clone().next() {
            if chr.is_whitespace() {
                self.program_iter.next();
            } else {
                break;
            }
        }

        match self.program_iter.next() {
            None => None,
            _ => Some(Token::Error("not implemented")),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(program: &str) -> Lexer {
        Lexer {
            program: program,
            program_iter: program.chars(),
        }
    }
}

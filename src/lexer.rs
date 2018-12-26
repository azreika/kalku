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
    idx: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &str) -> Lexer {
        Lexer {
            program: program,
            idx: 0,
        }
    }

    // TODO: implement as an iterator?
    pub fn next(&mut self) -> Token {
        self.idx += 1;
        Token::Error("not implemented")
    }

    pub fn has_next(&self) -> bool {
        self.idx < self.program.len()
    }
}

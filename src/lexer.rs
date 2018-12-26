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

impl<'a> Iterator for Lexer<'a> {
    // TODO: look into these lifetime parameters
    type Item = Token<'a>;

    // TODO: why no 'a on the mut on the lhs?
    fn next(&mut self) -> Option<Token<'a>> {
        if self.idx >= self.program.len() {
            return None;
        }

        self.idx += 1;
        Some(Token::Error("not implemented"))
    }
}

impl<'a> Lexer<'a> {
    pub fn new(program: &str) -> Lexer {
        Lexer {
            program: program,
            idx: 0,
        }
    }
}

use lexer;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &str) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(program),
        }
    }
}

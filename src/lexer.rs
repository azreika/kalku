use std::str::Chars;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
}

#[derive(Debug)]
pub enum Token<'a> {
    Op(Operator),
    Number(i32),
    LeftBracket,
    RightBracket,
    Error(&'a str),
}

pub struct Lexer<'a> {
    program: &'a str,
    it: Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    // TODO: look into these lifetime parameters
    type Item = Token<'a>;

    // TODO: why no 'a on the mut on the lhs?
    fn next(&mut self) -> Option<Token<'a>> {
        // TODO: is this clone bad? for peeking...
        // TODO: better way to write this?
        while let Some(chr) = self.it.clone().next() {
            if chr.is_whitespace() {
                self.it.next();
            } else {
                break;
            }
        }

        match self.it.next() {
            Some('+') => Some(Token::Op(Operator::Plus)),
            Some('-') => Some(Token::Op(Operator::Minus)),
            Some('*') => Some(Token::Op(Operator::Multiply)),
            Some('(') => Some(Token::LeftBracket),
            Some(')') => Some(Token::RightBracket),
            Some(x) if x.is_numeric() => {
                let mut next_num = String::from("");
                next_num.push(x);
                while let Some(chr) = self.it.clone().next() {
                    if chr.is_numeric() {
                        next_num.push(chr);
                        self.it.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Number(next_num.parse().unwrap()))
            },
            None => None,
            _ => Some(Token::Error("unexpected character")),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(program: &str) -> Lexer {
        Lexer {
            program: program,
            it: program.chars(),
        }
    }

    // TODO: implemnet this properly
    pub fn peek(&mut self) -> Option<Token> {
        let backup = self.it.clone();
        let result = self.next();
        self.it = backup;
        result
    }

    pub fn reset(&mut self) {
        self.it = self.program.chars();
    }
}

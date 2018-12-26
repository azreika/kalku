use lexer::{self,Operator,Token};
use std::fmt;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

#[derive(Debug)]
pub enum AstNodeType {
    // TODO: is Box the way to go.unwrap()
    BinaryOperation(Operator, Box<AstNode>, Box<AstNode>),
    Constant(i32),
}

#[derive(Debug)]
pub enum ParseError<'a> {
    ExpectedToken(Token<'a>),
    UnexpectedToken(Token<'a>),
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct AstNode {
    node_type: AstNodeType,
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.node_type {
            AstNodeType::BinaryOperation(ref op, ref left, ref right) => {
                let op = match op {
                    Operator::Plus => "+",
                    Operator::Minus => "-",
                    Operator::Multiply => "*",
                };

                write!(f, "({} {} {})", left, op, right)
            },

            AstNodeType::Constant(ref val) => write!(f, "{}", val),
        }
    }
}

impl AstNode {
    pub fn evaluate(&self) -> i32 {
        match self.node_type {
            // TODO: point of these refs? what do they mean?
            AstNodeType::BinaryOperation(ref op, ref left, ref right) => {
                match op {
                    Operator::Plus => left.evaluate() + right.evaluate(),
                    Operator::Minus => left.evaluate() - right.evaluate(),
                    Operator::Multiply => left.evaluate() * right.evaluate(),
                }
            },
            AstNodeType::Constant(value) => value,
        }
    }
}

// TODO: ERROR HANDLING!!! GEt rid of the unwraps!!! for some reason get the immutable problem
// though when it is removed

impl<'a> Parser<'a> {
    pub fn new(program: &str) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(program),
        }
    }

    pub fn parse(&mut self) -> Result<AstNode,ParseError> {
        self.parse_expression()
    }

    // TODO: why is a lifetime parameter needed on the RHS here?
    fn parse_expression(&mut self) -> Result<AstNode,ParseError> {
        let mut expr = self.parse_term().unwrap();

        loop {
            match self.lexer.peek() {
                Some(Token::Op(op)) => {
                    if op != Operator::Plus && op != Operator::Minus {
                        break Ok(expr)
                    }
                },
                _ => break Ok(expr),
            };

            expr = match self.lexer.next() {
                Some(tok) => {
                    match tok {
                        Token::Op(op) => {
                            let rhs = self.parse_term().unwrap();
                            AstNode {
                                node_type: AstNodeType::BinaryOperation(op, Box::new(expr), Box::new(rhs)),
                            }
                        }

                        // TODO: ERROR HANDLING!!!
                        _ => break Err(ParseError::UnexpectedToken(tok)),
                    }
                },

                // TODO: ERROR HANDLING!!!
                _ => break Err(ParseError::UnexpectedEOF),
            };
        }
    }

    fn parse_term(&mut self) -> Result<AstNode,ParseError> {
        let mut term = self.parse_factor().unwrap();

        loop {
            match self.lexer.peek() {
                // TODO: better way to write this.unwrap()
                Some(Token::Op(op)) => {
                    if op != Operator::Multiply {
                        break Ok(term)
                    }
                },
                _ => break Ok(term),
            };

            term = match self.lexer.next() {
                Some(tok) => {
                    match tok {
                        Token::Op(op) => {
                            let rhs = self.parse_factor().unwrap();
                            AstNode {
                                node_type: AstNodeType::BinaryOperation(op, Box::new(term), Box::new(rhs)),
                            }
                        },

                        _ => break Err(ParseError::UnexpectedToken(tok)),
                    }
                },

                _ => break Err(ParseError::UnexpectedEOF),
            };
        }
    }

    fn parse_factor(&mut self) -> Result<AstNode,ParseError> {
        match self.lexer.next() {
            Some(Token::LeftBracket) => {
                let result = self.parse_expression().unwrap();
                // final token should be a right bracket
                match self.lexer.next() {
                    Some(Token::RightBracket) => Ok(result),
                    // TODO: clean up error handling
                    _ => Err(ParseError::ExpectedToken(Token::RightBracket)),
                }
            },
            Some (Token::Number(val)) => {
                Ok(AstNode {
                    node_type: AstNodeType::Constant(val),
                })
            },
            Some(tok) => Err(ParseError::UnexpectedToken(tok)),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}

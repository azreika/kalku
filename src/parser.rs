use lexer::{self,Operator,Token};
use std::fmt;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

#[derive(Debug)]
pub enum AstNodeType {
    // TODO: is Box the way to go?
    BinaryOperation(Operator, Box<AstNode>, Box<AstNode>),
    Negation(Box<AstNode>),
    Constant(i32),
}

#[derive(Debug,PartialEq)]
// TODO: add tokens here!!! BUT THEN get that weird borrow error -- or maybe just strings?
// TODO: expand these errors so we get actual error messages
pub enum ParseError {
    GeneralError,
}

// TODO: add a function that creates a node for you
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
            AstNodeType::Negation(ref term) => write!(f, "(-{})", term),
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
            AstNodeType::Negation(ref term) => -term.evaluate(),
            AstNodeType::Constant(value) => value,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(program: &str) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(program),
        }
    }

    pub fn parse(&mut self) -> Result<AstNode,ParseError> {
        let expr = self.parse_expression()?;
        match self.lexer.peek() {
            None => Ok(expr),
            _ => Err(ParseError::GeneralError),
        }
    }

    // TODO: why is a lifetime parameter needed on the RHS here?
    fn parse_expression(&mut self) -> Result<AstNode,ParseError> {
        let mut expr = self.parse_term()?;

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
                            let rhs = self.parse_term()?;
                            AstNode {
                                node_type: AstNodeType::BinaryOperation(op, Box::new(expr), Box::new(rhs)),
                            }
                        }
                        _ => break Err(ParseError::GeneralError),
                    }
                },
                _ => break Err(ParseError::GeneralError),
            };
        }
    }

    fn parse_term(&mut self) -> Result<AstNode,ParseError> {
        if let Some(Token::Op(op)) = self.lexer.peek() {
            if op == Operator::Minus {
                self.lexer.next();
                let inner_term = self.parse_term()?;
                let term = AstNode {
                    node_type: AstNodeType::Negation(Box::new(inner_term)),
                };
                return Ok(term);
            }
        }

        let mut term = self.parse_factor()?;

        loop {
            match self.lexer.peek() {
                // TODO: better way to write this?
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
                            let rhs = self.parse_factor()?;
                            AstNode {
                                node_type: AstNodeType::BinaryOperation(op, Box::new(term), Box::new(rhs)),
                            }
                        },
                        _ => break Err(ParseError::GeneralError),
                    }
                },
                _ => break Err(ParseError::GeneralError),
            };
        }
    }

    fn parse_factor(&mut self) -> Result<AstNode,ParseError> {
        match self.lexer.next() {
            Some(Token::LeftBracket) => {
                let result = self.parse_expression()?;
                // final token should be a right bracket
                match self.lexer.next() {
                    Some(Token::RightBracket) => Ok(result),
                    _ => Err(ParseError::GeneralError),
                }
            },
            Some (Token::Number(val)) => Ok(AstNode {
                    node_type: AstNodeType::Constant(val),
            }),
            _ => Err(ParseError::GeneralError),
        }
    }
}

use lexer::{self,Operator,Token};

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

#[derive(Debug)]
pub enum AstNodeType {
    // TODO: is Box the way to go?
    BinaryOperation(Operator, Box<AstNode>, Box<AstNode>),
    Constant(i32),
}

#[derive(Debug)]
pub struct AstNode {
    node_type: AstNodeType,
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

impl<'a> Parser<'a> {
    pub fn new(program: &str) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(program),
        }
    }

    pub fn parse(&mut self) -> AstNode {
        self.parseExpression()
    }

    // TODO: why is a lifetime parameter needed on the RHS here?
    fn parseExpression(&mut self) -> AstNode {
        let mut expr = self.parseTerm();

        loop {
            match self.lexer.peek() {
                Some(Token::Op(op)) => {
                    if op != Operator::Plus && op != Operator::Minus {
                        break;
                    }
                },
                _ => break,
            };

            expr = match self.lexer.next() {
                Some(tok) => {
                    match tok {
                        Token::Op(op) => {
                            AstNode {
                                node_type: AstNodeType::BinaryOperation(op, Box::new(expr), Box::new(self.parseTerm())),
                            }
                        }

                        // TODO: ERROR HANDLING!!!
                        _ => AstNode {
                            node_type: AstNodeType::Constant(0),
                        },
                    }
                },

                // TODO: ERROR HANDLING!!!
                _ => AstNode {
                    node_type: AstNodeType::Constant(0),
                },
            };
        }

        expr
    }

    fn parseTerm(&mut self) -> AstNode {
        let mut term = self.parseFactor();

        loop {
            match self.lexer.peek() {
                Some(Token::Op(op)) => {
                    if op != Operator::Multiply {
                        break;
                    }
                },
                _ => break,
            };

            term = match self.lexer.next() {
                Some(tok) => {
                    match tok {
                        Token::Op(op) => {
                            AstNode {
                                node_type: AstNodeType::BinaryOperation(op, Box::new(term), Box::new(self.parseFactor())),
                            }
                        }

                        // TODO: ERROR HANDLING!!!
                        _ => AstNode {
                            node_type: AstNodeType::Constant(0),
                        },
                    }
                },

                // TODO: ERROR HANDLING!!!
                _ => AstNode {
                    node_type: AstNodeType::Constant(0),
                },
            };
        }

        term
    }

    fn parseFactor(&mut self) -> AstNode {
        match self.lexer.next() {
            Some(Token::LeftBracket) => {
                let result = self.parseExpression();
                // TODO: should be a right bracket!!
                self.lexer.next();
                result
            },

            Some (Token::Number(val)) => {
                AstNode {
                    node_type: AstNodeType::Constant(val),
                }
            },

            // TODO: error handling!
            x => {
                println!("FACTOR ERROR! GOT BACK {:?}", x);
                AstNode {
                    node_type: AstNodeType::Constant(0),
                }
            },
        }
    }
}

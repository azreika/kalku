use lexer::{self,Operator,Token};

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

#[derive(Debug)]
pub enum AstNodeType {
    BinaryOperation(Operator, Box<AstNode>, Box<AstNode>),
    Constant(i32),
}

#[derive(Debug)]
pub struct AstNode {
    node_type: AstNodeType,
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
        // TODO: probably dont veen need peek!
        let left = match self.lexer.next() {
            Some(Token::LeftBracket) => {
                // parse the first part
                let currExpr = self.parseExpression();

                //TODO: ERROR HANDLING!!!! this should be a right bracket
                self.lexer.next();

                // TODO: do the remaining rhs if possible
                currExpr
            },
            Some(Token::Number(value)) => {
                AstNode {
                    node_type: AstNodeType::Constant(value),
                }
            },
            // TODO: SHOULD ERROR out on Error, Op, or RightBracket
            _ => AstNode {
                // TODO: write this up
                node_type: AstNodeType::Constant(0)
            }
        };

        let middle = self.lexer.next();
        let op: Operator;
        match middle {
            Some(Token::Op(x)) => op = x,
            _ => return left,
        }

        let right = self.parseExpression();
        AstNode {
            node_type: AstNodeType::BinaryOperation(op, Box::new(left), Box::new(right)),
        }
    }
}

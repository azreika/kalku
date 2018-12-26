use lexer;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
}

#[derive(Debug)]
pub enum AstNodeType<'a> {
    BinaryOperation(Operator, &'a AstNode<'a>, &'a AstNode<'a>),
    Constant(i32),
}

#[derive(Debug)]
pub struct AstNode<'a> {
    node_type: AstNodeType<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &str) -> Parser {
        Parser {
            lexer: lexer::Lexer::new(program),
        }
    }

    pub fn parse(&mut self) -> AstNode<'a> {
        AstNode {
            // TODO: write this up
            node_type: AstNodeType::Constant(0)
        }
    }
}

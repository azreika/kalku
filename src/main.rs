#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
}

#[derive(Debug)]
enum Token<'a> {
    Operator(Operator),
    Number(i32),
    Error(&'a str),
}

struct Lexer<'a> {
    program: &'a str,
    idx: usize,
}

impl<'a> Lexer<'a> {
    fn new(program: &str) -> Lexer {
        Lexer {
            program: program,
            idx: 0,
        }
    }

    // TODO: implement as an iterator?
    fn next(&mut self) -> Token {
        self.idx += 1;
        Token::Error("not implemented")
    }

    fn has_next(&self) -> bool {
        self.idx < self.program.len()
    }
}

fn main() {
    run_tests();
}

fn run_tests() {
    test_program(&String::from("1"));
    test_program(&String::from("1+2"));
    test_program(&String::from("1-2"));
    test_program(&String::from("1*2"));
}

fn test_program(program: &str) {
    println!("Testing program: [{}]", program);
    let mut lexer = Lexer::new(&program);
    while lexer.has_next() {
        println!("TOKEN: {:?}", lexer.next());
    }
}

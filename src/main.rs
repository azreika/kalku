mod lexer;
mod parser;

fn main() {
    run_tests();
}

fn run_tests() {
    test_program(&String::from("1"));
    test_program(&String::from("1+2"));
    test_program(&String::from("1-2"));
    test_program(&String::from("1*2"));
    test_program(&String::from("      1    + 2 "));
    test_program(&String::from("10 + 17"));
    test_program(&String::from("10 / 17"));
    test_program(&String::from("(1+2)"));
}

fn test_program(program: &str) {
    println!("Testing program: [{}]", program);

    // test lexing
    println!("--- lexer ---");
    let mut lexer = lexer::Lexer::new(&program);
    while let Some(token) = lexer.next() {
        println!("TOKEN: {:?}", token);
    }

    // test parsing
    println!("--- parser ---");
    let mut parser = parser::Parser::new(&program);
    let result = parser.parse();
    println!("{:?}", result);
}

mod lexer;
mod parser;

fn main() {
    run_tests();
}

fn run_tests() {
    test_program(&String::from("1"), Ok(1));
    test_program(&String::from("(1)"), Ok(1));
    test_program(&String::from("1+2"), Ok(3));
    test_program(&String::from("1-2"), Ok(-1));
    test_program(&String::from("1*2"), Ok(2));
    test_program(&String::from("      1    + 2 "), Ok(3));
    test_program(&String::from("10 + 17"), Ok(27));
    // TODO: Fix error handling
    // test_program(&String::from("10 / 17"), Err("NULL"));
    test_program(&String::from("(1+2)"), Ok(3));
    test_program(&String::from("1*2 + 3"), Ok(5));
    test_program(&String::from("1*(2+3) + 3"), Ok(8));
}

fn test_program(program: &str, expected: Result<i32,String>) {
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

    // test evaluation
    println!("--- eval ---");
    println!("{}", result.evaluate());

    // TODO: FIX THIS ERROR CHECK
    assert_eq!(result.evaluate(), expected.unwrap());
}

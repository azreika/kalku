mod lexer;

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
    let mut lexer = lexer::Lexer::new(&program);
    while lexer.has_next() {
        println!("TOKEN: {:?}", lexer.next());
    }
}

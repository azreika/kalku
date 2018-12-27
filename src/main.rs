use std::io::{self, BufRead, Write};

mod lexer;
mod parser;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // start the REPL
    loop {
        print!("> ");
        io::stdout().flush().expect("error flushing to stdout");

        match lines.next() {
            // read some input
            Some(line) => {
                match line {
                    // program has been read in
                    Ok(program) => {
                        match parser::Parser::new(&program).parse() {
                            Ok(ast) => println!("{}", ast.evaluate()),
                            _ => println!("error: invalid input"),
                        }
                    },

                    // IO error
                    _ => println!("error: could not read input"),
                }
            },

            // end of input
            None => break,
        }
    }

    // finished
    println!("bye!");
}

#[cfg(test)]
mod tests {
    use super::parser::{self,ParseError};

    fn test_evaluate(program: &str) -> Result<i32,ParseError> {
        let mut parser = parser::Parser::new(&program);
        let ast = parser.parse()?;
        Ok(ast.evaluate())
    }

    fn expect_failure(program: &str, err: ParseError) {
        let result = test_evaluate(program).err().unwrap();
        assert_eq!(result, err);
    }

    fn expect_success(program: &str, value: i32) {
        let result = test_evaluate(program).unwrap();
        assert_eq!(result, value);
    }

    #[test]
    fn simple_correct() {
        expect_success("1", 1);
        expect_success("1+2", 3);
        expect_success("2+1", 3);
        expect_success("1+2+3", 6);
        expect_success("1-2", -1);
        expect_success("1*2", 2);
        expect_success("      1    + 2 ", 3);
        expect_success(" 10 + 17 ", 27);
        expect_success(" ( 1 + 2 ) ", 3);
        expect_success(" 1*2 + 3 ", 5);
        expect_success(" 1*(2+3) + 3 ", 8);
        expect_success("1 +2 - 3 + (4 + 5) - 3", 6);
        expect_success("-1-2", -3);
        expect_success("-1 - (1+2) -- 2", -2);
    }

    #[test]
    fn simple_incorrect() {
        expect_failure("1/7", ParseError::GeneralError);
        expect_failure("1+", ParseError::GeneralError);
        expect_failure("(1", ParseError::GeneralError);
        expect_failure("-", ParseError::GeneralError);
    }
}

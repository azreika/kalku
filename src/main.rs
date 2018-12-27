mod lexer;
mod parser;

fn main() {
    // TODO: add driver here
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
    }

    #[test]
    fn simple_incorrect() {
        expect_failure("1/7", ParseError::GeneralError);
        expect_failure("1+", ParseError::GeneralError);
        expect_failure("(1", ParseError::GeneralError);
    }
}

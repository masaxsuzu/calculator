use crate::ast::Statement::Expression;
use crate::ast::*;

#[derive(Debug)]
pub struct RPNCompiler {}

impl RPNCompiler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn compile(&self, program: Program) -> Option<String> {
        let mut byte_code = String::new();
        for s in program {
            let ex = match s {
                Expression(ex) => ex,
            };
            self.compile_expression(&ex, &mut byte_code);
            break;
        }
        Some(byte_code)
    }
    fn compile_expression(&self, ex: &Expr, byte_code: &mut String) {
        match ex {
            Expr::Literal(Literal::Int(i)) => byte_code.push_str(&i.to_string()),
            Expr::Prefix(p, x) => {
                self.compile_prefix(p, byte_code);
                self.compile_expression(x, byte_code);
            }

            Expr::Infix(i, x, y) => {
                self.compile_expression(x, byte_code);
                byte_code.push_str(" ");
                self.compile_expression(y, byte_code);
                byte_code.push_str(" ");
                self.compile_infix(i, byte_code);
            }
        };
    }

    fn compile_prefix(&self, p: &Prefix, byte_code: &mut String) {
        match *p {
            Prefix::Minus => byte_code.push_str("-"),
        }
    }

    fn compile_infix(&self, i: &Infix, byte_code: &mut String) {
        match *i {
            Infix::Plus => byte_code.push_str("+"),
            Infix::Minus => byte_code.push_str("-"),
            Infix::Multiply => byte_code.push_str("*"),
            Infix::Divide => byte_code.push_str("/"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::RPNCompiler;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn compile(input: &str) -> Option<String> {
        RPNCompiler::new().compile(Parser::new(Lexer::new(input)).parse().unwrap())
    }
    fn test_compile() {
        let tests = vec![
            (
                r#"1+1
            "#,
                "1 1 +".to_string(),
            ),
            (
                r#"1*2+3*4
            "#,
                "1 2 * 3 4 * +".to_string(),
            ),
            (
                r#"1234 + 1 * 0
            "#,
                "1234 1 0 * +".to_string(),
            ),
            (
                r#"-1234 / 5
            "#,
                "-1234 5 /".to_string(),
            ),
        ];

        for (input, expect) in tests {
            let got = compile(input).unwrap();
            assert_eq!(expect, got);
        }
    }
}

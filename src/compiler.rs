use crate::ast::Statement::Expression;
use crate::ast::*;

#[derive(Debug)]
pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn compile(&self, s: Statement) -> String {
        let mut byte_code = String::new();

        byte_code.push_str(".intel_syntax noprefix\n");
        byte_code.push_str(".global main\n");
        byte_code.push_str("main:\n");

        let ex = match s {
            Expression(ex) => ex,
        };

        self.compile_expression(&ex, &mut byte_code);

        byte_code.push_str("  pop rax\n");
        byte_code.push_str("  ret\n");
        byte_code
    }
    fn compile_expression(&self, ex: &Expr, byte_code: &mut String) {
        match ex {
            Expr::Literal(Literal::Int(i)) => {
                byte_code.push_str("  push ");
                byte_code.push_str(&i.to_string());
                byte_code.push_str("\n");
            }
            Expr::Infix(i, x, y) => {
                self.compile_expression(x, byte_code);
                self.compile_expression(y, byte_code);
                self.compile_infix(i, byte_code);
            }
            Expr::Prefix(_p, x) => {
                self.compile_expression(&Expr::Literal(Literal::Int(0)), byte_code);
                self.compile_expression(x, byte_code);
                self.compile_infix(&Infix::Minus, byte_code);
            }
        };
    }

    fn compile_infix(&self, i: &Infix, byte_code: &mut String) {
        byte_code.push_str("  pop rdi\n");
        byte_code.push_str("  pop rax\n");

        match *i {
            Infix::Plus => byte_code.push_str("  add rax, rdi\n"),
            Infix::Minus => byte_code.push_str("  sub rax, rdi\n"),
            Infix::Multiply => byte_code.push_str("  imul rax, rdi\n"),
            Infix::Divide => {
                byte_code.push_str("  cqo\n");
                byte_code.push_str("  idiv rdi\n");
            }
        }
        byte_code.push_str("  push rax\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Expr, Infix, Literal, Statement};
    use crate::compiler::Compiler;
    #[test]
    fn test_compile() {
        let header = r#".intel_syntax noprefix
.global main
main:"#;

        let tests = vec![
            (
                r#"
  push 1
  push 1
  pop rdi
  pop rax
  add rax, rdi
  push rax
  pop rax
  ret
"#,
                Statement::Expression(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ),
            (
                r#"
  push 1
  push 1
  pop rdi
  pop rax
  sub rax, rdi
  push rax
  pop rax
  ret
"#,
                Statement::Expression(Expr::Infix(
                    Infix::Minus,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ),
            (
                r#"
  push 1
  push 1
  pop rdi
  pop rax
  imul rax, rdi
  push rax
  pop rax
  ret
"#,
                Statement::Expression(Expr::Infix(
                    Infix::Multiply,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ),
            (
                r#"
  push 1
  push 1
  pop rdi
  pop rax
  cqo
  idiv rdi
  push rax
  pop rax
  ret
"#,
                Statement::Expression(Expr::Infix(
                    Infix::Divide,
                    Box::new(Expr::Literal(Literal::Int(1))),
                    Box::new(Expr::Literal(Literal::Int(1))),
                )),
            ),
        ];

        let c = Compiler::new();

        for (input, want) in tests {
            let got = c.compile(want);
            assert_eq!(format!("{}{}", header, input).to_string(), got);
        }
    }
}

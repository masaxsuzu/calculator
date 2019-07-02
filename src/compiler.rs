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
            Expr::Prefix(p, x) => {
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
    use crate::compiler::Compiler;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn test_compile() {
        let tests = vec![(
            r#"1
            "#,
            "1".to_string(),
        )];
    }
}

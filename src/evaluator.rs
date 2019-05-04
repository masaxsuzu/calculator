use crate::ast::*;
use crate::object::*;

#[derive(Debug)]
pub struct Evaluator {}

impl Evaluator {
    fn error(code: ErrorCode, msg: String) -> Object {
        Object::Error(code, msg)
    }

    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;

        for stmt in program {
            match self.eval_statement(stmt) {
                Some(Object::Error(code, msg)) => return Some(Object::Error(code, msg)),
                obj => result = obj,
            }
        }
        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Expression(expression) => self.eval_expression(expression),
        }
    }

    fn eval_expression(&mut self, expression: Expr) -> Option<Object> {
        match expression {
            Expr::Literal(literal) => Some(self.eval_literal(literal)),

            Expr::Prefix(prefix, right_expression) => match self.eval_expression(*right_expression)
            {
                Some(right) => Some(self.eval_prefix_expression(prefix, right)),
                _ => None,
            },

            Expr::Infix(infix, left_expression, right_expression) => match (
                self.eval_expression(*left_expression),
                self.eval_expression(*right_expression),
            ) {
                (Some(left), Some(right)) => Some(self.eval_infix_expression(infix, left, right)),
                _ => None,
            },
        }
    }

    fn eval_prefix_expression(&mut self, prefix: Prefix, right: Object) -> Object {
        match prefix {
            Prefix::Minus => self.eval_minus_prefix(right),
        }
    }

    fn eval_minus_prefix(&mut self, right: Object) -> Object {
        match right {
            Object::Int(value) => Object::Int(-value),
            _ => Self::error(ErrorCode::RuntimeError, format!("invalid operator '-'")),
        }
    }

    fn eval_infix_expression(&mut self, infix: Infix, left: Object, right: Object) -> Object {
        match (left.clone(), right.clone()) {
            (Object::Int(l), Object::Int(r)) => self.eval_infix_int_expr(infix, l, r),
            (Object::Int(_), _) => Self::error(
                ErrorCode::RuntimeError,
                format!("type mismatch '{}'", infix),
            ),
            _ => Self::error(
                ErrorCode::RuntimeError,
                format!("unknown operator '{}'", infix),
            ),
        }
    }

    fn eval_infix_int_expr(&mut self, infix: Infix, left: i64, right: i64) -> Object {
        match infix {
            Infix::Plus => Object::Int(left + right),
            Infix::Minus => Object::Int(left - right),
            Infix::Multiply => Object::Int(left * right),
            Infix::Divide => match right {
                0 => Self::error(ErrorCode::RuntimeError, format!("divide {} by 0", left)),
                _ => Object::Int(left / right),
            },
        }
    }

    fn eval_literal(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::Int(value) => Object::Int(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::*;
    use crate::lexer::Lexer;
    use crate::object::ErrorCode;
    use crate::object::Object;
    use crate::parser::Parser;

    fn eval(input: &str) -> Option<Object> {
        Evaluator::new().eval(Parser::new(Lexer::new(input)).parse())
    }

    #[test]
    fn test_literal() {
        let tests = vec![
            (r#"5\n"#, Some(Object::Int(5))),
            (r#"10\n"#, Some(Object::Int(10))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_prefix() {
        let tests = vec![(r#"-5\n"#, Some(Object::Int(-5)))];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_infix() {
        let tests = vec![
            (r#"1+1\n"#, Some(Object::Int(2))),
            (r#"2-3\n"#, Some(Object::Int(-1))),
            (r#"24*3\n"#, Some(Object::Int(72))),
            (r#"10/3\n"#, Some(Object::Int(3))),
            (
                r#"10/0\n"#,
                Some(Object::Error(
                    ErrorCode::RuntimeError,
                    String::from("divide 10 by 0"),
                )),
            ),
            (
                r#"1 + 10/0\n"#,
                Some(Object::Error(
                    ErrorCode::RuntimeError,
                    String::from("type mismatch \'+\'"),
                )),
            ),
            (
                r#"-(10/0)\n"#,
                Some(Object::Error(
                    ErrorCode::RuntimeError,
                    String::from("invalid operator \'-\'"),
                )),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_group() {
        let tests = vec![
            (r#"(1+1)\n"#, Some(Object::Int(2))),
            (r#"(2-3)*3\n"#, Some(Object::Int(-3))),
            (r#"(3*24)/3\n"#, Some(Object::Int(24))),
            (r#"(10/3)-1\n"#, Some(Object::Int(2))),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

}

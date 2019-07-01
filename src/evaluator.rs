use crate::ast::*;
use crate::error::RuntimeError;
use crate::object::*;

#[derive(Debug)]
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&mut self, program: Program) -> Result<Object, RuntimeError> {
        for stmt in program {
            return self.eval_statement(stmt);
        }
        Err(RuntimeError::FoundNoProgram)
    }
    fn eval_statement(&mut self, statement: Statement) -> Result<Object, RuntimeError> {
        match statement {
            Statement::Expression(expression) => self.eval_expression(expression),
        }
    }

    fn eval_expression(&mut self, expression: Expr) -> Result<Object, RuntimeError> {
        match expression {
            Expr::Literal(literal) => Ok(self.eval_literal(literal)),

            Expr::Prefix(prefix, right_expression) => match self.eval_expression(*right_expression)
            {
                Ok(right) => Ok(self.eval_prefix_expression(prefix, right)),
                Err(e) => Err(e),
            },

            Expr::Infix(infix, left_expression, right_expression) => match (
                self.eval_expression(*left_expression),
                self.eval_expression(*right_expression),
            ) {
                (Ok(left), Ok(right)) => self.eval_infix_expression(infix, left, right),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
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
        }
    }

    fn eval_infix_expression(
        &mut self,
        infix: Infix,
        left: Object,
        right: Object,
    ) -> Result<Object, RuntimeError> {
        match (left.clone(), right.clone()) {
            (Object::Int(l), Object::Int(r)) => self.eval_infix_int_expr(infix, l, r),
        }
    }

    fn eval_infix_int_expr(
        &mut self,
        infix: Infix,
        left: i64,
        right: i64,
    ) -> Result<Object, RuntimeError> {
        match infix {
            Infix::Plus => Ok(Object::Int(left + right)),
            Infix::Minus => Ok(Object::Int(left - right)),
            Infix::Multiply => Ok(Object::Int(left * right)),
            Infix::Divide => match right {
                0 => Err(RuntimeError::DivideByZero),
                _ => Ok(Object::Int(left / right)),
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
    use crate::error::RuntimeError;
    use crate::evaluator::*;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    fn eval(input: &str) -> Result<Object, RuntimeError> {
        Evaluator::new().eval(Parser::new(Lexer::new(input)).parse().unwrap())
    }

    #[test]
    fn test_literal() {
        let tests = vec![
            (
                r#"5
            "#,
                Ok(Object::Int(5)),
            ),
            (
                r#"10
            "#,
                Ok(Object::Int(10)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }
    #[test]
    fn test_prefix() {
        let tests = vec![(
            r#"-5
        "#,
            Ok(Object::Int(-5)),
        )];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }
    #[test]
    fn test_infix() {
        let tests = vec![
            (
                r#"1+1
            "#,
                Ok(Object::Int(2)),
            ),
            (
                r#"2-3
            "#,
                Ok(Object::Int(-1)),
            ),
            (
                r#"24*3
            "#,
                Ok(Object::Int(72)),
            ),
            (
                r#"10/3
            "#,
                Ok(Object::Int(3)),
            ),
            (
                r#"10/0
                "#,
                Err(RuntimeError::DivideByZero),
            ),
            (
                r#"1 + 10/0
                "#,
                Err(RuntimeError::DivideByZero),
            ),
            (
                r#"-(10/0)
                "#,
                Err(RuntimeError::DivideByZero),
            ),
            (
                r#"
                "#,
                Err(RuntimeError::FoundNoProgram),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

    #[test]
    fn test_group() {
        let tests = vec![
            (
                r#"(1+1)
            "#,
                Ok(Object::Int(2)),
            ),
            (
                r#"(2-3)*3
            "#,
                Ok(Object::Int(-3)),
            ),
            (
                r#"(3*24)/3
            "#,
                Ok(Object::Int(24)),
            ),
            (
                r#"(10/3)-1
            "#,
                Ok(Object::Int(2)),
            ),
        ];

        for (input, expect) in tests {
            assert_eq!(expect, eval(input));
        }
    }

}

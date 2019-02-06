use std::fmt;

pub type Program = BlockStatement;

pub type BlockStatement = Vec<Statement>;

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Expression(Expr),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Minus,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Prefix::Minus => write!(f, "-"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Divide => write!(f, "/"),
            Infix::Multiply => write!(f, "*"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Sum,     // + -
    Product, // * /
    Prefix,
}

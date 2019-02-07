use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Int(i64),
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Int(ref value) => write!(f, "{}", value),
            Object::Error(ref value) => write!(f, "{}", value),
        }
    }
}

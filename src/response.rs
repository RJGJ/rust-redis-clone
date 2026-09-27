use core::fmt;

#[derive(Debug)]
pub enum Response {
    Ok,
    Value(String),
    Integer(i64),
    Nil,
    Error(String),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "OK"),
            Response::Value(value) => write!(f, "{}", value),
            Response::Integer(value) => write!(f, "{}", value),
            Response::Nil => write!(f, "(nil)"),
            Response::Error(message) => write!(f, "ERR {}", message),
        }
    }
}

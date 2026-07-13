#[derive(Debug)]
pub enum Response {
    Ok,
    Value(String),
    Integer(i64),
    Nil,
    Error(String),
}

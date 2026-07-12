use crate::command::Command;

pub fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.as_slice() {
        ["GET", key] => Some(Command::Get { key: (*key).into() }),
        ["SET", key, value] => Some(Command::Set {
            key: (*key).into(),
            value: (*value).into(),
        }),
        ["DEL", key] => Some(Command::Del { key: (*key).into() }),
        _ => None,
    }
}

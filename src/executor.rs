use crate::{command::Command, response::Response, store::Store};

pub fn execute(store: &mut Store, command: Command) -> Response {
    match command {
        Command::Set { key, value } => {
            store.set(&key, &value);
            Response::Ok
        }
        Command::Get { key } => match store.get(&key) {
            Some(value) => Response::Value(value.clone()),
            None => Response::Nil,
        },
        Command::Del { key } => Response::Integer(if store.del(&key) { 1 } else { 0 }),
        Command::Ping => Response::Value("PONG".to_owned()),
    }
}

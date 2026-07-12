mod command;
mod parser;
mod store;

use command::Command;
use store::Store;

fn main() {
    println!("Hello, world!");

    let mut store = Store::new();

    let command = Command::Set {
        key: "name".into(),
        value: "RJ".into(),
    };

    let _ = command;
    let _ = store;
}

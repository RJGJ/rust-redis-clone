mod command;
mod executor;
mod parser;
mod response;
mod store;

use std::net::TcpListener;

use command::Command;
use executor::execute;
use store::Store;

use crate::parser::parse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").expect("Failed to bind port 6380");

    println!("Redis Clone is running on 127.0.0.1:6380");

    let mut store = Store::new();

    for stream in listener.incoming() {
        println!("Client connected!");

        let _stream = stream.expect("Connection failed");
    }
}

mod command;
mod executor;
mod parser;
mod response;
mod store;

use std::io::{Read, Write};
use std::net::TcpListener;

use executor::execute;
use store::Store;

use crate::parser::parse;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").expect("Failed to bind port 6380");

    println!("Redis Clone is running on 127.0.0.1:6380");

    let mut store = Store::new();

    for stream in listener.incoming() {
        println!("Client connected!");

        let mut stream = stream.expect("Connection failed");

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).expect("Read failed");
        let input = String::from_utf8_lossy(&buffer[..bytes_read]);
        let input = input.trim();

        if let Some(command) = parse(input) {
            let response = execute(&mut store, command);

            let output = format!("{}\n\n", response);

            stream.write_all(output.as_bytes()).expect("Write failed");
        } else {
            println!("Invalid command");
        }
    }
}

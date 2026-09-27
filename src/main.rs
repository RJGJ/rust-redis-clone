mod command;
mod executor;
mod parser;
mod response;
mod store;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

use executor::execute;
use store::Store;

use crate::parser::parse;

fn main() {
    let mut store = Store::new();

    println!("Redis Clone is running on 127.0.0.1:6380");

    let listener = TcpListener::bind("127.0.0.1:6380").expect("Failed to bind port 6380");
    for stream in listener.incoming() {
        let mut stream = stream.expect("Connection failed");
        println!("Client connected!");

        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        loop {
            line.clear();

            let bytes_read = reader.read_line(&mut line).expect("Read failed!");

            // client disconnected
            if bytes_read == 0 {
                break;
            }

            let input = line.trim_end_matches(&['\r', '\n'][..]);

            println!("Received: {}", input);

            if let Some(command) = parse(input) {
                let response = execute(&mut store, command);

                let output = format!("{}\r\n", response);

                reader
                    .get_mut()
                    .write_all(output.as_bytes())
                    .expect("Write failed!");
            } else {
                reader
                    .get_mut()
                    .write_all(b"ERR Invalid command\r\n")
                    .expect("Write failed!");
            }

            println!("Client disconnected");
        }
    }
}

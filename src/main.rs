mod command;
mod executor;
mod parser;
mod response;
mod store;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use executor::execute;
use store::Store;

use crate::parser::parse;

fn handle_client(stream: TcpStream, store: Arc<Mutex<Store>>) {
    print!("Client connected");

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line).expect("Read failed");

        if bytes_read == 0 {
            break;
        }

        let input = line.trim_end_matches(&['\r', '\n'][..]);

        println!("Received: {}", input);

        if let Some(command) = parse(input) {
            let response = {
                let mut store = store.lock().expect("Store lock poisoned");

                execute(&mut store, command)
            };

            let output = format!("{}\r\n", response);

            reader
                .get_mut()
                .write_all(output.as_bytes())
                .expect("Write failed");
        } else {
            reader
                .get_mut()
                .write_all(b"ERR Invalid command\r\n")
                .expect("Write failed");
        }
    }

    println!("Client disconnected");
}

fn main() {
    let mut store = Arc::new(Mutex::new(Store::new()));

    println!("Redis Clone is running on 127.0.0.1:6380");

    let listener = TcpListener::bind("127.0.0.1:6380").expect("Failed to bind port 6380");
    for stream in listener.incoming() {
        let stream = stream.expect("Connection failed");

        let store = Arc::clone(&store);

        thread::spawn(move || {
            handle_client(stream, store);
        });
    }
}

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("handling incoming connection");

    let respone = "+PONG\r\n";

    loop {
        match stream.read(&mut [0; 128]) {
            Ok(0) => {
                println!("No more bytes to read");
                break;
            }
            Ok(num) => {
                println!("Read {} bytes", num);
                stream.write(respone.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

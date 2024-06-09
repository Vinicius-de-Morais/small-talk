use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

use small_talk::protocol;
use small_talk::protocol::Protocol;
use small_talk::ThreadPool;
use small_talk::conn;

fn main() {
    
    // escutando a porta 6969
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPool::new(4);


    // fazendo um laço a partir da stream de dados vinda do listener
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = Protocol::handle_request(&buffer);

    stream.write_all(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}
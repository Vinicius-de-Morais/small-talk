use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use small_talk::ThreadPool;
use small_talk::conn;
mod components;



fn main() {

    let handle_up_server = thread::spawn(|| {    
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
    });

    let handle_ui = thread::spawn(|| {    
        components::init_ui();
    });
    
    let connection = &mut conn::establish_connection();

    handle_up_server.join().unwrap();
    handle_ui.join().unwrap();

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = response();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn response() -> String{

    let mut html: String = "".to_owned();
    
    html.push_str(&"<!DOCTYPE html>");

    html.push_str(&"<html lang='en'>");
    html.push_str(&"<head>");
    html.push_str(&"    <meta charset='utf-8'>");
    html.push_str(&"    <title>Hello!</title>");
    html.push_str(&"</head>");
    html.push_str(&"<body>");
    html.push_str(&"    <h1>Hello!</h1>");
    html.push_str(&"    <p>Hi from Rust</p>");
    html.push_str(&"</body>");
    html.push_str(&"</html>");

    html
}
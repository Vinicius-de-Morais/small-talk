use std::{
    io::{prelude::*, BufReader, Result},
    net::{TcpListener, TcpStream}, thread,
};

use small_talk::ThreadPool;

fn main() {
    
    // escutando a porta 6969
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPool::new(4);


    // fazendo um laço a partir da stream de dados vinda do listener
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });
    }

}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let content_line = "Content-Type: text/html; charset=utf-8\r\n\r\n";
    
    let content = response();
    let length = content.len();


    let res = format!("{status_line}\r\n{content_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    //stream.write_all()
    stream.write_all(res.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request);
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
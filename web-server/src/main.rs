use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const RESPONSE: &'static str = concat!(
    "HTTP/1.1 200 OK",
    "\r\n",
    "\r\n",
    include_str!("../static/index.html"),
);

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(x) => handle_conn(x),
            Err(e) => eprintln!("err: {}", e),
        };
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write(RESPONSE.as_bytes()).unwrap();
    stream.flush().unwrap();
}

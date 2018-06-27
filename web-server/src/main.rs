use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const OK: &'static str = concat!(
    "HTTP/1.1 200 OK",
    "\r\n",
    "\r\n",
    include_str!("../static/index.html"),
);

const NOT_FOUND: &'static str = concat!(
    "HTTP/1.1 404 Not Found",
    "\r\n",
    "\r\n",
    include_str!("../static/404.html"),
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
    // println!("request: {}", String::from_utf8_lossy(&buffer[..]));
    let resp = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        OK
    } else {
        NOT_FOUND
    };
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

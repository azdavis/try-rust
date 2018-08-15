use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const OK: &'static str = concat!(
    "HTTP/1.1 200 OK",
    "\r\n",
    "\r\n",
    include_str!("../static/index.html"),
);

const STYLE: &'static str = concat!(
    "HTTP/1.1 200 OK",
    "\r\n",
    "\r\n",
    include_str!("../static/style.css"),
);

const NOT_FOUND: &'static str = concat!(
    "HTTP/1.1 404 Not Found",
    "\r\n",
    "\r\n",
    include_str!("../static/404.html"),
);

/// Handle a connection stream. Just respond with some simple HTML pages.
/// Panic if couldn't read or write from the stream.
pub fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let resp = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        OK
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        OK
    } else if buffer.starts_with(b"GET /style.css HTTP/1.1\r\n") {
        STYLE
    } else {
        NOT_FOUND
    };
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}

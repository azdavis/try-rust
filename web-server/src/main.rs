use std::net::TcpListener;

mod conn;

const HOST: &'static str = "localhost:7878";

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(x) => conn::handle(x),
            Err(e) => eprintln!("err: {}", e),
        };
    }
}

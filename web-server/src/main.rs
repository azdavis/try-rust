use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(x) => println!("ok: {:?}", x),
            Err(e) => eprintln!("err: {}", e),
        };
    }
}

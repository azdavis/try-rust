use std::net::TcpListener;

mod conn;

const HOST: &'static str = "localhost:7878";

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();
    let pool = ThreadPool::new(5);
    for stream in listener.incoming() {
        match stream {
            Ok(x) => pool.execute(|| conn::handle(x)),
            Err(e) => eprintln!("err: {}", e),
        };
    }
}

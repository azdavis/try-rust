use std::net::TcpListener;

mod conn;
mod thread_pool;

use thread_pool::ThreadPool;

const HOST: &'static str = "localhost:7878";

fn main() {
  let listener = TcpListener::bind(HOST).unwrap();
  let pool = ThreadPool::new(5);
  println!("Serving on http://{}", HOST);
  for stream in listener.incoming() {
    match stream {
      Ok(x) => pool.execute(|| conn::handle(x)),
      Err(e) => eprintln!("err: {}", e),
    };
  }
}

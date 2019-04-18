extern crate communicator;

fn main() {
  communicator::client::connect();
  communicator::network::connect();
  communicator::network::server::connect();
}

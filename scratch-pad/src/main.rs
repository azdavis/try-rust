mod empty_let;
mod lazy;
mod partial_move;
mod rect;

fn main() {
    println!("Hello, world!");
    rect::run();
    partial_move::run();
    empty_let::run();
    lazy::run();
}

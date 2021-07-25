#![allow(unused)]

mod eat;
mod empty_let;
mod from_fn;
mod if_let;
mod iter;
mod lazy;
mod multi_mut_borrow;
mod mut_ref;
mod partial_move;
mod rc_ref_cell;
mod rect;
mod slice;
mod trait_obj;

fn main() {
  println!("hello from the sandbox!");
  rc_ref_cell::run();
}

#[no_mangle]
pub extern "C" fn inc(x: usize) -> usize {
  println!("hello from rust inc({})", x);
  x + 1
}

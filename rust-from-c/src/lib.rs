#[no_mangle]
pub extern "C" fn inc(x: i32) -> i32 {
  println!("hello from rust inc({})", x);
  x + 1
}

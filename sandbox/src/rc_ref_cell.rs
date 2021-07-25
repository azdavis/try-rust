use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
  let one = Rc::new(RefCell::new(String::new()));
  let two = one.clone();
  println!("mutating one");
  mutate(one);
  println!("got {} from two", two.borrow());
}

fn mutate(c: Rc<RefCell<String>>) {
  c.borrow_mut().push_str("hi");
}

struct Lazy<T, F>
where
  F: Fn() -> T,
{
  calc: F,
  val: Option<T>,
}

impl<T, F> Lazy<T, F>
where
  F: Fn() -> T,
{
  fn new(calc: F) -> Self {
    Lazy { calc, val: None }
  }

  fn get(&mut self) -> &T {
    match self.val {
      Some(ref x) => x,
      None => self.val.get_or_insert_with(&self.calc),
    }
  }
}

pub fn run() {
  let mut thing = Lazy::new(|| {
    println!("called");
    3 * 5
  });
  println!("begin");
  // does print 'called'
  println!("fst = {}", thing.get());
  // does not print 'called'
  println!("snd = {}", thing.get());
}

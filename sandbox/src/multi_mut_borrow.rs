struct S;

impl S {
  fn foo(&mut self) {}
  fn bar(&mut self) {}
  fn quz(&self) {}
}

pub fn run() {
  //  vvv mut is required
  let mut x = S;
  // note many mutable borrows in the scope of run, but rust is fine with it.
  // is this the fabled NLL? I think it is.
  x.foo();
  x.bar();
  x.quz();
  x.foo();
  x.bar();
  x.quz();
}

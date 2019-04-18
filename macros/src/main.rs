fn main() {
  println!("Hello, world!");
  let a = Settings::new();
  println!("a = {:?}", a);
  let b = a.foo(true).heh(5).lel(vec!["fella".to_string()]);
  println!("b = {:?}", b);
}

#[derive(Debug)]
struct Settings {
  foo: bool,
  heh: i32,
  lel: Vec<String>,
}

impl Settings {
  fn new() -> Self {
    Settings {
      foo: false,
      heh: 0,
      lel: vec![],
    }
  }
}

macro_rules! builder_pattern_impl {
  ( $f:ident: $t:ty  ) => {
    impl Settings {
      fn $f(self, x: $t) -> Self {
        Settings { $f: x, ..self }
      }
    }
  };
}

macro_rules! builder_pattern_impl_all {
    ( $( $f:ident: $t:ty, )* ) => {
        $(
            builder_pattern_impl!($f: $t);
        )*
    };
}

builder_pattern_impl_all! {
    foo: bool,
    heh: i32,
    lel: Vec<String>,
}

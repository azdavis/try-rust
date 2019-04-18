pub fn run() {
  let mut s = "s".to_string();
  let rs = hmm(&mut s);
  heh(rs);
}

fn hmm(x: &mut String) -> &String {
  x.push_str("hmm");
  x
}

fn heh(x: &String) {
  println!("{}", x);
  // x.push_str("heh")
}

pub fn run() {
  /*
   *                             this needs to implement Copy for map to work
   *                             vvvvvvvvvvvvvvvvvvv
  let a: Result<String, ()> = Ok(String::from("heh"));
   * because of the partial move here
   * vvvvvvvvvvvvv
  if let Ok(b) = a {
      a.map(|x| println!("yay {}", x));
      println!("heh {}", b);
  }
   */
}

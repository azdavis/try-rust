pub fn run() {
    let a = [1,2,3,4];
    for x in a.iter().take(100) {
        println!("{}", x);
    }
}

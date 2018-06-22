struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_contain(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    println!("Hello, world!");
    let a = Rect { width: 3, height: 5 };
    let b = Rect { width: 2, height: 9 };
    let c = Rect { width: 2, height: 5 };
    println!("area of a: {}", a.area());
    println!("a can contain b: {}", a.can_contain(&b));
    println!("a can contain c: {}", a.can_contain(&c));
    println!("b can contain c: {}", b.can_contain(&c));
}

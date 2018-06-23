#[derive(Debug)]
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

    fn hue(&mut self) {
        self.width += 1;
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

    let mut guy = Rect { width: 3, height: 5 };
    println!("guy = {:?}", guy);
    guy.hue();
    println!("guy (after hue) = {:?}", guy);

    let mut v = vec![
        String::from("fella"),
    ];
    v.push(String::from("dude"));
    {
        let ref mut x = v[0];
        x.push('s');
    }
    let x = &v[0];
    println!("v = {:?}, x = {:?}", v, x);

    let guy = {
        let mut guy = String::new();
        guy.push_str("fella");
        guy.push_str("guy");
        guy
    };
    println!("guy = {}", guy);

    /*
     *                             this needs to implement Copy for this to work
     *                             vvvvvvvvvvvvvvvvvvv
    let a: Result<String, ()> = Ok(String::from("heh"));
     * because of the partial move here
     * vvvvvvvvvvvvv
    if let Ok(b) = a {
        a.map(|x| println!("yay {}", x));
        println!("heh {}", b);
    }
     */

    let hue;
    if 3 == 2 {
        hue = 4;
    } else {
        println!("oh fug");
        hue = 5;
    }
    println!("{}", hue);

    let mut x = String::from("guy");
    f(&mut x);
    println!("{}", x);
}

fn f(x: &mut String) {
    x.push_str("fella");
}

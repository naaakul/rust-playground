struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    };
    println!("The area of the rect is {}", rect1.area());
}

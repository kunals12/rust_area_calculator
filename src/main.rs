#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 3,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let square = Rectangle::square(10);

    println!("The area of rectangle is {} square pixels", rect1.area());
    println!("{}", rect1.width());
    println!("square {:?}", square.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect1));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect2));
}
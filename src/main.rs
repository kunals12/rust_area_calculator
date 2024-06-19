struct Rectangle {
    height: u32,
    width: u32,
}

struct Square {
    side: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width:50,
    };

    let square1 = Square {
        side: 5,
    };

    // Rectangle
    // println!("rect1 is {:#?}", rect1);
    println!("The area of the reactangle is {} square pixel", area(&rect1));

    // Square
    println!("The area of Square is {}", area_of_square(square1))
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn area_of_square(square: Square) -> u32 {
    square.side * 4
}
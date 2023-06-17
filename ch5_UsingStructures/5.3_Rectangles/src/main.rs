/// 1st version
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!(
//         "The area of the rectangle is {} sq.units.",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

/// 2nd version
// fn main() {
//     let rect = (30, 50);
//     println!(
//         "The area of the rectangle is {} sq.units.",
//         area(rect)
//     );
// }

// fn area(rect: (u32, u32)) -> u32 {
//     return rect.0 * rect.1;
// }

///3rd version
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return (self.width >= rect.width) && (self.height >= rect.height);
    }

    fn square(size: u32) -> Self {
        return Self{height: size, width: size};
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square1 = Rectangle::square(15);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square1? {}", rect1.can_hold(&square1));
    println!("Here is square1: {square1:?}");

    println!(
        "The area of the rectangle is {} sq.units.",
        rect1.area()
    );
    println!("Your rectangle is {rect1:?}")
}

// fn area(rect: &Rectangle) -> u32 {
//     return rect.width * rect.height;
// }
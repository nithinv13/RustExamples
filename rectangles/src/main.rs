// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!("The area of th rectangle is {}", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect1 = (30, 50);
//     println!("The area of the rectangle is {}", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("The area of the rectangle is {}", area(&rect1));
//     println!("The rectangle structure is: {:?}", rect1);
// }

// fn area(rectangle: &Rectangle)  -> u32 {
//     rectangle.width * rectangle.height
// }

// Defining area as a method on Rectangle

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions are those without self, usually used for initialization
    // They are reference using struct_name::function_name
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size, 
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}", rect1.area());
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rect3 is {} square pixels", rect3.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // All functions defined within an impl block are called associated functions because
    // they’re associated with the type named after the impl. We can define associated
    // functions that don’t have self as their first parameter (and thus are not methods)
    // because they don’t need an instance of the type to work with.

    // Associated function (not a method)
    // The Self keywords in the return type and in the body of the function are aliases
    // for the type that appears after the impl keyword, which in this case is Rectangle.
    //
    // To call this associated function, we use the :: syntax with the struct name
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Method
    // self is a reference to the instance of the struct that the method is called on.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

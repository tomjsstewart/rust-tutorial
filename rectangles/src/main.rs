#[derive(Debug)] // Derive the trait debug
struct Rectangle {
    width: u32,
    height: u32,
}

// Method, defined on a struct
// These are called associated functions
impl Rectangle {
    fn area(&self) -> u32 {
        // self is a reference to the struct the method is defined on
        // &self if shorthand for self: &Self where self is an alias for the current type
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }
}

// Can have more than one impl block
impl Rectangle {
    fn perimeter(&self) -> u32 {
        (self.height * 2) + (self.width * 2)
    }
}

// This is an associated function but not a method
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels (naive).",
        area_naive(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels (tuple).",
        area_tuple((width1, height1))
    );

    let rect = Rectangle {
        width: width1,
        height: height1,
    };
    println!(
        "The area of the rectangle is {} square pixels (struct).",
        area_struct(&rect) // Reference to rect so ownership is not lost by main
    );
    println!("rect is {:?}", rect); // The Debug trait provides an output format for the struct. Can use {:#?} for pretty-print

    // dbg! macro can be used to output to stderr - it prints file and line number
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // dbg! macro takes ownership of value passed in so pass reference to rect2
    dbg!(&rect2);

    println!(
        "The area of the rectangle is {} square pixels (method).",
        rect.area() // Call method on Rectangle
    );

    println!(
        "The perimeter of the rectangle is {} pixels (method).",
        rect.perimeter() // Call method on Rectangle
    );

    // Methods can be named the same as fields, Rust can differentiate
    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    // Methods with parameters
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq1 = Rectangle::square(10);
    println!("I am a square {:?}", sq1);

}

fn area_naive(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

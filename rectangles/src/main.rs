#[derive(Debug)] // Derive the trait debug
struct Rectangle {
    width: u32,
    height: u32,
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

    let rect = Rectangle{width: width1, height: height1};
    println!(
        "The area of the rectangle is {} square pixels (tuple).",
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

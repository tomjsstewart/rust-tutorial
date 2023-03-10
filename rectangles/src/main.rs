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

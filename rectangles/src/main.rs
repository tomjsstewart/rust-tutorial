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
}

fn area_naive(width: u32, height:u32) -> u32 {
    width*height
}

fn area_tuple(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_naive(width1, height1)
    );
}

fn area_naive(width: u32, height:u32) -> u32 {
    width*height
}
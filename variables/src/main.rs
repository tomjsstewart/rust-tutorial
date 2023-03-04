fn main() {
    // mut must be used to make x mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");

    // constants use const and require a type annotation, expression is evaluated at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant: {THREE_HOURS_IN_SECONDS}");

}

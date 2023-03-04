fn main() {
    // mut must be used to make x mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is {x}");

    // constants use const and require a type annotation, expression is evaluated at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant: {THREE_HOURS_IN_SECONDS}");

    // shadowing, inside the inner scope the varible x is overshadowed and returns to the previous value outside the scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}");

    // by using let we can shadow the spaces variable with 2 different types
    let spaces = "   ";
    let spaces = spaces.len();
}

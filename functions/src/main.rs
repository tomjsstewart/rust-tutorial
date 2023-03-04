fn main() {
    println!("Hello, world!");

    another_function(5, 'M');

    an_expression();

    println!("value in func five: {}", five());
}

fn another_function(x: i32, h: char) {
    println!("The value of x is {x}, h={h}");
}

fn an_expression() {
    let y = {
        let x = 3;
        x + 1 // no ; at the end, this expression is returned by the scope
    };

    println!("The value of y is: {y}");
}
fn five() -> u32 {
    // type added to function
    5 // functions return the last expression explicitly (no semi-colon)
      // return 5; // can use statement with return keyword too
}

fn plus_one(inp: i32) -> i32 {
    inp + 1
}

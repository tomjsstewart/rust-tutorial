fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number {} is not valid as rust will not cast to boolean automatically

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if is an expression, we can use it on the right side of a let statement
    let y = if false {6} else {-1};


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break keyword can return an expression
            break counter * 2;  
        }
    };

    // result = 20
    println!("The result is {result}");
    // This construct can be used for requesting user input until it is valid


}

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

    loop_label();

    while_loop();

    for_loop();
}


fn loop_label() {
    let mut count = 0;
    'counting_up: loop {  // counting_up is a loop label and beings with a '
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  // break innermost loop
            }
            if count == 2 {
                break 'counting_up;  // break counting_up loop even if in inner loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    // Output:
    // count = 0
    // remaining = 10
    // remaining = 9
    // count = 1
    // remaining = 10
    // remaining = 9
    // count = 2
    // remaining = 10
    // End count = 2
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {  // use in to iterate an array
        println!("The value is {element}");
    }
}

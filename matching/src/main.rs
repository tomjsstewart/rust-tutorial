enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("{num_spaces}");
}

fn main() {
    let random_roll = 77; // A very random number
    match random_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // Other is a variable and can be named anything we want
                                     // Can use _ => ... To match all other values and throw them away
    };

    if_let();
}

fn if_let() {
    let config_max = Some(3u8);
    // if let provides a shorthand to match a single value with an optional else statement to handle any other values
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum was not provided")
    }

    // This is equivalent to:
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
}

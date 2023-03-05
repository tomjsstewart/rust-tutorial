fn main() {
    println!("400f in c = {}", fahrenheit_to_celsius(400.0));
    println!("25c in f = {}", celsius_to_fahrenheit(25.0));

    for i in 0..=10 {
        println!("{i}th Fibonacci number is {}", fib(i));
    }

    for i in 0..=10 {
        println!(
            "{i}th Fibonacci number is {} (using recursion)",
            fib_recur(i)
        );
    }

    println!("Lyrics to the 12 days of christmas are:");
    days_of_christmas();
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn fib(n: u32) -> u64 {
    // nth Fibonacci number naive
    let mut first: u64 = 0;
    let mut second: u64 = 1;

    if n == 0 {
        return first;
    }

    for i in (1..n) {
        let new = first + second;
        first = second;
        second = new;
    }
    return second;
}

fn fib_recur(n: u32) -> u64 {
    // nth Fibonacci number using recursion
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib_recur(n - 1) + fib_recur(n - 2);
}

fn days_of_christmas() {
    for day in 1..=12 {
        // logic for correct number here is too much effort
        println!("On the {day}th day of Christmas, my true love sent to me");

        if day >= 12 {
            println!("Twelve drummers drumming");
        }
        if day >= 11 {
            println!("Eleven pipers piping");
        }
        if day >= 10 {
            println!("Ten lords a-leaping");
        }
        if day >= 9 {
            println!("Nine ladies dancing");
        }
        if day >= 8 {
            println!("Eight maids a-milking");
        }
        if day >= 7 {
            println!("Seven swans a-swimming");
        }
        if day >= 6 {
            println!("Six geese a-laying");
        }
        if day >= 5 {
            println!("Five golden rings");
        }
        if day >= 4 {
            println!("Four calling birds");
        }
        if day >= 3 {
            println!("Three french hens");
        }
        if day >= 2 {
            println!("Two turtle doves, and");
        }
        if day >= 1 {
            println!("A partridge in a pear tree");
        }
        println!("");
    }
}

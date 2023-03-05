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

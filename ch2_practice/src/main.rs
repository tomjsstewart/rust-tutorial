fn main() {
    println!("400f in c = {}", fahrenheit_to_celsius(400.0));
    println!("25c in f = {}", celsius_to_fahrenheit(25.0));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

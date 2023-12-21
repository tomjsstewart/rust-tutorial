use std::fs::File;

fn main() {
    // crash_and_burn();
    // panic_again();
}


fn crash_and_burn() {
    // Only call to panic code!
    panic!("crash and burn");
}

fn panic_again() {
    let v = vec![1,2,3];

    v[99];  // Illegal access
}
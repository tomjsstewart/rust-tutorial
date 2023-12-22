use core::panic;
use std::io::ErrorKind;
use std::{error, fs::File};

fn main() {
    // crash_and_burn();
    // panic_again();

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_kind => {
                panic!("Problem opening the file {:?}", other_kind)
            }
        },
    };
}

fn crash_and_burn() {
    // Only call to panic code!
    panic!("crash and burn");
}

fn panic_again() {
    let v = vec![1, 2, 3];

    v[99]; // Illegal access
}

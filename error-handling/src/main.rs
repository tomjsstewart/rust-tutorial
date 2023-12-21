use core::panic;
use std::{fs::File, error};

fn main() {
    // crash_and_burn();
    // panic_again();

    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


fn crash_and_burn() {
    // Only call to panic code!
    panic!("crash and burn");
}

fn panic_again() {
    let v = vec![1,2,3];

    v[99];  // Illegal access
}
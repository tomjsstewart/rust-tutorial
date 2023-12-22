use core::panic;
use std::error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

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

    match read_username_from_file() {
        Ok(username) => print!("Read username {username}"),
        Err(e) => panic!("Error reading username from file {:?}", e),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Create a string to write to
    let mut username = String::new();

    // Use ? to propagate the errors, either return ok result to variable or return error from function
    let mut username_file = File::open("hello.txt")?; // Open the file
    username_file.read_to_string(&mut username)?; // Read into string

    // This can be chained equivalently:
    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Both options return Ok(username)
    Ok(username)
}

fn crash_and_burn() {
    // Only call to panic code!
    panic!("crash and burn");
}

fn panic_again() {
    let v = vec![1, 2, 3];

    v[99]; // Illegal access
}

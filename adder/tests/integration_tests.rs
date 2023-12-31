use adder::{self, add_two};

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // Run the common setup
    assert_eq!(add_two(3), 5);
}

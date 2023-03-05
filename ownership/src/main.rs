fn main() {
    // Instances of the String Type can be mutated (but string literals cannot)
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // borrow_error();

    cloning();

    function_ownership();
}

// fn scope(){            // s is not valid here, it’s not yet declared
//     let s = "hello";   // s is valid from this point forward
//     // do stuff with s
// }                      // this scope is now over, and s is no longer valid

//This function induces error[E0382], as move invalidates s1 so that only one variable points to String data on the heap
// fn borrow_error() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // s1 has been invalidated here and cannot be used.
//     println!("{}, world!", s1);
// }
// A side effect of this is that automatic copying is always cheap and heap memory is never copied

fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // This creates deep copy of s1 that copies the heap data and does not invalidate s1

    println!("s1 = {}, s2 = {}", s1, s2);

    // variables that have a fixed size are stored on the stack so copying and cloning are synonymous as no heap data is allocated
    // clone is thus not necessary
    // let x = 5;
    // let y = x;
    // x is still valid here as it is only assigned on the stack
}

fn function_ownership() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("x is still in scope here {x}");

    // Cannot do this as s's owner is `takes_ownership` and that scope has ended
    // so s has been dropped
    // println!("I cannot print s: {s}");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("I now own some_string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("I have a copy of some_integer: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

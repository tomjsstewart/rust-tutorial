fn main() {
    // Instances of the String Type can be mutated (but string literals cannot)
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // borrow_error();

    cloning();

    function_ownership();

    return_values();

    references();

    mutable_reference();
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

fn return_values() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn references() {
    let s1 = String::from("hello");

    // Pass a reference to calculate_length so that it is not dropped
    // This is called borrowing
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len)
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    return s.len();
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


// Immutable borrowing
//   fn immutable_borrowing() {
//     let s = String::from("hello");

//     change(&s);
// }

// Borrowing is immutable and we cannot modify something we have a reference to
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


// Mutable references
fn mutable_reference(){
    let mut s1 = String::from("hello");

    // If there is a mutable reference to s1, no other references to s1, mutable or otherwise, can exist
    // Multiple immutable references is fine
    change(&mut s1);

    println!("s1 is now: {s1}");

    mutable_after_immutable();
}

fn change(s: &mut String){
    s.push_str(" world!");
}

fn mutable_after_immutable() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
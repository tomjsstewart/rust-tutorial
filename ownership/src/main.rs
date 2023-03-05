fn main() {
    // Instances of the String Type can be mutated (but string literals cannot)
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // borrow_error();

    cloning();
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

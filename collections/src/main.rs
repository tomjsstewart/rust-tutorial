fn main() {
    vector();

    string();
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector() {
    // vector with type provided
    let v1: Vec<i32> = Vec::new();

    // vector macro (type infered as i32)
    let v2 = vec![1, 2, 3];

    // Updating vector (mutable)
    let mut v3: Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];
    // Third element
    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third2: Option<&i32> = v4.get(2);
    match third2 {
        Some(third2) => println!("The third element is {third2}"),
        None => println!("There is no third element"),
    }

    // This code panics as 100 is out of bounds
    // let does_not_exist = &v4[100];

    // None
    let does_not_exist = v4.get(100);

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        println!("{i}");
        *i *= 50;
        println!("{i}");
    }

    // We can use enum variants to wrap different elements to store different types in a vector
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}

fn string() {
    // from and to_string do the same things
    let s1 = String::from("Hello world");
    let s2 = "Hello world".to_string();

    // growing a string
    let mut s3 = String::from("foo");
    s3.push_str("bar"); // foobar
    println!("s3 is {s3}");

    // Adding strings
    let _s = String::from("hello ");
    let __s = String::from("world");
    let s4 = _s + &__s; // _s has been moved and cannot be used any more
    println!("Adding gives {s4}");

    // Using !format
    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");

    let s8 = format!("{s5}-{s6}-{s7}");
    println!("!format gives {s8}");

    // Accessing elemts of String
    // cannot use s[0]
    // use s[1..3] with great care

    println!("chars:");
    for c in "Зд".chars() {
        println!("{c}");
    }

    println!("bytes:");
    for c in "Зд".bytes() {
        println!("{c}"); // each utf-8 character is 2 bytes
    }
}

fn main() {
    vector();
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

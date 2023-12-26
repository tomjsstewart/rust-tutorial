mod aggregator;
use crate::aggregator::{Summary, Tweet}; // Avoid needing namespace in calls

// largest_i32 and largest_char are specific to their data types

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     // Type of T must be restricted to types that can be ordered
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
struct Point<T> {
    x: T,
    y: T,
}

// Point where x and y have different generic types
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    // method specifically for f32 Points
    fn distance_from_origin(&self) -> f32 {
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

// fn notify(item: &impl Summary) {  // This is syntactic sugar for trait bound syntax
fn notify<T: Summary>(item: &T) {
    // Can require multiple traits with <T: Summary + Display>
    // Function taking any type that implements the Summary Trait
    println!("Breaking news! {}", item.Summarise());
}

// To many trait bounds can get complex
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// Use where syntax:
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where
//         T: Display + Clone,
//         U: Clone + Debug,
//     {

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // Generic Struct
    let float_point: Point<f32> = Point { x: 5.2, y: 7.9 };
    let int_point = Point { x: 8, y: 6 };

    // let wont_work = Point { x: 5, y: 4.0 };  // This will not compile as x and y are not the same types

    println!("int_point.x = {}", int_point.x());

    // Traits
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.Summarise());

    notify(&tweet);
}

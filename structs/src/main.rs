// Define a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs - do not need to specify field names but still requires field types
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // Create an instance of the User struct
    let mut user1 = User {
        // use `let mut` to make the struct mutable
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); // use dot notation to access

    build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    // New instance of struct - bad!
    // let user2 = User {
    //       // Order of the fields when instanciating does not matter
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),  // Only this field has changed
    //     sign_in_count: user1.sign_in_count,
    // };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // fields not explicitly set should come from specified instance
                // Fields of user1 that do not use `Copy` trait are not not valid (username in this case)
    };

    // Tuple structs
    let black = Colour(0, 0, 0); // Access fields using number dot syntax
    let origin = Point(0, 0, 0); // can destructure as with tuples
                                 // Although both tuple structs have the same field types functions which accept one will not accept the other
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // Since username and email match the parameter name we can use field init shorthand syntax to reduce repetition
        email,
        sign_in_count: 1,
    }
}

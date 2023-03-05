fn main() {
    // floats
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    //chars use single quotes, strings use double quotes

    // Compound type - tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructure tup into 3 variables

    // access tuple indexes with .-notation
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let arr = [1, 2, 3, 4, 5];
    let arr_3s = [3; 5]; // array of 5 3's

    // Access values with index notation
    let first = arr[0];
    let second = arr[1];
    // Accessing an out-of-range index causes a panic
}

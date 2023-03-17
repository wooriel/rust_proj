use std::string::String;

fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);

    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // array
    let _months = ["January", "Febuary", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"];
    let months: [&str; 12] = ["January", "Febuary", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"];

    println!("The first month of the year is: {}", months[0]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("a[0]: {}", a[2]);
    println!("b[1]: {}", b[1]);
}
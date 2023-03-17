fn main() {
    // let reference_to_nothing = dangle();

    let dir_string = no_dangle();

    println!("{dir_string}");
}

// fn dangle() -> &String { // returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // return the reference to string
// } // s goes out of scope and is dropped

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
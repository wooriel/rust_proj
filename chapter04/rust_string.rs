fn main() {
    // let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world"); // push_str() appends literal to a string
    println!("{}", s);
}
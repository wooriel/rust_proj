fn main() {
    let s = String::from("hello");

    let i = 4;

    takes_ownership(s);

    // println!("{s}"); the original s in invalidated = compile error

    makes_copy(i);
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn makes_copy(i: i32) {
    println!("{i}");
}
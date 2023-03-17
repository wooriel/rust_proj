fn main() {
    let s1 = String::from("hello");

    let s1_length = calculate_length(&s1);

    println!("The length of {s1} is: {s1_length}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope (the reference is dropped),
  // but the value pointed by the reference (s1) is not dropped.
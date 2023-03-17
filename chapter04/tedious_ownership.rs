fn main() {
    let s1 = String::from("frustration");

    let (s1_length, s1) = calculate_length(s1);

    println!("The length of {s1} is: {s1_length}");
}

fn calculate_length(str: String) -> (usize, String) {
    let length = str.len();

    (length, str)
}
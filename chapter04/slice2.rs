fn main() {
    let my_string = String::from("I want to eat breakfast");

    let word = first_word(&my_string[0..1]);
    let word = first_word(&my_string[..]);

    let word = first_word(&my_string);

    let my_string_literal = "I want to eat breakfast";

    let word = first_word(&my_string_literal[0..1]);
    let word = first_word(&my_string_literal[..]);
    let word = first_word(&my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
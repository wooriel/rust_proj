fn main() {
    let cheering = String::from("You can do it");

    let first = first_word(&cheering);

    println!("{first}, {cheering}");

    let second = second_word(&cheering);

    println!("{second}");

    cheering.clear();

    println!("the first word is: {}", cheering);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    let mut start_index = s.len();
    let mut entered: bool = false;
    // let mut end_index = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if entered == true && item == b' ' {
            // end_index = i;
            return &s[start_index..i]
        }
        if item == b' ' {
            if i < s.len() - 1 {
                start_index = i+1;
                entered = true;
            } else {
                return &s[..]
            }
        }
    }

    &s[..]
}
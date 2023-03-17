fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // string is removed and the index = word is not valid
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // array같은곳 안에 하나씩 h e l l o  w o r l d 들어있는걸 기대, byte로 읽음

    for (i, &item) in bytes.iter().enumerate() { // i = index, &item = reference to 
        if item == b' ' { // when byte representing the space is found
            return i // return the position
        }
    }

    s.len() // if no space, just return the length of the string
}

// fn second_word(s: &String) -> (usize, usize) {

// }
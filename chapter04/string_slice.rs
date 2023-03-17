fn main() {
    let s = String::from("Happy birthday");

    let happy = &s[0..5];
    let happy2 = &s[..5];
    let birthday = &s[6..14];
    let birthday2 = &s[6..];

    println!("{happy} {happy2} {birthday} {birthday2}");
}
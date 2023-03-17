fn main() {
    let number = 3;
    let number = 7; // shadowing

    if number < 5 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }
}
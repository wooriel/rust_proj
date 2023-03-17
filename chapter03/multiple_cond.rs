fn main() {
    let visited = 90;

    if visited <= 50 {
        println!("You're a newbie");
    } else if visited > 50 {
        println!("You're a fan");
    } else if visited > 100 {
        println!("You're a 3-month fan");
    } else if visited > 365 {
        println!("It's already 1 years from now!");
    } else {
        println!("You've become a fan for more than one years")
    }
}
fn main() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    let number2 = if !condition {"five"} else {"six"};

    println!("The value of number2 is: {number2}");
}
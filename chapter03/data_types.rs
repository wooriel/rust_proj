fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    
    // let guess = "42".parse().expect("Not a number");
    // let guess: /* Type */ = "42".parse().expect("Not a number");

    println!("The guessed number is: {guess}");

    // example of data types
    let thousand: i32 = 1_000;
    let unknown: i32 = 0b11010;
    let byte: u8 = b'A';

    println!("{thousand}");
    println!("{} {}", unknown, byte);

    // example of overflow
    // let pos_num: u8 = 256;
    // the literal `256` does not fit into the type `u8` whose range is `0..=255`
    // println!("{}", pos_num);

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;
}
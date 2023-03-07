use std::io;
use std::str::Split;
use std::cmp::Ordering;


fn main() {
    println!("Enter sequence of numbers");
    
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line");

    let numbers: Vec<&str> = numbers.split(' ').collect();

    let mut str_iter = numbers.into_iter();

    // if let Some(strnum) = iter.next() {
    //     println!("{}", strnum);

    //     for num in iter {
    //         println!("{}", num);
    //     }
    // }

    for strnum in str_iter {
        println!("{}", strnum)
    }
}

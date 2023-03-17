fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    let s3 = String::from("Youth");
    let s4 = s3.clone(); // deep-copying the data, expensive

    println!("Bye {}", s3);
    println!("Bye {}", s4);
}
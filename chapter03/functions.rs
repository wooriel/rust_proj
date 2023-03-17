
fn main() {
    println!("Hello World!");

    another_function(10);
    print_labeled_measurement(3, 'h');

    let y = 6;
    // let x = (let y = 6);
}

fn another_function(x: i32) {
    // println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
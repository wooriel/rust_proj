#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(50 * scale),
        height: 30,
    };

    // println!("rect1 is {:?}", rect1);
    dbg!(&rect1);
}
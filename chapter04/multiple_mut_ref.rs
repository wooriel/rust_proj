fn main() {
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    {
        let r1 = &mut s;

        println!("{r1}");
    }

    let r2 = &mut s;

    println!("{r2}");
}
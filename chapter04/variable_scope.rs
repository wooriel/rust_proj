fn main() {
    let s = "hello";

    {
        let s2 = "Korea";

        println!("{s} {s2}");
    }
    // s2 no longer valid

    // println!("{s} {s2}")
}
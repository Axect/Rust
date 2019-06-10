fn main() {
    let aliens: [&str;4] = ["hi", "hello", "wow", "cool"];

    for i in &aliens {
        println!("{}", *i);
    }
}

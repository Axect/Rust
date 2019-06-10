fn main() {
    let aliens = vec!["hi", "hello", "wow", "cool"];

    for a in &aliens {
        println!("{}", *a);
    }
}

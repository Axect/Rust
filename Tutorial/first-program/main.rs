fn main() {
    // these two variables have identical types
    let target_inferred = "inferred world";
    let target: &'static str = "non-inferred world";
    println!("Hi there, {}", target_inferred);
    println!("Hi there, {}", target)
}

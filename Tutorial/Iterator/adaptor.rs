fn main() {
    let arr = [10, 14, 5, 76, 84];
    let mut iter = arr.iter();

    while let Some(elm) = iter.next() {
        println!("{}", elm);
    }

    for elm in &arr {
        println!("{}", elm);
    }
}

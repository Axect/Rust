use std::cmp::Ordering;
use peroxide::fuga::*;

fn main() {
    let a = seq(0, 10, 1);
    
    let b = gen_range(&a);

    let x = 11f64;

    let bs = b.binary_search_by(|r| {
        if r.contains(&x) {
            Ordering::Equal
        } else if r.start > x {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    println!("{:?}", bs);
}

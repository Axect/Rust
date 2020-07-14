#[macro_use]
extern crate peroxide;
extern crate high_ad;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let a = c!(2, 1, 0);
    let b = c!(4, 4, 2);
    div(&a, &b).print();
}

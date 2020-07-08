#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let a = c!(3, 1, 0);
    exp(&a).print();
    exp(&powi(&a, 2)).print();
}

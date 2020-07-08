#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use high_ad::*;

fn main() {
    let a = c!(3, 1, 0);
    exp(&a).print();
    exp(&powi(&a, 2)).print();
}

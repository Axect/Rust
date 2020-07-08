#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let x = c!(1, 1, 0, 0, 0, 0, 0);
    powi(&x, 100).print();
}

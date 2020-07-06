#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use high_ad::*;

fn main() {
    let x = c!(1, 1, 0, 0, 0, 0, 0);
    powi(&x, 100).print();
}

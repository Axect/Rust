#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let x = c!(1, 1, 0, 0, 0, 0, 0);
    powf(&x, 100f64).print();
}

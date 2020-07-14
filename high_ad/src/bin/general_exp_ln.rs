#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let x = c!(4, 4, 2);
    powd(2f64, &x).print();
    ln(&x).print();
}

#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let x = c!(3, 1, 0);
    powd(2f64, &x).print();
    log(&x, 2f64).print();
}

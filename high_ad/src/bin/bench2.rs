#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;
use high_ad::*;

fn main() {
    let x = c!(1, 1, 0, 0, 0, 0, 0);
    let mut y = x.clone();
    for _i in 0 .. 29 {
        y = mul(&y, &x);
    }
    y.print();
}

#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let x = c!(3, 1, 0);
    let (s, c) = sin_cos(&x);
    s.print();
    c.print();
    let t = tan(&x);
    t.print();
}

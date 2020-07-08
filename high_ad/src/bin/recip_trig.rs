#[macro_use]
extern crate peroxide;
use high_ad::*;
use peroxide::fuga::*;

fn main() {
    let x = c!(3, 1, 0);
    let (c, s) = csc_sec(&x);
    s.print();
    c.print();
    let t = cot(&x);
    t.print();
}

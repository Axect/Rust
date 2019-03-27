extern crate peroxide;
use peroxide::*;

pub fn main() {
    let n = 50usize;

    let mut s: f64 = 0f64;

    for i in 0 .. 10 {
        s += rand(n, n).det();
    }

    s.print();
}

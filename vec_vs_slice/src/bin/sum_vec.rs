extern crate peroxide;
use peroxide::*;

fn main() {
    let mut v = vec![0f64; 100_0000];
    for (i, x) in v.iter_mut().enumerate() {
        *x += i as f64;
    }

    v.mean().print();
}

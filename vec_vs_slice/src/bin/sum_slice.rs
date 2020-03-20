extern crate peroxide;
use peroxide::*;

fn main() {
    let mut a = [0f64; 100_0000];
    for (i, x) in a.iter_mut().enumerate() {
        *x += i as f64;
    }

    let b = &a[..];

    mean(b).print();
}

fn mean(v: &[f64]) -> f64 {
    let l_f64 = v.len() as f64;
    let mut s = 0f64;
    for x in v.iter() {
        s += x;
    }
    s / l_f64
}

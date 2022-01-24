#![feature(portable_simd)]
use std::simd::f64x4;

fn main() {
    let x: Vec<f64> = (1 .. 100_0001).map(|t| t as f64).collect();

    let mut s = f64x4::splat(0.);
    for i in (0 .. x.len()).step_by(4) {
        s += f64x4::from_slice(&x[i..]);
    }
    println!("{}", s.as_array().iter().sum::<f64>());
}

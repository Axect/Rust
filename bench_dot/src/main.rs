extern crate peroxide;
use peroxide::fuga::*;
use std::env::args;

fn main() {
    //let args: Vec<String> = args().collect();
    //let num: usize = args[1].parse().unwrap();
    //if num == 1 {
    //    bench_1().print();
    //} else if num == 2 {
    //    bench_2().print();
    //} else {
    //    println!("err");
    //}
    // x^3
    let x = vec![8f64, 12f64, 12f64, 6f64];
    let y = vec![2f64, 1f64];

    bench_1(&x, &y).print();
}

fn bench_1(x: &[f64], y:&[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    for t in 0 .. x.len() {
        z[t] = if t < y.len() {
            x.iter().take(t+1).zip(y.iter().take(t+1).rev()).fold(0f64, |x, (x1, y1)| x + x1 * y1)
        } else {
            x.iter().take(t+1).rev().zip(y.iter()).fold(0f64, |x, (x1, x2)| x + x1 * x2)
        };
    }
    z
}

fn bench_2() -> Vec<f64> {
    let x = seq(1, 2000, 1);
    let y = seq(1, 1950, 1);
    let a = Matrix::from_index(|i, j| {
        if i >= j {
            x[i-j]
        } else {
            0f64
        }
    }, (2000, 2000));
    let mut b = y.clone();
    b.extend(vec![0f64; 50].iter());

    a * b
}

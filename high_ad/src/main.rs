extern crate peroxide;
use peroxide::fuga::*;
//use std::env::args;

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
    let x1 = vec![3f64, 1f64];
    x1.print();
    let x2 = mul(&x1, &x1);
    x2.print();
    let x3 = mul(&x2, &x1);
    x3.print();
    let x4 = mul(&x3, &x1);
    x4.print();
    let x5 = mul(&x3, &x2);
    x5.print();
    let x6 = mul(&x4, &x1);
    x6.print();

    let y1 = div(&x1, &x2);
    y1.print();
    let y2 = div(&x1, &x3);
    y2.print();
}

fn mul(x: &[f64], y: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len() + y.len() - 1];
    for t in 0..z.len() {
        z[t] = if t < y.len() {
            x.iter()
                .take(t + 1)
                .zip(y.iter().take(t + 1).rev())
                .enumerate()
                .fold(0f64, |s, (i, (x1, y1))| s + (C(t,i) as f64) * x1 * y1)
        } else if t < x.len() {
            x.iter()
                .take(t + 1)
                .rev()
                .zip(y.iter())
                .enumerate()
                .fold(0f64, |s, (i, (x1, x2))| s + (C(t,i) as f64) * x1 * x2)
        } else {
            x.iter()
                .enumerate()
                .skip(t - y.len() + 1)
                .zip(y.iter().rev())
                .fold(0f64, |s, ((i, x1), y1)| s + (C(t, i) as f64) * x1 * y1)
        };
    }
    z
}

fn div(x: &[f64], y: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    z[0] = x[0] / y[0];
    let y0 = 1f64 / y[0];
    for i in 1 .. z.len() {
        let mut s = 0f64;
        for (j, (y1, z1)) in y[1..i+1].iter().zip(z[0..i].iter().rev()).enumerate() {
            s += (C(i, j+1) as f64) * y1 * z1;
            s.print();
        }
        z[i] = y0 * (x[i] - s);
    }
    z
}

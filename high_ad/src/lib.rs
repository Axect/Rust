extern crate peroxide;
use peroxide::fuga::*;
//use std::env::args;

pub fn mul(x: &[f64], y: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    for t in 0..z.len() {
        z[t] = if t < y.len() {
            x.iter()
                .take(t + 1)
                .zip(y.iter().take(t + 1).rev())
                .enumerate()
                .fold(0f64, |s, (i, (x1, y1))| s + (C(t, i) as f64) * x1 * y1)
        } else if t < x.len() {
            x.iter()
                .take(t + 1)
                .rev()
                .zip(y.iter())
                .enumerate()
                .fold(0f64, |s, (i, (x1, x2))| s + (C(t, i) as f64) * x1 * x2)
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

pub fn div(x: &[f64], y: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    z[0] = x[0] / y[0];
    let y0 = 1f64 / y[0];
    for i in 1..z.len() {
        let mut s = 0f64;
        for (j, (y1, z1)) in y[1..i + 1].iter().zip(z[0..i].iter().rev()).enumerate() {
            s += (C(i, j + 1) as f64) * y1 * z1;
        }
        z[i] = y0 * (x[i] - s);
    }
    z
}

pub fn ln(x: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    z[0] = x[0].ln();
    let x0 = 1f64 / x[0];
    for i in 1..z.len() {
        let mut s = 0f64;
        for (j, (x1, z1)) in x[1..i].iter().zip(z[1..i].iter().rev()).enumerate() {
            s += (C(i - 1, j + 1) as f64) * x1 * z1;
        }
        z[i] = x0 * (x[i] - s);
    }
    z
}

pub fn powf(x: &[f64], f: f64) -> Vec<f64> {
    let ln_x = ln(x);
    let mut z = vec![0f64; x.len()];
    z[0] = x[0].powf(f);
    for i in 1..z.len() {
        let mut s = 0f64;
        for (j, (z1, ln_x1)) in z[1..i].iter().zip(ln_x[1..i].iter().rev()).enumerate() {
            s += (C(i - 1, j + 1) as f64) * z1 * ln_x1;
        }
        z[i] = f * (z[0] * ln_x[i] + s);
    }
    z
}

pub fn powi(x: &[f64], n: usize) -> Vec<f64> {
    let mut z = Vec::from(x);
    for _i in 1..n {
        z = mul(&z, x);
    }
    z
}

pub fn exp(x: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    z[0] = x[0].exp();
    for i in 1..z.len() {
        z[i] = z[0..i]
            .iter()
            .zip(x[1..i + 1].iter().rev())
            .enumerate()
            .fold(0f64, |x, (k, (z1, x1))| x + (C(i - 1, k) as f64) * x1 * z1);
    }
    z
}

pub fn sin_cos(x: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut u = vec![0f64; x.len()];
    let mut v = vec![0f64; x.len()];
    u[0] = x[0].sin();
    v[0] = x[0].cos();

    for i in 1..x.len() {
        u[i] = v[0..i]
            .iter()
            .zip(x[1..i + 1].iter().rev())
            .enumerate()
            .fold(0f64, |x, (k, (v1, x1))| x + (C(i - 1, k) as f64) * x1 * v1);
        v[i] = -u[0..i]
            .iter()
            .zip(x[1..i + 1].iter().rev())
            .enumerate()
            .fold(0f64, |x, (k, (u1, x1))| x + (C(i - 1, k) as f64) * x1 * u1);
    }
    (u, v)
}

pub fn tan(x: &[f64]) -> Vec<f64> {
    let (s, c) = sin_cos(x);
    div(&s, &c)
}

pub fn recip(x: &[f64]) -> Vec<f64> {
    let mut z = vec![0f64; x.len()];
    z[0] = 1f64 / x[0];
    for i in 1..z.len() {
        let s = z[0..i]
            .iter()
            .zip(x[1..i + 1].iter().rev())
            .enumerate()
            .fold(0f64, |x, (k, (z1, x1))| x + (C(i, k) as f64) * z1 * x1);
        z[i] = -s * z[0];
    }
    z
}

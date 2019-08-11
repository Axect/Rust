#[macro_use]
extern crate ndarray;
use ndarray::prelude::*;

fn main() {
    let a: Array2<f64> = Array::zeros((1000, 1000));
    println!("{}", a.dot(&a));
}

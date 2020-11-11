extern crate ndarray;
extern crate ndarray_linalg;

use ndarray::*;
use ndarray_linalg::*;

fn main() {
    let a: Array2<f64> = random((3, 3));
    println!("{}", a);
    let x = solve().unwrap();
    println!("{}", x);
    factorize().unwrap();
}

fn solve() -> Result<Array1<f64>, error::LinalgError> {
    let a: Array2<f64> = random((3, 3));
    let b: Array1<f64> = random(3);
    let x = a.solve(&b)?;
    Ok(x)
}

fn factorize() -> Result<(), error::LinalgError> {
    let a: Array2<f64> = random((3,3));
    let f = a.factorize_into()?;
    for _ in 0 .. 10 {
        let b: Array1<f64> = random(3);
        let _x = f.solve_into(b)?;
    }
    Ok(())
}

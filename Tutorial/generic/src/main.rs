use std::convert;
fn main() {
    let p = Matrix::new(1,2,vec![1,2,3,4]);
    println!("{:?}", p);
}

#[derive(Debug)]
pub struct Matrix {
    i: u64,
    j: u64,
    data: Vec<f64>,
}

trait Array2D<T: convert::Into<f64>> {
    fn new(x:u64, y:u64, v: Vec<T>) -> Matrix;
}

impl<T> Array2D<T> for Matrix where T: convert::Into<f64> {
    fn new(x: u64, y: u64, v: Vec<T>) -> Matrix {
        Matrix {
            i: x,
            j: y,
            data: v.into_iter().map(|x| x.into()).collect(),
        }
    }
}

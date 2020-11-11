extern crate ndarray;
extern crate ndarray_rand;
use ndarray::Array;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

fn main() {
    let a = Array::random((1000, 1000), Uniform::new(0., 1.));
    let b = Array::random((1000, 1000), Uniform::new(0., 1.));
    println!("{}", a.dot(&b)[[99, 99]]);
}

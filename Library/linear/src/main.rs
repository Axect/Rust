#[macro_use(array)]
extern crate ndarray;

use ndarray::Array;

fn main() {
    let a1 = array![1, 2, 3, 4];
    let a2 = array![[1, 2], [3, 4]];
    println!("{}", a1);
    println!("{}", a2.dot(&a2));

    let mut a = Array::zeros((2, 2));
    for mut row in a.genrows_mut() {
        row.fill(1.);
    }
    println!("{}", a);
}

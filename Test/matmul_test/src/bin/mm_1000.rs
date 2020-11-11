extern crate peroxide;
extern crate matrixmultiply;
use matrixmultiply::dgemm;
use peroxide::*;

fn main() {
    let a = zeros_shape(1000, 1000, Col);
    let b = zeros_shape(1000, 1000, Col);
    let mut c = zeros_shape(1000, 1000, Col);
    let m = a.row;
    let k = a.col;
    let n = a.col;
    unsafe {
        dgemm(
            m,
            k,
            n,
            1f64,
            a.ptr(),
            1,
            a.row as isize,
            b.ptr(),
            1,
            b.row as isize,
            0f64,
            c.mut_ptr(),
            1,
            c.row as isize,
        );
    }
    c[(999, 999)].print();
}

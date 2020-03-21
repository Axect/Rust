extern crate peroxide;
extern crate matrixmultiply;
use matrixmultiply::dgemm;
use peroxide::*;

fn main() {
    let a = matrix(c!(1,2,3,4), 2, 2, Col);
    let b = matrix(c!(5,6,7,8), 2, 2, Col);
    a.print();
    b.print();
    let mut c = zeros_shape(2, 2, Col);
    let m = a.row;
    let k = a.col;
    let n = b.col;
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

    c.print();
}

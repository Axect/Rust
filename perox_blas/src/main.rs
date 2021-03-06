extern crate peroxide;
extern crate blas;
use peroxide::*;
use blas::*;

fn main() {
    let a = zeros(3, 2);
    let b = zeros(2, 2);
    Matrix::blas_mul(&a, &b).print();
}

trait PeroxBLAS {
    fn blas_mul(m1: &Self, m2: &Self) -> Self;
}

impl PeroxBLAS for Matrix {
    // m1: m x k
    // m2: k x n
    // result: m x n
    fn blas_mul(m1: &Self, m2: &Self) -> Self {
        let m = m1.row;
        let k = m1.col;
        assert_eq!(k, m2.row);
        let n = m2.col;

        let m_i32 = m as i32;
        let n_i32 = n as i32;
        let k_i32 = k as i32;

        let a = &m1.data;
        let b = &m2.data;
        let mut c = vec![0f64; m * n];

        unsafe {
            dgemm(b'N', b'N', m_i32, n_i32, k_i32, 1f64, a, m_i32, b, k_i32, 0f64, &mut c, m_i32);
        }

        matrix(c, m, n, Row)
    }
}

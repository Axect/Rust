extern crate peroxide;
extern crate packed_simd;
use peroxide::*;
use packed_simd::f64x8;

fn main() {
    let a = zeros(1000, 1000);
    let b = zeros(1000, 1000);

    let mut c = zeros(1000, 1000);
    for i in 0 .. 1000 {
        for j in 0 .. 1000 {
            c[(i, j)] = dot_prod(&(a.row(i)[..]), &(a.col(j)[..]));
        }
    }
    c[(999, 999)].print();
}

fn dot_prod(a: &[f64], b: &[f64]) -> f64 {
    assert!(a.len() % 8 == 0);

    a.chunks_exact(8)
        .map(f64x8::from_slice_unaligned)
        .zip(b.chunks_exact(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum()
}

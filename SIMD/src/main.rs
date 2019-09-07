use packed_simd::f64x4;

fn main() {
    let a = vec![0f64; 10000_0000];
    let b = vec![0f64; 10000_0000];

    println!("{}", dot_prod(&a, &b));
}

fn dot_prod(a: &[f64], b: &[f64]) -> f64 {
    assert!(a.len() % 4 == 0);

    a.chunks_exact(4)
        .map(f64x4::from_slice_unaligned)
        .zip(b.chunks_exact(4).map(f64x4::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x4>()
        .sum()
}

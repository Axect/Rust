use packed_simd::f32x16;

fn main() {
    let a = vec![0f32; 10000_0000];
    let b = vec![0f32; 10000_0000];

    println!("{}", dot_prod(&a, &b));
}

fn dot_prod(a: &[f32], b: &[f32]) -> f32 {
    assert!(a.len() % 16 == 0);

    a.chunks_exact(16)
        .map(f32x16::from_slice_unaligned)
        .zip(b.chunks_exact(16).map(f32x16::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f32x16>()
        .sum()
}

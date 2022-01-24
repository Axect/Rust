#![feature(portable_simd)]
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::simd::{f64x4, f64x8};

fn for_sum(v: &[f64]) -> f64 {
    let mut s = 0f64;
    for t in v {
        s += *t;
    }
    s
}

fn f64x4_sum(v: &[f64]) -> f64 {
    let mut sum = f64x4::splat(0.);
    for i in (0 .. v.len()).step_by(4) {
        sum += f64x4::from_slice(&v[i..]);
    }
    sum.as_array().iter().sum()
}

fn f64x8_sum(v: &[f64]) -> f64 {
    let mut sum = f64x8::splat(0.);
    for i in (0 .. v.len()).step_by(8) {
        sum += f64x8::from_slice(&v[i..]);
    }
    sum.as_array().iter().sum()
}

fn bench_sums(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    let middle_range = (1 .. 11).map(|t| 100_0000 * t);
    
    for i in middle_range {
        let x: Vec<f64> = (0 .. i).map(|t| t as f64).collect();
        group.bench_with_input(BenchmarkId::new("for_sum", i), black_box(&x), |b, x| b.iter(|| for_sum(&x)));
        group.bench_with_input(BenchmarkId::new("f64x4_sum", i), black_box(&x), |b, x| b.iter(|| f64x4_sum(&x)));
        group.bench_with_input(BenchmarkId::new("f64x8_sum", i), black_box(&x), |b, x| b.iter(|| f64x8_sum(&x)));
    }
    group.finish();
}

criterion_group!(benches, bench_sums);
criterion_main!(benches);

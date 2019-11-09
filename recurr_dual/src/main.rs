#![feature(box_patterns)]

use std::ops::{Add,Mul,Sub,Div};
use Dual2::*;

fn main() {
    let a = D(1f64, Box::new(D(3f64, Box::new(T(6f64)))));
    let b = D(1f64, Box::new(D(2f64, Box::new(T(0f64)))));
    println!("{:?}", a + b);
}

#[derive(Debug)]
enum Dual2 {
    D(f64, Box<Dual2>),
    T(f64),
}

impl Add for Dual2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (D(x1, box dx1), D(x2, box dx2)) => {
                D(x1 + x2, Box::new(dx1 + dx2))
            },
            (D(x1, dx1), T(x2)) => {
                D(x1 + x2, dx1)
            },
            (T(x1), D(x2, dx2)) => {
                D(x1 + x2, dx2)
            },
            (T(x1), T(x2)) => {
                T(x1 + x2)
            }
        }
    }
}

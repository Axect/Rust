#![feature(box_patterns)]

use std::ops::{Add, Mul, Sub, Div, Neg};
use Dual2::*;

fn main() {
    let a = D(1f64, Box::new(D(3f64, Box::new(D(6f64, Box::new(T(6f64)))))));
    let b = D(1f64, Box::new(D(2f64, Box::new(T(2f64)))));
    println!("{:?}", a.clone() + b.clone());
    println!("{:?}", a * b);
}

#[derive(Debug, Clone)]
enum Dual2 {
    D(f64, Box<Dual2>),
    T(f64),
}

impl Neg for Dual2 {
    type Output = Dual2;

    fn neg(self) -> Self::Output {
        match self {
            D(x, box dx) => {
                D(-x, Box::new(-dx))
            }
            T(x) => {
                T(-x)
            }
        }
    }
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

impl Sub for Dual2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Dual2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (D(x1, box dx1), D(x2, box dx2)) => {
                D(x1 * x2, Box::new(T(x1) * dx2 + dx1 * T(x2)))
            }
            (D(x1, box dx1), T(x2)) => {
                D(x1 * x2, Box::new(dx1 * T(x2)))
            }
            (T(x1), D(x2, box dx2)) => {
                D(x1 * x2, Box::new(T(x1) * dx2))
            }
            (T(x1), T(x2)) => {
                T(x1 * x2)
            }
        }
    }
}
use std::ops::{Neg, Add, Sub, Mul};

fn main() {
    let x3 = Dual::new(vec![1., 3., 6., 6.]);
    let x2 = Dual::new(vec![1., 2., 2.]);

    println!("{:?}", x3.clone() * x2.clone());
    println!("{:?}", x3.powi(3));
}

#[derive(Debug, Clone)]
struct Dual {
    data: Vec<f64>
}

impl Dual {
    fn new(v: Vec<f64>) -> Self {
        Dual {
            data: v
        }
    }
}

impl Neg for Dual {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Dual::new(self.data.into_iter().map(|x| -x).collect())
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let l1 = self.data.len();
        let l2 = rhs.data.len();
        if l1 >= l2 {
            let mut v = vec![0f64; l1];
            for i in 0 .. l2 {
                v[i] = self.data[i] + rhs.data[i];
            }
            for i in l2 .. l1 {
                v[i] = self.data[i];
            }
            Dual::new(v)
        } else {
            let mut v = vec![0f64; l2];
            for i in 0 .. l1 {
                v[i] = self.data[i] + rhs.data[i];
            }
            for i in l1 .. l2 {
                v[i] = rhs.data[i];
            }
            Dual::new(v)
        }
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[allow(non_snake_case)]
fn P(n: usize, r: usize) -> usize {
    if r == 0 {
        1
    } else {
        let mut p = 1usize;
        for i in (n-r+1) .. n+1 {
            p *= i;
        }
        p
    }
}

fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        let mut s = 1usize;
        for i in 2 .. n+1 {
            s *= n;
        }
        s
    }
}

#[allow(non_snake_case)]
fn C(n: usize, r: usize) -> usize {
    if r > n / 2 {
        return C(n, n-r);
    }

    P(n, r) / factorial(r)
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let l1 = self.data.len();
        let l2 = rhs.data.len();

        if l1 > l2 {
            let mut v = rhs.data.clone();
            for i in 0 .. l1 - l2 {
                v.push(0f64);
            }
            return self.mul(Dual::new(v));
        } else if l1 < l2 {
            let mut v = self.data.clone();
            for i in 0 .. l2 - l1 {
                v.push(0f64);
            }
            return Dual::new(v).mul(rhs);
        }

        println!("{:?}", self);
        println!("{:?}", rhs);

        let mut v = vec![0f64; l1];

        v[0] = self.data[0] * rhs.data[0];

        for k in 1 .. v.len() {
            v[k] = {
                let mut s = 0f64;
                for i in 0 .. k+1 {
                    s += (C(k, i) as f64) * self.data[i] * rhs.data[k-i];
                }
                s
            }
        }

        Dual::new(v)
    }
}

trait PowOps {
    fn powi(self, n: i32) -> Self;
}

impl PowOps for Dual {
    fn powi(self, n: i32) -> Self {
        Dual::new(powi(self.data, n))
    }
}

fn mul(lhs: Vec<f64>, rhs: Vec<f64>) -> Vec<f64> {
    let l1 = lhs.len();
    let l2 = rhs.len();

    if l1 > l2 {
        let mut v = rhs.clone();
        for i in 0 .. l1 - l2 {
            v.push(0f64);
        }
        return mul(lhs, v);
    } else if l1 < l2 {
        let mut v = lhs.clone();
        for i in 0 .. l2 - l1 {
            v.push(0f64);
        }
        return mul(v, rhs);
    }

    let mut v = vec![0f64; l1];

    v[0] = lhs[0] * rhs[0];

    for k in 1 .. v.len() {
        v[k] = {
            let mut s = 0f64;
            for i in 0 .. k+1 {
                s += (C(k, i) as f64) * lhs[i] * rhs[k-i];
            }
            s
        }
    }
    v
}

fn powi(v: Vec<f64>, n: i32) -> Vec<f64> {
    if n <= 1 {
        return v;
    }

    let x0 = v[0];
    let x1: Vec<f64> = v.clone().into_iter().skip(1).collect();
    let mut result = vec![x0];
    let mut tail = mul(powi(v, n-1), x1).into_iter().map(|x| x * (n as f64));
    result.extend(tail);
    result
}

use std::ops::{Add};

fn main() {
    let x = Test { val : 0.1 };
    let y = Test { val : 0.2 };

    println!("{:?}", f(x,y));
}

fn f<T: Real>(x: T, y: T) -> <T as Add>::Output {
    x.sin() + y
}

trait TrigOps: Add + Sized {
    fn sin(&self) -> Self;
}

trait Real: TrigOps + Add + Sized {
}

#[derive(Debug, Clone)]
struct Test {
    val: f64,
}

impl TrigOps for Test {
    fn sin(&self) -> Self {
        Test { val: self.val.sin() }
    }
}

impl Add<Test> for Test {
    type Output = Self;
    fn add(self, other: Test) -> Self::Output {
        Test { val: self.val + other.val }
    }
}

impl Real for Test {
}

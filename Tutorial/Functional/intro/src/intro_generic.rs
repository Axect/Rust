#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

pub fn foo<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}

pub fn bar<F, T>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(x)
}

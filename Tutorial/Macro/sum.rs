pub fn main() {
    println!("{}", sum![1,2,3,4,5]);
}

#[macro_export]
macro_rules! sum {
    () => (0);
    ($head:expr $(, $tail:expr)*) => ($head + sum!($($tail),*));
}

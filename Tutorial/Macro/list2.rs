pub fn main() {
    let a = list!(1;4);
    println!("{:?}", a);
}

#[macro_export]
macro_rules! list {
    ( $start:expr;$end:expr ) => {
        {
            let mut v = Vec::new();
            for i in $start .. ($end + 1) {
                v.push(i);
            }
            v
        }
    };
}
